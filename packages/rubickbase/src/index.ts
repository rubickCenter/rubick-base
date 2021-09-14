import os from 'os'
import Mali from 'mali'
import path from 'path'
import {
	Logger,
	RubickBaseSettings,
	DeviceEvent,
	RubickExtendAPI,
	Position,
	RubickAPI,
} from './types'
import newRustBackend, { RustBackendAPI } from './worker'
import extendAPI from './extendAPI'
import { loadPackageDefinition } from '@grpc/grpc-js'
import { fromJSON } from '@grpc/proto-loader'
import { INamespace } from 'protobufjs'
import { join } from 'path'
import fs from 'fs'
import { getRandomNum, rgbToHex } from './utils'
import { evtDeviceEvent } from './event'
import { defaultLogger } from './logger'

export class RubickBase {
	private server!: Mali<any>
	private worker!: RustBackendAPI
	private port: string
	private cursorPosition: Position
	private started: boolean
	private tmpdir: string
	logger: Logger
	constructor(settings: RubickBaseSettings) {
		const { port, logger, tmpdir, ioEventCallback } = settings
		// settings
		// if no port, gen a port from 50000-60000
		this.port = (port || getRandomNum(50000, 60000)).toString()
		this.logger = logger || defaultLogger
		this.tmpdir = tmpdir || os.tmpdir()
		// values
		this.started = false
		this.cursorPosition = { x: 0, y: 0 }
		// base init
		this.initBuiltinService()
		// create capture tmp path
		const captureTmpPath = path.resolve(this.tmpdir, 'capture')
		if (!fs.existsSync(captureTmpPath)) {
			fs.mkdirSync(captureTmpPath)
		}
		// listen event
		evtDeviceEvent.attachExtract(async (event) => {
			if (ioEventCallback) await ioEventCallback(event)
			if (event.device === 'Mouse' && event.action === 'Move') {
				this.cursorPosition = event.info
			}
		})
	}

	async start() {
		this.worker = await newRustBackend()
		await this.server.start(`127.0.0.1:${this.port}`)
		await this.afterStart()
		this.started = true
	}

	async close() {
		this.validStarted()
		await this.server.close()
		this.started = false
	}

	getAPI(): RubickAPI {
		// valid start
		this.validStarted()

		// utils
		const tryBackend = async <T>(func: () => Promise<T>, errorReturn: T): Promise<T> => {
			try {
				return await func()
			} catch (error) {
				this.logger.error(error)
				return errorReturn
			}
		}

		const validDirectoryAndTryBackend = async <T>(
			path: string[] | string,
			func: () => Promise<T>,
			errorReturn: T,
		): Promise<T> => {
			if (typeof path === 'string') {
				path = [path]
			}
			let v = path.map((path) => fs.existsSync(path) && fs.lstatSync(path).isDirectory())
			if (!v.includes(false)) {
				return await tryBackend(func, errorReturn)
			} else {
				this.logger.error('No such directory!')
				return errorReturn
			}
		}

		// API
		const getCursorPosition = () => this.cursorPosition

		const screenCapture = async (capturePath: string, captureName?: string) => {
			return await validDirectoryAndTryBackend(
				capturePath,
				async () => {
					captureName = captureName || Date.now().toString() + '.png'
					if (!captureName.endsWith('.png')) {
						captureName = captureName + '.png'
					}
					const captureFilePath = join(capturePath, captureName)
					await this.worker.capture(captureFilePath)
					return path.resolve(captureFilePath)
				},
				'error',
			)
		}

		const getPicturePixelColor = async (path: string, position: Position) => {
			return await tryBackend(
				async () => {
					const rgba = await this.worker.colorPicker(path, position)
					return { hex16: rgbToHex(rgba.r, rgba.g, rgba.b, rgba.a), rgba }
				},
				{
					hex16: 'error',
					rgba: {
						r: -1,
						g: -1,
						b: -1,
						a: -1,
					},
				},
			)
		}

		const getCursorPositionPixelColor = async () => {
			return await tryBackend(
				async () => {
					const rgb = await this.worker.screenColorPicker(getCursorPosition())
					return {
						hex16: rgbToHex(rgb.r, rgb.g, rgb.b),
						rgba: {
							r: rgb.r,
							g: rgb.g,
							b: rgb.b,
							a: 255,
						},
					}
				},
				{
					hex16: 'error',
					rgba: {
						r: -1,
						g: -1,
						b: -1,
						a: -1,
					},
				},
			)
		}

		const compress = async (fromPath: string, toPath: string) =>
			await validDirectoryAndTryBackend(
				[fromPath, toPath],
				async () => await this.worker.compress(fromPath, toPath),
				undefined,
			)

		const decompress = async (fromPath: string, toPath: string) =>
			await validDirectoryAndTryBackend(
				[fromPath, toPath],
				async () => await this.worker.decompress(fromPath, toPath),
				undefined,
			)

		return {
			compress,
			decompress,
			getPicturePixelColor,
			getCursorPosition,
			screenCapture,
			getCursorPositionPixelColor,
			...extendAPI,
		}
	}

	// can work without server start
	getExtendedAPI(): RubickExtendAPI {
		return extendAPI
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
	private async initBuiltinService() {
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
			evtDeviceEvent.post(event)
			ctx.res = { ok: true }
		})
	}

	private validStarted() {
		if (!this.started) {
			throw new Error('Rubick has not started! Start it first!')
		}
	}

	private async loadProto(): Promise<string | object> {
		let proto
		try {
			const protoJSON = await import('./proto/rubick.proto')
			proto = loadPackageDefinition(fromJSON(protoJSON as INamespace))
			this.logger.info('You are in production mode, protoJSON loaded.')
		} catch {
			this.logger.info('You are in development mode, load proto from file.')
			proto = './proto/rubick.proto'
		}
		return proto
	}
}

export const newRubickBase = (settings?: RubickBaseSettings) => {
	return new RubickBase(settings || {})
}
