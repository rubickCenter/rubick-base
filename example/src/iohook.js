import { newRubickBase } from 'rubickbase'

const rubickBase = newRubickBase({
    defaultHooks: {
        ioio_hook: (e) => {
            console.log(e)
        }
    }
})

async function main() {
    await rubickBase.start()
}

main()
