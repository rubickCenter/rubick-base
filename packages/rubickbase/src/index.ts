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
} from './types'
import worker, { WorkerAPI } from './worker'
import extendAPI from './extendAPI'
import os from 'os'

const proto_path = '../../proto/rubick.proto'

export class RubickBase {
	private server: Mali<any>
	private port: string
	private defaultHooks: RubickDefaultHooks
	private worker: WorkerAPI
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
		this.server = new Mali(path.resolve(__dirname, proto_path), 'Rubick')
		this.worker = worker
		this.initBuiltinService()
		this.started = false
	}

	async start() {
		await this.server.start(`0.0.0.0:${this.port}`)
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

		const screenCapture = async (capturePath: string) => {
			if (capturePath.endsWith('/')) {
				capturePath = capturePath + Date.now().toString() + '.png'
			}
			if (!capturePath.endsWith('.png')) {
				capturePath = capturePath + '.png'
			}
			await worker.capture(capturePath)
			return path.resolve(capturePath)
		}

		const getCursorPositionPixelColor = async () => {
			const capturePath = await screenCapture(path.resolve(this.tmpdir, 'capture/'))
			return extendAPI.getPicturePixelColor(capturePath, getCursorPosition())
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
		log(await this.worker.ioioStart(this.port), 'ioio')
	}

	// registe builtin RPC services
	private initBuiltinService() {
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
}

export interface NewRubickBase {
	settings?: RubickBaseSettings
	defaultHooks?: RubickDefaultHooks
}

export const newRubickBase = (setting?: NewRubickBase) => {
	const { settings, defaultHooks } = setting || {}
	return new RubickBase(settings || {}, defaultHooks || {})
}
