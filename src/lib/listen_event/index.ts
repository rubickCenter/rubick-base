import ioio from './ioio'
import { expose } from "threads/worker"

expose({
    start(port: string) {
        ioio.start(port)
    },
    stop() {
        process.exit(1)
    }
})


