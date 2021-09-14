import { defaultLogger } from './logger'

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

export { rgbToHex, getRandomNum }
