const RubickServer = require('../dist').default

let server = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

const api = server.getAPI()

async function main() {
    await server.start()
    await api.screenCapture("./ .png")
}

main()
