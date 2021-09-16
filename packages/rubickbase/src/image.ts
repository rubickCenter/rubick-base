import fs from 'fs-extra'
import { PhotonImage, resize } from '@silvia-odwyer/photon-node'
import { Color, Position } from './types'
import { rgbToHex } from './utils'

class Image {
	private photonImage: PhotonImage
	constructor(photonImage: PhotonImage) {
		this.photonImage = photonImage
	}

	toBase64(): string {
		return this.photonImage.get_base64()
	}

	width(): number {
		return this.photonImage.get_width()
	}

	height(): number {
		return this.photonImage.get_height()
	}

	async save(path: string) {
		let output_base64 = this.photonImage.get_base64()
		const output_data = output_base64.replace(/^data:image\/\w+;base64,/, '')
		await fs.writeFile(path, output_data, { encoding: 'base64' })
	}

	/** resize the image
	 * @param width
	 * @param height
	 * @param sampling_filter 最邻近差值算法 = 1, 二值寻找算法 = 2, CatmullRom插值算法 = 3, 高斯算法 = 4, 插值算法 = 5
	 * @returns {Image}
	 */
	resize(width: number, height: number, sampling_filter?: number): Image {
		sampling_filter = sampling_filter || 1
		const img = resize(this.photonImage, width, height, sampling_filter)
		return new Image(img)
	}

	getRawPixel() {
		return this.photonImage.get_raw_pixels()
	}

	/** get pixel color at picture position
	 * @param position 取色位置
	 * @return {Color} 位置像素颜色
	 */
	colorAt(position: Position): Color {
		if (
			0 < position.x &&
			position.x <= this.width() &&
			0 < position.y &&
			position.y <= this.height()
		) {
			const strip = 4 * (this.width() * (position.y - 1) + position.x)
			const color = this.getRawPixel().slice(strip - 4, strip)
			return <Color>{
				hex16: rgbToHex(color[0], color[1], color[2], color[3]),
				rgba: {
					r: color[0],
					g: color[1],
					b: color[2],
					a: color[3],
				},
			}
		} else {
			throw new Error('position out of bounds!')
		}
	}
}

const newImageFromFile = async (path: string): Promise<Image> => {
	let base64 = await fs.readFile(path, { encoding: 'base64' })
	const data = base64.replace(/^data:image\/(png|jpg);base64,/, '')
	try {
		const img = PhotonImage.new_from_base64(data)
		return new Image(img)
	} catch (error) {
		throw error
	}
}

const newImageFromBase64 = (base64: string): Image => {
	if (base64 === 'error') {
		throw new Error('error image')
	}
	const data = base64.replace(/^data:image\/(png|jpg);base64,/, '')
	try {
		const img = PhotonImage.new_from_base64(data)
		return new Image(img)
	} catch (error) {
		throw error
	}
}

export { Image, newImageFromFile, newImageFromBase64 }
