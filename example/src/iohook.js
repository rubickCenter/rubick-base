import RubickBase from 'rubickbase'

const rubickBase = new RubickBase({
    ioio_hook: (e) => {
        console.log(e)
    }
})

async function main() {
    // start rubickbase
    await rubickBase.start()
}

main()