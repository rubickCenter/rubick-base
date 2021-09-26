declare namespace rubick_backend {
	// input simulation
	function send_event_start(
		device: string,
		action: string,
		info: string | number | { x: number; y: number },
	): Promise<undefined>
	// app finder
	function find_apps_start(detail_json: boolean, extra_dirs?: Array<string>): Promise<string>
	// input listen
	function ioio_start(port: string): Promise<boolean>
	// screen capture
	function capture_base64_start(): Promise<string>
	// screen color picker
	function screen_color_picker_start(
		x: number,
		y: number,
	): Promise<{
		r: number
		g: number
		b: number
	}>
	// compress
	function lzma_compress_start(fromPath: string, toPath: string): Promise<undefined>
	// decompress
	function lzma_decompress_start(fromPath: string, toPath: string): Promise<undefined>
	// capture screen around position
	function screen_capture_rect_base64_start(
		x: number,
		y: number,
		width: number,
		height: number,
	): Promise<string>
	// Deprecated
	// function capture_start(path: string): Promise<undefined>
	// function color_picker_start(
	// 	path: string,
	// 	x: number,
	// 	y: number,
	// ): Promise<{
	// 	r: number
	// 	g: number
	// 	b: number
	// 	a: number
	// }>
	// function screen_capture_rect_start(
	// 	x: number,
	// 	y: number,
	// 	width: number,
	// 	height: number,
	// 	path: string,
	// ): Promise<undefined>
}

export = rubick_backend
