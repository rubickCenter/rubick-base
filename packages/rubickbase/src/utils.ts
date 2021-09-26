import { defaultLogger } from './logger'
import { DeviceEvent, Position } from './types'
import { createServer } from 'net'

// MIT LICENSE https://github.com/sindresorhus/rgb-hex
const rgbToHex = (red: number, green: number, blue: number, alpha?: number) => {
	alpha = alpha || 1
	if (red > 255 || green > 255 || blue > 255) {
		defaultLogger.error('Expected three numbers below 256')
	}

	if (alpha >= 0 && alpha <= 1) {
		alpha = Math.round(255 * alpha)
	}

	return (
		'#' +
		(
			(blue | (green << 8) | (red << 16) | (1 << 24)).toString(16).slice(1) +
			(alpha | (1 << 8)).toString(16).slice(1)
		).toUpperCase()
	)
}

const infoEqual = (a: string | Position | number | undefined, b: string | Position | number) =>
	typeof a === 'string' || typeof b === 'string' || typeof a === 'number' || typeof b === 'number'
		? a === b
		: a?.x === b.x && a?.y === b.y

const eventEqual = (deviceEvent: DeviceEvent, bindEvent: DeviceEvent) =>
	(bindEvent.device ? deviceEvent.device === bindEvent.device : true) &&
	(bindEvent.action ? deviceEvent.action === bindEvent.action : true) &&
	(bindEvent.info ? infoEqual(deviceEvent.info, bindEvent.info) : true)

const tryPort = (port: number): Promise<number> => {
	class ApiError extends Error {
		code: string | undefined
	}
	const server = createServer().listen(port)
	return new Promise((resolve, reject) => {
		server.on('listening', () => {
			server.close()
			resolve(port)
		})
		server.on('error', (err) => {
			if ((err as ApiError).code === 'EADDRINUSE') {
				resolve(tryPort(port + 1)) //如占用端口号+1
				console.warn(`The port ${port} is occupied try another.`)
			} else {
				reject(err)
			}
		})
	})
}

export { rgbToHex, eventEqual, tryPort }
