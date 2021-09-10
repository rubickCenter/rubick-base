const RubickServer = require('../dist').default

const server = new RubickServer()

async function main() {
    // start rubickbase
    await server.start()
    const api = server.getAPI()
    // screen capture
    await api.screenCapture("./capture.png")
    // cursor Position
    let task = setInterval(async () => {
        console.log(await api.getCursorPositionPixelColor())
    }, 1000)
    // close rubickbase
    setTimeout(async () => {
        await server.close()
        clearInterval(task)
    }, 10000)
}

main()
