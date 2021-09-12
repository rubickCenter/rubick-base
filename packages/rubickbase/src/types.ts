export interface RubickAPI extends RubickExtendAPI {
	/** get cursor position
	 * @returns {Position} 鼠标位置
	 */
	getCursorPosition: () => Position

	/** capture screen and save file to path
	 * @param capturePath 截屏文件存放的文件路径
	 * @param captureName 截屏文件存放的文件名称 默认时间戳
	 */
	screenCapture: (capturePath: string, captureName?: string) => Promise<string>

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

export type DeviceEvent = MouseEvent | KeyBoardEvent

export interface KeyBoardEvent {
	device: 'KeyBoard'
	action: 'Press' | 'Release'
	info:
		| 'Alt'
		| 'AltGr'
		| 'Backspace'
		| 'CapsLock'
		| 'ControlLeft'
		| 'ControlRight'
		| 'Delete'
		| 'DownArrow'
		| 'End'
		| 'Escape'
		| 'F1'
		| 'F10'
		| 'F11'
		| 'F12'
		| 'F2'
		| 'F3'
		| 'F4'
		| 'F5'
		| 'F6'
		| 'F7'
		| 'F8'
		| 'F9'
		| 'Home'
		| 'LeftArrow'
		| 'MetaLeft'
		| 'MetaRight'
		| 'PageDown'
		| 'PageUp'
		| 'Return'
		| 'RightArrow'
		| 'ShiftLeft'
		| 'ShiftRight'
		| 'Space'
		| 'Tab'
		| 'UpArrow'
		| 'PrintScreen'
		| 'ScrollLock'
		| 'Pause'
		| 'NumLock'
		| 'BackQuote'
		| 'Num1'
		| 'Num2'
		| 'Num3'
		| 'Num4'
		| 'Num5'
		| 'Num6'
		| 'Num7'
		| 'Num8'
		| 'Num9'
		| 'Num0'
		| 'Minus'
		| 'Equal'
		| 'KeyQ'
		| 'KeyW'
		| 'KeyE'
		| 'KeyR'
		| 'KeyT'
		| 'KeyY'
		| 'KeyU'
		| 'KeyI'
		| 'KeyO'
		| 'KeyP'
		| 'LeftBracket'
		| 'RightBracket'
		| 'KeyA'
		| 'KeyS'
		| 'KeyD'
		| 'KeyF'
		| 'KeyG'
		| 'KeyH'
		| 'KeyJ'
		| 'KeyK'
		| 'KeyL'
		| 'SemiColon'
		| 'Quote'
		| 'BackSlash'
		| 'IntlBackslash'
		| 'KeyZ'
		| 'KeyX'
		| 'KeyC'
		| 'KeyV'
		| 'KeyB'
		| 'KeyN'
		| 'KeyM'
		| 'Comma'
		| 'Dot'
		| 'Slash'
		| 'Insert'
		| 'KpReturn'
		| 'KpMinus'
		| 'KpPlus'
		| 'KpMultiply'
		| 'KpDivide'
		| 'Kp0'
		| 'Kp1'
		| 'Kp2'
		| 'Kp3'
		| 'Kp4'
		| 'Kp5'
		| 'Kp6'
		| 'Kp7'
		| 'Kp8'
		| 'Kp9'
		| 'KpDelete'
		| 'Function'
		| number
}

export type MouseEvent = MouseClickEvent | MouseMoveEvent | MouseWheelEvent

export interface MouseClickEvent {
	device: 'Mouse'
	action: 'Press' | 'Release'
	info: 'Left' | 'Right' | 'Middle' | number
}

export interface MouseWheelEvent {
	device: 'Mouse'
	action: 'Wheel'
	info: 'Up' | 'Down'
}

export interface MouseMoveEvent {
	device: 'Mouse'
	action: 'Move'
	info: Position
}

export interface Position {
	x: number
	y: number
}

export interface RubickBaseSettings {
	// grpc server port
	port?: number
	// custom logger
	logger?: Logger
	// tmpdir for file storage
	tmpdir?: string
	// event callback
	ioEventCallback?: (event: DeviceEvent) => void | Promise<void>
}

export interface Logger {
	error: Function
	debug: Function
	info: Function
	success: Function
	warn: Function
}
