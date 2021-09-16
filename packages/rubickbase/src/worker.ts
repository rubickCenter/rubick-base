import { Position, RGB, RGBA } from './types'

export interface RustBackendAPI {
	ioioStart: (port: string) => Promise<boolean>
	// capture: (path: string) => Promise<undefined>
	captureToBase64: () => Promise<string>
	colorPicker: (path: string, position: Position) => Promise<RGBA>
	screenColorPicker: (position: Position) => Promise<RGB>
	compress: (fromPath: string, toPath: string) => Promise<undefined>
	decompress: (fromPath: string, toPath: string) => Promise<undefined>
	// screenCaptureAroundPosition: (
	// 	position: Position,
	// 	width: number,
	// 	height: number,
	// 	path: string,
	// ) => Promise<undefined>
	screenCaptureAroundPositionToBase64: (
		position: Position,
		width: number,
		height: number,
	) => Promise<string>
}

async function newRustBackend(): Promise<RustBackendAPI> {
	const rustBackend = await import(`rubick_backend-${process.platform}`)
	return {
		ioioStart: async (port: string) => {
			return await rustBackend.ioio_start(port)
		},
		// capture: async (path: string) => {
		// 	return await rustBackend.capture_start(path)
		// },
		captureToBase64: async () => {
			return await rustBackend.capture_base64_start()
		},
		colorPicker: async (path: string, position: Position) => {
			return await rustBackend.color_picker_start(path, position.x, position.y)
		},
		screenColorPicker: async (position: Position) => {
			return await rustBackend.screen_color_picker_start(position.x, position.y)
		},
		compress: async (fromPath: string, toPath: string) => {
			return await rustBackend.lzma_compress_start(fromPath, toPath)
		},
		decompress: async (fromPath: string, toPath: string) => {
			return await rustBackend.lzma_decompress_start(fromPath, toPath)
		},
		// screenCaptureAroundPosition: async (
		// 	position: Position,
		// 	width: number,
		// 	height: number,
		// 	path: string,
		// ) => {
		// 	return await rustBackend.screen_capture_rect_start(
		// 		position.x,
		// 		position.y,
		// 		width,
		// 		height,
		// 		path,
		// 	)
		// },
		screenCaptureAroundPositionToBase64: async (
			position: Position,
			width: number,
			height: number,
		) => {
			return await rustBackend.screen_capture_rect_base64_start(
				position.x,
				position.y,
				width,
				height,
			)
		},
	}
}

export default newRustBackend
