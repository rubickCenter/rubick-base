import { newRubickBase } from 'rubickbase'

const rubickBase = newRubickBase({
	// device event listener
	ioEventCallback: (e) => { console.log(e) }
})

async function main() {
	// start rubickbase
	await rubickBase.start()
	const api = rubickBase.getAPI()
	// screen capture
	await api.screenCapture('./capture.png')
	// cursor Position
	let task = setInterval(async () => {
		console.log("cursor at ", api.getCursorPosition())
	}, 1000)
	// close rubickbase
	setTimeout(async () => {
		await rubickBase.close()
		clearInterval(task)
	}, 10000)
}

main()