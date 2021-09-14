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
	ButtonEvent,
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
import { Ctx, Evt } from 'evt'

export class RubickBase {
	private server!: Mali<any>
	private worker!: RustBackendAPI
	private port: string
	private tmpdir: string
	private cursorPosition: Position = { x: 0, y: 0 }
	private started: boolean = false
	private eventChannels: Map<string, Ctx<void>> = new Map()
	logger: Logger
	constructor(settings: RubickBaseSettings) {
		const { port, logger, tmpdir, ioEventCallback } = settings
		// settings
		// if no port, gen a port from 50000-60000
		this.port = (port || getRandomNum(50000, 60000)).toString()
		this.logger = logger || defaultLogger
		this.tmpdir = tmpdir || os.tmpdir()

		// base init
		this.initBuiltinService()

		// create tmp path
		if (!fs.existsSync(this.tmpdir)) {
			fs.mkdirSync(this.tmpdir)
		}

		// event attach/detach annotations
		evtDeviceEvent.evtAttach.attach((handler) => {
			this.logger.info(`${handler.callback?.name} attached`)
		})

		evtDeviceEvent.evtDetach.attach((handler) => {
			this.logger.info(`${handler.callback?.name} attached`)
		})

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

		// 调用 rust-backend 并捕捉异常
		const tryBackend = async <T>(func: () => Promise<T>, errorReturn: T): Promise<T> => {
			try {
				return await func()
			} catch (error) {
				this.logger.error(error)
				return errorReturn
			}
		}

		// 检查目录和文件名是否合法
		const validAndTryBackend = async <T>(
			func: () => Promise<T>,
			errorReturn: T,
			dic: string[] | string = [],
			file: string[] | string = [],
		): Promise<T> => {
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
				return await tryBackend(func, errorReturn)
			} else {
				this.logger.error('No such directory!')
				return errorReturn
			}
		}

		// API
		// 获取鼠标位置
		const getCursorPosition = () => this.cursorPosition

		// 截屏
		const screenCapture = async (capturePath: string, captureName?: string) =>
			await validAndTryBackend(
				async () => {
					// 默认名称为时间戳
					captureName = captureName || Date.now().toString() + '.png'
					// 检查 png 后辍
					if (!captureName.endsWith('.png')) {
						captureName = captureName + '.png'
					}
					const captureFilePath = join(capturePath, captureName)
					await this.worker.capture(captureFilePath)
					return path.resolve(captureFilePath)
				},
				'error',
				capturePath,
			)

		// 获取图片位置像素
		const getPicturePixelColor = async (path: string, position: Position) =>
			await tryBackend(
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

		// 获取光标位置像素
		const getCursorPositionPixelColor = async () =>
			await tryBackend(
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

		// lzma2 压缩文件
		const compress = async (fromPath: string, toPath: string) =>
			await validAndTryBackend(
				async () => await this.worker.compress(fromPath, toPath),
				undefined,
				[],
				[fromPath, toPath],
			)

		// lzma2 解压文件
		const decompress = async (fromPath: string, toPath: string) =>
			await validAndTryBackend(
				async () => await this.worker.decompress(fromPath, toPath),
				undefined,
				[],
				[fromPath, toPath],
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

	private register(name: string, bindEvent: ButtonEvent) {
		// todo 多个条件 (同时多个键保持按下的状态)
		// todo 设置延迟触发时间 (一个键被按下的时间)
		bindEvent.time = bindEvent.time || 0

		const eventChannel = new Evt<DeviceEvent>()
		const ctx = Evt.newCtx()

		// 创建一个隧道并在注册表中记录
		this.eventChannels.set(name, ctx)

		// 让全局事件监听器在键名和操作都匹配的情况下向这个隧道发送消息
		evtDeviceEvent.attach(
			(deviceEvent) =>
				deviceEvent.info === bindEvent.name && deviceEvent.action === bindEvent.action,
			ctx,
			(deviceEvent) => {
				eventChannel.post(deviceEvent)
			},
		)

		// todo 拓展 Evt 类型 实现全局事件注册管理器
		const deleteChannel = () => {
			// 删除全局事件的监听挂钩
			const ctx = this.eventChannels.get(name)
			if (ctx) evtDeviceEvent.detach(ctx)
			// 删除注册表中的隧道
			this.eventChannels.delete(name)
		}

		// 返回这个隧道
		return { eventChannel, deleteChannel }
	}
}

export const newRubickBase = (settings?: RubickBaseSettings) => {
	return new RubickBase(settings || {})
}
