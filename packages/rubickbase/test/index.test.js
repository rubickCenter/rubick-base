const { newRubickBase } = require('../dist')

const rubickBase = newRubickBase()

async function main() {
	// start rubickbase
	await rubickBase.start()
	const api = rubickBase.getAPI()

	// screen capture
	await api.screenCapture('./')

	// cursor Position
	let task = setInterval(async () => {
		const position = api.getCursorPosition()
		console.log("Now cursor at ", position)
		// screen around cursor
		const img = await api.screenCaptureAroundPosition(position, 100, 100)
		console.log(await img.toBase64())
	}, 2000)

	// hook device event
	const { registerHook } = rubickBase.setEventChannel({
		device: 'Mouse',
		action: 'Press',
		info: 'Left',
	})
	console.log(rubickBase.allEventChannels())
	registerHook('myeventchannel', async (e) => { console.log(e) })
	console.log(rubickBase.allEventChannels())
	setTimeout(async () => {
		await rubickBase.close()
		clearInterval(task)
	}, 10000)
}

main()
