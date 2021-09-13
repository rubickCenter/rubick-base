import { defaultLogger } from './logger'

// MIT LICENSE https://github.com/sindresorhus/rgb-hex
const rgbToHex = (red: number, green: number, blue: number, alpha?: number) => {
	alpha = alpha || 1
	const isPercent = (red + (alpha || '').toString()).toString().includes('%')

	if (
		typeof red !== 'number' ||
		typeof green !== 'number' ||
		typeof blue !== 'number' ||
		red > 255 ||
		green > 255 ||
		blue > 255
	) {
		defaultLogger.error('Expected three numbers below 256')
	}

	if (!isPercent && alpha >= 0 && alpha <= 1) {
		alpha = Math.round(255 * alpha)
	} else if (isPercent && alpha >= 0 && alpha <= 100) {
		alpha = Math.round((255 * alpha) / 100)
	} else {
		defaultLogger.error(`Expected alpha value (${alpha}) as a fraction or percentage`)
	}

	return (
		'#' +
		(
			(blue | (green << 8) | (red << 16) | (1 << 24)).toString(16).slice(1) +
			(alpha | (1 << 8)).toString(16).slice(1).toString()
		).toUpperCase()
	)
}

const getRandomNum = (Min: number, Max: number) => {
	var Range = Max - Min
	var Rand = Math.random()
	return Min + Math.round(Rand * Range)
}

export { rgbToHex, getRandomNum }
