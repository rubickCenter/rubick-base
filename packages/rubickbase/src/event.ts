import { EventEmitter } from 'events'
import { DeviceEvent, Logger } from './types'

type EventCallback = (deviceEvent: DeviceEvent) => Promise<void>

class DeviceEventEmitter extends EventEmitter {}

class EventChannelMap extends Map<string, EventCallback> {
	logger: Logger
	constructor(logger: Logger) {
		super()
		this.logger = logger
	}
	set(key: string, value: EventCallback) {
		this.logger.info(`A new event channel [${key}] hooked`)
		return super.set(key, value)
	}
	delete(key: string) {
		this.logger.info(`Event channel [${key}] unhooked`)
		return super.delete(key)
	}
}

const deviceEventEmitter = new DeviceEventEmitter({ captureRejections: true })

export { deviceEventEmitter, EventChannelMap }
