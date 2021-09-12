import os from 'os'
import Mali from 'mali'
import path from 'path'
import signale from 'signale'
import {
	RubickDefaultHooks,
	Logger,
	RubickBaseSettings,
	DeviceEvent,
	RubickExtendAPI,
	Position,
	RubickAPI,
	Color,
} from './types'
import newRustBackend, { RustBackendAPI } from './worker'
import extendAPI from './extendAPI'
import { loadPackageDefinition } from '@grpc/grpc-js'
import { fromJSON } from '@grpc/proto-loader'
import { INamespace } from 'protobufjs'
import { join } from 'path'
import fs from 'fs'

export class RubickBase {
	private server!: Mali<any>
	private worker!: RustBackendAPI
	private port: string
	private defaultHooks: RubickDefaultHooks
	private cursorPosition: Position
	private started: boolean
	private tmpdir: string
	logger: Logger
	constructor(settings: RubickBaseSettings, defaultHooks: RubickDefaultHooks) {
		const { port, logger, tmpdir } = settings
		// if no port, gen a port from 50000-60000
		this.port = (port || this.getRandomNum(50000, 60000)).toString()
		this.tmpdir = tmpdir || os.tmpdir()
		this.defaultHooks = defaultHooks
		this.logger = logger || signale
		this.cursorPosition = { x: 0, y: 0 }
		this.started = false
		this.initBuiltinService()
		// create capture tmp path
		const captureTmpPath = path.resolve(this.tmpdir, 'capture')
		if (fs.existsSync(captureTmpPath)) {
			fs.mkdirSync(captureTmpPath)
		}
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
				return extendAPI.getPicturePixelColor(capturePath, getCursorPosition())
			} catch (error) {
				this.logger.error(error)
				return <Color>{}
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
			// mousemove info is still string here convert to Position
			if ((event.info as string).startsWith('{')) {
				event.info = JSON.parse(event.info as string)
				this.cursorPosition = event.info as Position
			}
			if (this.defaultHooks.ioio_hook) await this.defaultHooks.ioio_hook(event)
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

export interface NewRubickBase {
	settings?: RubickBaseSettings
	defaultHooks?: RubickDefaultHooks
}

export const newRubickBase = (setting?: NewRubickBase) => {
	const { settings, defaultHooks } = setting || {}
	return new RubickBase(settings || {}, defaultHooks || {})
}
