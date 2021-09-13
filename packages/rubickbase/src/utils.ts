const rgbToHex = (r: number, g: number, b: number) => {
	return ((r << 16) | (g << 8) | b).toString(16)
}

const getRandomNum = (Min: number, Max: number) => {
	var Range = Max - Min
	var Rand = Math.random()
	return Min + Math.round(Rand * Range)
}

export { rgbToHex, getRandomNum }
