import jimp from 'jimp'
import { Color, RubickExtendAPI } from './types'
import worker from './worker'

/** screen capture
 * @param path 截屏文件存放的文件路径
 */
async function screenCapture(path: string) {
    if (path.endsWith("/")) {
        path = path + Date.now().toString() + ".png"
    }
    if (!path.endsWith(".png")) {
        path = path + ".png"
    }
    await worker.capture(path)
    return path
}

/** Get pixel color at the cursur position
 * @returns 颜色
 */
async function getPixelColor(): Promise<Color> {
   
    // const img = await jimp.read()
    const hex = 0

    // pick color
    // const hex = img.getPixelColor(position.x, position.y)
    const hex16 = "#" + hex.toString(16).toUpperCase()
    const rgba = jimp.intToRGBA(hex)

    return { hex16, rgba }
}

const RubickExtendAPI: RubickExtendAPI = { getPixelColor, screenCapture }

export default RubickExtendAPI