import jimp from 'jimp'
import { Color, Position, RubickExtendAPI } from './types'

/** Get pixel color from picture position
 * @returns 颜色
 */
async function getPicturePixelColor(path: string, position: Position): Promise<Color> {
    // read file
    const img = await jimp.read(path)

    // pick color
    const hex = img.getPixelColor(position.x, position.y)
    const hex16 = "#" + hex.toString(16).toUpperCase()
    const rgba = jimp.intToRGBA(hex)

    return { hex16, rgba }
}

const extendAPI: RubickExtendAPI = { getPicturePixelColor }

export default extendAPI