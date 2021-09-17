import os from 'os'
import Mali from 'mali'
import { Logger, RubickBaseSettings, DeviceEvent, Position, Color } from './types'
import newRustBackend, { RustBackendAPI } from './worker'
import { loadPackageDefinition } from '@grpc/grpc-js'
import { fromJSON } from '@grpc/proto-loader'
import { INamespace } from 'protobufjs'
import fs from 'fs-extra'
import { eventEqual, getRandomNum, rgbToHex } from './utils'
import { defaultLogger } from './logger'
import { deviceEventEmitter, EventCallback, EventChannelMap } from './event'
import { newImageFromBase64, Image } from './image'

export class RubickBase {
	private server!: Mali<any>
	private worker!: RustBackendAPI
	private port: string
	private tmpdir: string
	private eventChannels: EventChannelMap
	private cursorPosition: Position = { x: 1, y: 1 }
	private started: boolean = false
	logger: Logger
	constructor(settings: RubickBaseSettings) {
		const { port, logger, tmpdir, ioEventCallback } = settings
		// settings
		// if no port, gen a port from 50000-60000
		this.port = (port || getRandomNum(50000, 60000)).toString()
		this.logger = logger || defaultLogger
		this.tmpdir = tmpdir || os.tmpdir()
		this.eventChannels = new EventChannelMap(this.logger)

		// start buitin service
		this.initBuiltinService(ioEventCallback || ((_) => {}))
	}

	// ******************************* life cycle *******************************
	async start() {
		this.worker = await newRustBackend()
		await this.server.start(`127.0.0.1:${this.port}`)
		await this.afterStart()
		this.started = true
	}

	async close() {
		deviceEventEmitter.removeAllListeners()
		await this.server.close()
		this.started = false
	}

	// start workers
	private async afterStart() {
		const log = (success: boolean, name: string) => {
			if (success) {
				this.logger.success(`Start ${name} worker`)
			} else {
				this.logger.error(`Start ${name} worker`)
			}
		}
		// start workers
		log(await this.worker?.ioioStart(this.port), 'ioio')
	}

	// registe builtin RPC services
	private async initBuiltinService(eventCallback: EventCallback) {
		this.server = new Mali(await this.loadProto(), 'Rubick')
		this.server.use('ioio', async (ctx: any) => {
			const event: DeviceEvent = ctx.request.req
			// mousemove info is still string here, need convert to Position object
			if (
				event.device === 'Mouse' &&
				event.action === 'Move' &&
				((<unknown>event.info) as string).startsWith('{')
			) {
				event.info = JSON.parse((<unknown>event.info) as string)
			}
			// post event to global event channel
			deviceEventEmitter.emit('deviceEvent', event)
			ctx.res = { ok: true }
		})

		// handle async event callback error
		deviceEventEmitter.on('error', (err) => {
			this.logger.error(err)
		})

		// global listen event
		deviceEventEmitter.on('deviceEvent', async (event) => {
			if (eventCallback) await eventCallback(event)
			if (event.device === 'Mouse' && event.action === 'Move') {
				this.cursorPosition = event.info
			}
		})
	}

	// ******************************* Utils *******************************
	private async loadProto(): Promise<string | object> {
		let proto: string | object = './proto/rubick.proto'
		try {
			const protoJSON = await import('./proto/rubick.proto')
			proto = loadPackageDefinition(fromJSON(protoJSON as INamespace))
			this.logger.info('You are in production mode, protoJSON loaded.')
		} catch {}
		return proto
	}

	// try rust-backend or log error
	private async tryBackend<T>(func: () => Promise<T>, errorReturn: () => T): Promise<T> {
		try {
			return await func()
		} catch (error) {
			this.logger.error(error)
			return errorReturn()
		}
	}

	// valid directory and file then try rust-backend
	private async validAndTryBackend<T>(
		func: () => Promise<T>,
		errorReturn: () => T,
		dic: string[] | string = [],
		file: string[] | string = [],
	): Promise<T> {
		if (typeof dic === 'string') {
			dic = [dic]
		}
		if (typeof file === 'string') {
			file = [file]
		}
		let v1 = dic.map((dic) => fs.existsSync(dic) && fs.lstatSync(dic).isDirectory())
		let v2 = file.map((path) => fs.existsSync(path) && fs.lstatSync(path).isFile())
		let v = [...v1, ...v2]
		if (!v.includes(false)) {
			return await this.tryBackend(func, errorReturn)
		} else {
			this.logger.error('No such directory!')
			return errorReturn()
		}
	}

	// ******************************* errors *******************************
	private colorError() {
		this.logger.error('Got an color error!')
		return {
			hex16: 'error',
			rgba: {
				r: -1,
				g: -1,
				b: -1,
				a: -1,
			},
		}
	}

	private imageError() {
		this.logger.error('Got an image error!')
		return newImageFromBase64('error')
	}

	// ******************************* expose APIs *******************************
	getAPI() {
		// valid start
		if (!this.started) {
			throw new Error('Rubick has not started! Start it first!')
		}

		// get cursor position
		const getCursorPosition = this.getCursorPosition

		// get pixel color at cursor position
		const getCursorPositionPixelColor = this.getCursorPositionPixelColor

		// screen capture
		const screenCapture = this.screenCapture

		// capture screen around position
		const screenCaptureAroundPosition = this.screenCaptureAroundPosition

		// lzma2 compress/decompress
		const compress = this.compress
		const decompress = this.decompress

		// event channel life cycle
		const setEventChannel = this.setEventChannel
		const allEventChannels = this.allEventChannels
		const hasEventChannel = this.hasEventChannel
		const delEventChannel = this.delEventChannel

		return {
			getCursorPosition,
			getCursorPositionPixelColor,
			screenCapture,
			screenCaptureAroundPosition,
			compress,
			decompress,
			setEventChannel,
			allEventChannels,
			hasEventChannel,
			delEventChannel,
		}
	}

	// ******************************* define APIs *******************************

	/** capture primary screen
	 *
	 * @returns {Promise<Image>} image object
	 */
	private async screenCapture(): Promise<Image> {
		return await this.tryBackend(async () => {
			const imgBase64 = await this.worker.captureToBase64()
			return newImageFromBase64(imgBase64)
		}, this.imageError)
	}

	/** capture screen return the area around position
	 *
	 * @param position center of the image
	 * @param width width
	 * @param height height
	 * @returns {Promise<Image>} image object
	 */
	private async screenCaptureAroundPosition(
		position: Position,
		width: number,
		height: number,
	): Promise<Image> {
		return await this.tryBackend(async () => {
			const imgBase64 = await this.worker.screenCaptureAroundPositionToBase64(
				position,
				width,
				height,
			)
			return newImageFromBase64(imgBase64)
		}, this.imageError)
	}

	/** get cursor position
	 *
	 * @returns {Position}
	 */
	private getCursorPosition(): Position {
		return this.cursorPosition
	}

	/** get pixel color at cursor position
	 *
	 * @return {Promise<Color>} color object
	 */
	private async getCursorPositionPixelColor(): Promise<Color> {
		return await this.tryBackend(async () => {
			const rgb = await this.worker.screenColorPicker(this.getCursorPosition())
			return {
				hex16: rgbToHex(rgb.r, rgb.g, rgb.b),
				rgba: {
					r: rgb.r,
					g: rgb.g,
					b: rgb.b,
					a: 255,
				},
			}
		}, this.colorError)
	}

	/** lzma compress
	 * @param fromPath from file
	 * @param toPath to file
	 */
	private async compress(fromPath: string, toPath: string) {
		return await this.validAndTryBackend(
			async () => await this.worker.compress(fromPath, toPath),
			() => undefined,
			[],
			[fromPath, toPath],
		)
	}

	/** lzma decompress
	 * @param fromPath from file
	 * @param toPath to file
	 */
	private async decompress(fromPath: string, toPath: string) {
		return await this.validAndTryBackend(
			async () => await this.worker.decompress(fromPath, toPath),
			() => undefined,
			[],
			[fromPath, toPath],
		)
	}

	/** set a channel and get register
	 *
	 * @param bindEvent
	 * @returns register - Decorator register; registerHook - Function hook register
	 */
	private setEventChannel(bindEvent: DeviceEvent) {
		// Decorator
		const register = (name: string) => {
			return (hook: EventCallback) => {
				const listener = async (deviceEvent: DeviceEvent) => {
					if (eventEqual(deviceEvent, bindEvent)) await hook(deviceEvent)
				}

				// register in map
				this.eventChannels.set(name, listener)

				// hook callback
				deviceEventEmitter.on('deviceEvent', listener)
			}
		}

		const registerHook = (name: string, hook: EventCallback) => {
			register(name)(hook)
		}

		// return register
		return { register, registerHook }
	}

	/** get all channels
	 *
	 * @returns {IterableIterator<string>} channels name
	 */
	private allEventChannels(): IterableIterator<string> {
		return this.eventChannels.keys()
	}

	/** has channel or not
	 *
	 * @param name channel name
	 * @returns {boolean}
	 */
	private hasEventChannel(name: string): boolean {
		return this.eventChannels.has(name)
	}

	/** del a channel
	 *
	 */
	private delEventChannel(name: string) {
		if (this.eventChannels.has(name)) {
			// remove listener
			const listener = this.eventChannels.get(name)
			if (listener) deviceEventEmitter.removeListener('deviceEvent', listener)
			// remove register item
			this.eventChannels.delete(name)
		} else {
			this.logger.error(`no such handler: ${name}`)
		}
	}
}

/** A new rubickbase service
 *
 * @param settings RubickBaseSettings
 * @returns {RubickBase}
 */
export const newRubickBase = (settings?: RubickBaseSettings): RubickBase => {
	return new RubickBase(settings || {})
}
