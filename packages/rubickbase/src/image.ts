import fs from 'fs-extra'
import { PhotonImage, resize } from '@silvia-odwyer/photon-node'
import { Position } from './types'

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

	// sampling_filter - 最邻近差值算法 = 1, 二值寻找算法 = 2, CatmullRom插值算法 = 3, 高斯算法 = 4, 插值算法 = 5
	resize(width: number, height: number, sampling_filter?: number) {
		sampling_filter = sampling_filter || 1
		const img = resize(this.photonImage, width, height, sampling_filter)
		return new Image(img)
	}

	getRawPixel() {
		return this.photonImage.get_raw_pixels()
	}

	colorAt(position: Position) {
		if (
			0 < position.x &&
			position.x < this.width() &&
			0 < position.y &&
			position.y < this.height()
		) {
			const strip = this.width() * position.y - position.y + position.x
			const rawPixel = this.getRawPixel()
			// todo
			let color = rawPixel.slice(strip + 2, strip + 6)

			return color
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
