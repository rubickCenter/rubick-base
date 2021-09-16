import { defaultLogger } from './logger'
import { DeviceEvent, Position } from './types'

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

const getRandomNum = (Min: number, Max: number) => {
	var Range = Max - Min
	var Rand = Math.random()
	return Min + Math.round(Rand * Range)
}

const infoEqual = (a: string | Position | number | undefined, b: string | Position | number) =>
	typeof a === 'string' || typeof b === 'string' || typeof a === 'number' || typeof b === 'number'
		? a === b
		: a?.x === b.x && a?.y === b.y

const eventEqual = (deviceEvent: DeviceEvent, bindEvent: DeviceEvent) =>
	(bindEvent.device ? deviceEvent.device === bindEvent.device : true) &&
	(bindEvent.action ? deviceEvent.action === bindEvent.action : true) &&
	(bindEvent.info ? infoEqual(deviceEvent.info, bindEvent.info) : true)

export { rgbToHex, getRandomNum, eventEqual }
