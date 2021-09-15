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
import fs from 'fs-extra'
import { eventEqual, getRandomNum, rgbToHex } from './utils'
import { defaultLogger } from './logger'
import { deviceEventEmitter, EventChannelMap } from './event'
import { newImage } from './image'

export class RubickBase {
	private server!: Mali<any>
	private worker!: RustBackendAPI
	private port: string
	private tmpdir: string
	private eventChannels: EventChannelMap
	private captureTmpPath: string
	private cursorPosition: Position = { x: 0, y: 0 }
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
		this.captureTmpPath = path.join(this.tmpdir, 'capture')

		// base init
		this.initBuiltinService()

		// create tmp path
		if (!fs.existsSync(this.tmpdir)) {
			fs.mkdirSync(this.tmpdir)
		}

		if (!fs.existsSync(this.captureTmpPath)) {
			fs.mkdirSync(this.captureTmpPath)
		}

		deviceEventEmitter.on('error', (err) => {
			this.logger.error(err)
		})

		// global listen event
		deviceEventEmitter.on('deviceEvent', async (event) => {
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
		deviceEventEmitter.removeAllListeners()
		await this.server.close()
		this.started = false
	}

	getAPI(): RubickAPI {
		// valid start
		if (!this.started) {
			throw new Error('Rubick has not started! Start it first!')
		}

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
					return newImage(path.resolve(captureFilePath))
				},
				newImage('error'),
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

		const screenCaptureAroundPosition = async (
			position: Position,
			width: number,
			height: number,
		) => {
			return await tryBackend(async () => {
				const capturePath = path.join(this.captureTmpPath, Date.now().toString() + '.png')
				await this.worker.screenCaptureAroundPosition(position, width, height, capturePath)
				return newImage(path.resolve(capturePath))
			}, newImage('error'))
		}

		return {
			screenCaptureAroundPosition,
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
			// post event to global event channel
			deviceEventEmitter.emit('deviceEvent', event)
			ctx.res = { ok: true }
		})
	}

	private async loadProto(): Promise<string | object> {
		let proto: string | object = './proto/rubick.proto'
		try {
			const protoJSON = await import('./proto/rubick.proto')
			proto = loadPackageDefinition(fromJSON(protoJSON as INamespace))
			this.logger.info('You are in production mode, protoJSON loaded.')
		} catch {}
		return proto
	}

	setEventChannel(bindEvent: DeviceEvent) {
		// Decorator
		const register = (name: string) => {
			return (hook: (deviceEvent: DeviceEvent) => Promise<void>) => {
				const listener = async (deviceEvent: DeviceEvent) => {
					if (eventEqual(bindEvent, deviceEvent)) await hook(deviceEvent)
				}

				// 在注册表中记录
				this.eventChannels.set(name, listener)

				// 让全局事件监听器在操作匹配的情况下向这个隧道发送消息
				deviceEventEmitter.on('deviceEvent', listener)
			}
		}

		const registerHook = (name: string, hook: (deviceEvent: DeviceEvent) => Promise<void>) => {
			register(name)(hook)
		}

		// 返回注册器
		return { register, registerHook }
	}

	allEventChannels() {
		return this.eventChannels.keys()
	}

	hasEventChannel(name: string) {
		return this.eventChannels.has(name)
	}

	delEventChannel(name: string) {
		if (this.eventChannels.has(name)) {
			// 删除全局事件的监听挂钩
			const listener = this.eventChannels.get(name)
			if (listener) deviceEventEmitter.removeListener('deviceEvent', listener)
			// 删除注册表中的隧道
			this.eventChannels.delete(name)
		} else {
			this.logger.error(`no such handler: ${name}`)
		}
	}
}

export const newRubickBase = (settings?: RubickBaseSettings) => {
	return new RubickBase(settings || {})
}
