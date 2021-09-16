declare namespace rubick_backend {
	function ioio_start(port: string): Promise<boolean>
	function capture_base64_start(): Promise<string>
	function screen_color_picker_start(x: number, y: number): Promise<RGB>
	function lzma_compress_start(fromPath: string, toPath: string): Promise<undefined>
	function lzma_decompress_start(fromPath: string, toPath: string): Promise<undefined>
	function screen_capture_rect_base64_start(
		x: number,
		y: number,
		width: number,
		height: number,
	): Promise<string>
	// Deprecated
	// function capture_start(path: string): Promise<undefined>
	// function color_picker_start(path: string, x: number, y: number): Promise<RGBA>
	// function screen_capture_rect_start(
	// 	x: number,
	// 	y: number,
	// 	width: number,
	// 	height: number,
	// 	path: string,
	// ): Promise<undefined>
}

interface RGBA {
	r: number
	g: number
	b: number
	a: number
}

interface RGB {
	r: number
	g: number
	b: number
}

export = rubick_backend
