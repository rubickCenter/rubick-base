import { DeviceEvent, Position, RGB } from './types'

export interface RustBackendAPI {
	ioioStart: (port: string) => Promise<boolean>
	captureToBase64: () => Promise<string>
	screenColorPicker: (position: Position) => Promise<RGB>
	screenCaptureAroundPositionToBase64: (
		position: Position,
		width: number,
		height: number,
	) => Promise<string>
	getInstalledApps: (getDetailInfo: boolean, extraDirs?: Array<string>) => Promise<string>
	sendEvent: (event: DeviceEvent) => Promise<undefined>
	language: () => Promise<string>
	captureAllToBase64: () => Promise<Array<string>>
	asarList(path: string): Promise<Array<string>>
	asarExtractFile(path: string, dest: string): Promise<undefined>
	asarExtract(path: string, dest: string): Promise<undefined>
	asarPack(path: string, dest: string, level?: number): Promise<undefined>
	// Deprecated
	// compress: (fromPath: string, toPath: string) => Promise<undefined>
	// decompress: (fromPath: string, toPath: string) => Promise<undefined>
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
	let rustBackend = await import(`rubick_backend-${process.platform}`)
	if (!!rustBackend.default) {
		rustBackend = rustBackend.default
	}
	return {
		asarList: async (path: string) => {
			return await rustBackend.asar_list(path)
		},
		asarExtract: async (path: string, dest: string) => {
			return await rustBackend.asar_extract(path, dest)
		},
		asarExtractFile: async (path: string, dest: string) => {
			return await rustBackend.asar_extract_file(path, dest)
		},
		asarPack: async (path: string, dest: string, level?: number) => {
			level = level || 0
			if (level < 0) level = 0
			if (level > 21) level = 21
			return await rustBackend.asar_pack(path, dest, level)
		},
		captureAllToBase64: async () => {
			return await rustBackend.capture_all_base64_start()
		},
		ioioStart: async (port: string) => {
			return await rustBackend.ioio_start(port)
		},
		captureToBase64: async () => {
			return await rustBackend.capture_base64_start()
		},
		screenColorPicker: async (position: Position) => {
			return await rustBackend.screen_color_picker_start(position.x, position.y)
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
		sendEvent: async (event: DeviceEvent) => {
			if (!event.device || !event.action || !event.info) {
				throw new Error('Not valid event!')
			}
			return await rustBackend.send_event_start(event.device, event.action, event.info)
		},
		language: async () => {
			return await rustBackend.current_locale_language()
		},
		// Deprecated
		// compress: async (fromPath: string, toPath: string) => {
		// 	return await rustBackend.lzma_compress_start(fromPath, toPath)
		// },
		// decompress: async (fromPath: string, toPath: string) => {
		// 	return await rustBackend.lzma_decompress_start(fromPath, toPath)
		// },
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
