import os from 'os'
import Mali from 'mali'
import path from 'path'
import signale from 'signale'
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
import { Evt } from 'evt'

const evtDeviceEvent = new Evt<DeviceEvent>()

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
		this.port = (port || this.getRandomNum(50000, 60000)).toString()
		this.logger = logger || signale
		this.tmpdir = tmpdir || os.tmpdir()
		// values
		this.started = false
		this.cursorPosition = { x: 0, y: 0 }
		// base init
		this.initBuiltinService()
		// create capture tmp path
		const captureTmpPath = path.resolve(this.tmpdir, 'capture')
		if (fs.existsSync(captureTmpPath)) {
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
		this.validStarted()

		const getCursorPosition = () => this.cursorPosition

		const screenCapture = async (capturePath: string, captureName?: string) => {
			if (fs.existsSync(capturePath) && fs.lstatSync(capturePath).isDirectory()) {
				captureName = captureName || Date.now().toString() + '.png'
				if (!captureName.endsWith('.png')) {
					captureName = captureName + '.png'
				}
				const captureFilePath = join(capturePath, captureName)
				try {
					await this.worker.capture(captureFilePath)
					return path.resolve(captureFilePath)
				} catch (error) {
					this.logger.error(error)
					return 'error'
				}
			} else {
				this.logger.error('No such directory!')
				return 'error'
			}
		}

		const getCursorPositionPixelColor = async () => {
			const captureTmpPath = path.resolve(this.tmpdir, 'capture')
			const capturePath = await screenCapture(captureTmpPath)
			try {
				return extendAPI.getPicturePixelColor(capturePath, await getCursorPosition())
			} catch (error) {
				this.logger.error(error)
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
		}

		return {
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

	private getRandomNum(Min: number, Max: number) {
		var Range = Max - Min
		var Rand = Math.random()
		return Min + Math.round(Rand * Range)
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
