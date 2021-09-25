import { Position, RGB } from './types'

export interface RustBackendAPI {
	ioioStart: (port: string) => Promise<boolean>
	captureToBase64: () => Promise<string>
	screenColorPicker: (position: Position) => Promise<RGB>
	compress: (fromPath: string, toPath: string) => Promise<undefined>
	decompress: (fromPath: string, toPath: string) => Promise<undefined>
	screenCaptureAroundPositionToBase64: (
		position: Position,
		width: number,
		height: number,
	) => Promise<string>
	getInstalledApps: (getDetailInfo: boolean, extraDirs?: Array<string>) => Promise<string>
	// Deprecated
	// capture: (path: string) => Promise<undefined>
	// colorPicker: (path: string, position: Position) => Promise<RGBA>
	// screenCaptureAroundPosition: (
	// 	position: Position,
	// 	width: number,
	// 	height: number,
	// 	path: string,
	// ) => Promise<undefined>
}

async function newRustBackend(): Promise<RustBackendAPI> {
	const rustBackend = await import(`rubick_backend-${process.platform}`)
	return {
		ioioStart: async (port: string) => {
			return await rustBackend.ioio_start(port)
		},
		captureToBase64: async () => {
			return await rustBackend.capture_base64_start()
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
		getInstalledApps: async (getDetailInfo: boolean, extraDirs?: Array<string>) => {
			return await rustBackend.find_apps_start(getDetailInfo, extraDirs || [])
		},
		// Deprecated
		// capture: async (path: string) => {
		// 	return await rustBackend.capture_start(path)
		// },
		// colorPicker: async (path: string, position: Position) => {
		// 	return await rustBackend.color_picker_start(path, position.x, position.y)
		// },
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
	}
}

export default newRustBackend
