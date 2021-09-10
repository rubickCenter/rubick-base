const newRubickBase = require('../dist').default

const rubickBase = newRubickBase()

async function main() {
    // start rubickbase
    await rubickBase.start()
    const api = rubickBase.getAPI()
    // screen capture
    await api.screenCapture("./capture.png")
    // cursor Position
    let task = setInterval(async () => {
        console.log(await api.getCursorPositionPixelColor())
    }, 1000)
    // close rubickbase
    setTimeout(async () => {
        await rubickBase.close()
        clearInterval(task)
    }, 10000)
}

main()
