const RubickServer = require('../dist').default
const fs = require('fs')

let server = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

const api = server.getAPI()

async function main() {
    const b = await api.screenCapture("./ .png")
}

main()
