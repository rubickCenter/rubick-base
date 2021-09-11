export interface RubickDefaultHooks {
	// mouse/keyboard event listen hook
	ioio_hook?: (event: DeviceEvent) => Promise<void>
}

export interface RubickAPI extends RubickExtendAPI {
	/** get cursor position
	 * @returns {Position} 鼠标位置
	 */
	getCursorPosition: () => Position

	/** capture screen and save file to path
	 * @param capturePath 截屏文件存放的文件路径
	 */
	screenCapture: (path: string) => Promise<string>

	/** get pixel color at cursor position
	 * @return {Promise<Color>} 鼠标位置像素颜色
	 */
	getCursorPositionPixelColor: () => Promise<Color>
}

export interface RubickExtendAPI {
	// Get pixel color at the cursor position
	getPicturePixelColor: (path: string, position: Position) => Promise<Color>
}

export interface RGBA {
	r: number
	g: number
	b: number
	a: number
}

export interface Color {
	hex16: string
	rgba: RGBA
}

export interface DeviceEvent {
	device: string
	action: string
	info: string | Position
}

export interface Position {
	x: number
	y: number
}

export interface RubickBaseSettings {
	port?: number
	logger?: Logger
	tmpdir?: string
}

export interface Logger {
	error: Function
	debug: Function
	info: Function
	success: Function
	warn: Function
}
