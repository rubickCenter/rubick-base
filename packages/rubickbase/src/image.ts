import fs from 'fs-extra'
import mt from 'mime-types'

class Image {
	path: string

	constructor(path: string) {
		this.path = path
	}

	async toBase64(): Promise<string> {
		const buffer = await fs.readFile(this.path)
		return 'data:' + mt.lookup(this.path) + ';base64,' + buffer.toString('base64')
	}
}

const newImage = (path: string) => {
	return new Image(path)
}

export { newImage, Image }
