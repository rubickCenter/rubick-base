const RubickServer = require('../dist').default

const rubickServer = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

async function main() {
    await rubickServer.start()
    // setInterval(()=>{console.log("working...")},100)
    setTimeout(async () => { await rubickServer.close() }, 5000)
}

console.log("The service will close after 5 sec!")
main()