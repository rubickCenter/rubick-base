import newRubickBase from 'rubickbase'

async function s() {
    const a = await import('rubickbase')
    console.log(newRubickBase)
}

s()
// const rubickBase = newRubickBase.default({
//     ioio_hook: (e) => {
//         console.log(e)
//     }
// })

// async function main() {
//     // start rubickbase
//     await rubickBase.start()
// }

// main()