export interface DeviceEvent {
  device: string
  action: string
  info: string | PointPosition
}

export interface PointPosition {
  x: number
  y: number
}

export interface RubickDefaultHooks {
  ioio_hook: (event: DeviceEvent) => {}
}

export interface RubickServerSettings {
  port: number
  logger?: Logger
  env?: string
  silent?: boolean
}

export interface Logger {
  error:Function
  debug:Function
  info:Function
  success:Function
  warn:Function
}