import rustBackend from './rust-backend'

export interface WorkerAPI {
    ioioStart: (port: string) => Promise<boolean>
    capture: (path: string) => Promise<undefined>
}

const RustBackend: WorkerAPI = {
    ioioStart: async (port: string) => { return await rustBackend.ioio_start(port) },
    capture: async (path: string) => { return await rustBackend.capture_start(path) }
}
 
export default RustBackend