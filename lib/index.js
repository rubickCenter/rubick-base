const rustBackend = require('./rust-backend')
const { expose } = require("threads/worker")

expose({
    // ioio port:string
    async ioioStart(port) { return await rustBackend.ioio_start(port) }
})
