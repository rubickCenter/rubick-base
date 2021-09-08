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
  listen_event_hook: (event: DeviceEvent) => {}
}

export interface RubickServerSettings {
  port: number
  env?: string
  silent?: boolean
}