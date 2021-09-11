declare namespace rubick_backend {
	function ioio_start(port: string): Promise<boolean>
	function capture_start(path: string): Promise<undefined>
}

export = rubick_backend
