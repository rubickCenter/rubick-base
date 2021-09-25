import os from 'os'
import Mali from 'mali'
import { Logger, RubickBaseSettings, DeviceEvent, Position, Color, WorkerSettings } from './types'
import newRustBackend, { RustBackendAPI } from './backend'
import { loadPackageDefinition } from '@grpc/grpc-js'
import { fromJSON } from '@grpc/proto-loader'
import { INamespace } from 'protobufjs'
import fs from 'fs-extra'
import { eventEqual, rgbToHex } from './utils'
import { defaultLogger } from './logger'
import { deviceEventEmitter, EventCallback, EventChannelMap } from './event'
import { newImageFromBase64, Image } from './image'
import { RubickWorker } from './worker'

export class RubickBase {
	private server!: Mali<any>
	private rustBackend!: RustBackendAPI
	private port: string
	private tmpdir: string
	private eventChannels: EventChannelMap
	private cursorPosition: Position = { x: 1, y: 1 }
	private workerBoot: boolean
	private ioEventCallback: EventCallback
	private started: boolean = false
	logger: Logger
	constructor(settings: RubickBaseSettings) {
		const { port, logger, tmpdir, workerBoot, ioEventCallback } = settings
		// settings
		this.port = port?.toString() || '50068'
		this.logger = logger || defaultLogger
		this.tmpdir = tmpdir || os.tmpdir()
		this.eventChannels = new EventChannelMap(this.logger)
		this.workerBoot = workerBoot !== undefined ? workerBoot : true
		this.ioEventCallback = ioEventCallback || ((_) => {})
	}

	// ******************************* life cycle *******************************
	async start() {
		// start buitin service
		this.rustBackend = await newRustBackend()
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
			if (this.ioEventCallback) await this.ioEventCallback(event)
			if (event.device === 'Mouse' && event.action === 'Move') {
				this.cursorPosition = event.info
			}
		})

		await this.server.start(`127.0.0.1:${this.port}`)
		// bootstrap worker with rubickbase
		if (this.workerBoot) {
			await newRubickWorker({
				port: this.port,
				logger: this.logger,
			}).start()
		}
		this.started = true
	}

	async close() {
		deviceEventEmitter.removeAllListeners()
		this.started = false
		await this.server.close()
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
		this.logger.error('Got an api color error!')
		return undefined
	}

	private imageError() {
		this.logger.error('Got an api image error!')
		return undefined
	}

	private lzmaError() {
		this.logger.error('Got an api lzma error!')
		return undefined
	}

	private appSearchError() {
		this.logger.error('Got an api app search error!')
		return undefined
	}

	// ******************************* expose APIs *******************************
	getAPI() {
		if (!this.started) {
			this.start()
		}

		/** get cursor position
		 *
		 * @returns {Position}
		 */
		const getCursorPosition = (): Position => this.cursorPosition

		/** get pixel color at cursor position
		 *
		 * @return {Promise<Color | undefined>} color object
		 */
		const getCursorPositionPixelColor = async (): Promise<Color | undefined> =>
			await this.tryBackend(async () => {
				const rgb = await this.rustBackend.screenColorPicker(getCursorPosition())
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

		/** capture primary screen
		 *
		 * @returns {Promise<Image | undefined>} image object
		 */
		const screenCapture = async (): Promise<Image | undefined> =>
			await this.tryBackend(async () => {
				const imgBase64 = await this.rustBackend.captureToBase64()
				return newImageFromBase64(imgBase64)
			}, this.imageError)

		/** capture screen return the area around position
		 *
		 * @param position center of the image
		 * @param width width
		 * @param height height
		 * @returns {Promise<Image | undefined>} image object
		 */
		const screenCaptureAroundPosition = async (
			position: Position,
			width: number,
			height: number,
		): Promise<Image | undefined> =>
			await this.tryBackend(async () => {
				const imgBase64 = await this.rustBackend.screenCaptureAroundPositionToBase64(
					position,
					width,
					height,
				)
				return newImageFromBase64(imgBase64)
			}, this.imageError)

		/** lzma compress
		 * @param fromPath from file
		 * @param toPath to file
		 */
		const compress = async (fromPath: string, toPath: string) =>
			await this.validAndTryBackend(
				async () => await this.rustBackend.compress(fromPath, toPath),
				this.lzmaError,
				[],
				[fromPath, toPath],
			)

		/** lzma decompress
		 * @param fromPath from file
		 * @param toPath to file
		 */
		const decompress = async (fromPath: string, toPath: string) =>
			await this.validAndTryBackend(
				async () => await this.rustBackend.decompress(fromPath, toPath),
				this.lzmaError,
				[],
				[fromPath, toPath],
			)

		/** set a channel and get register
		 *
		 * @param bindEvent
		 * @returns register - Decorator register; registerHook - Function hook register
		 */
		const setEventChannel = (bindEvent: DeviceEvent) => {
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
		const allEventChannels = (): IterableIterator<string> => {
			return this.eventChannels.keys()
		}

		/** has channel or not
		 *
		 * @param name channel name
		 * @returns {boolean}
		 */
		const hasEventChannel = (name: string): boolean => {
			return this.eventChannels.has(name)
		}

		/** del a channel
		 *
		 */
		const delEventChannel = (name: string) => {
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

		/** get installed app or app detail info
		 *
		 * @param getDetailInfo get app detail info rather than app entry default false
		 * @param extraDirs extra dirs to scan
		 * @returns {Promise<string | undefined>}
		 */
		const getInstalledApps = async (
			getDetailInfo: boolean = false,
			extraDirs?: Array<string>,
		): Promise<string | undefined> =>
			await this.validAndTryBackend(
				async () => await this.rustBackend.getInstalledApps(getDetailInfo, extraDirs),
				this.appSearchError,
				extraDirs,
			)

		return {
			// use whenever you want
			getInstalledApps,
			screenCapture,
			screenCaptureAroundPosition,
			compress,
			decompress,
			// must use after `ioio` worker start
			getCursorPosition,
			getCursorPositionPixelColor,
			setEventChannel,
			allEventChannels,
			hasEventChannel,
			delEventChannel,
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

/** A new rubickworker client
 *
 * @param settings WorkerSettings
 * @returns {RubickBase}
 */
export const newRubickWorker = (settings?: WorkerSettings): RubickWorker => {
	return new RubickWorker(settings || {})
}
