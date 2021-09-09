const RubickServer = require('../dist').default

new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
}).start()
