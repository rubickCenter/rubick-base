export interface RustBackendAPI {
	ioioStart: (port: string) => Promise<boolean>
	capture: (path: string) => Promise<undefined>
}

async function newRustBackend(): Promise<RustBackendAPI> {
	const rustBackend = await import(`rubick_backend-${process.platform}`)
	return {
		ioioStart: async (port: string) => {
			return await rustBackend.ioio_start(port)
		},
		capture: async (path: string) => {
			return await rustBackend.capture_start(path)
		},
	}
}

export default newRustBackend
