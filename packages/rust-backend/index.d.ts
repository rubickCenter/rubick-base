declare namespace rubick_backend {
	function ioio_start(port: string): Promise<boolean>
	function capture_start(path: string): Promise<undefined>
	function color_picker_start(path: string, x: number, y: number): Promise<RGBA>
}

interface RGBA {
	r: number
	g: number
	b: number
	a: number
}

export = rubick_backend
