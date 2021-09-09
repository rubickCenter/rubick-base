import rustBackend from './rust-backend'

export interface API {
    ioioStart:  (port: string) => Promise<boolean>    
}

const BackEnd: API = {
    ioioStart: async (port: string) => { return await rustBackend.ioio_start(port) }
}
 

export default BackEnd