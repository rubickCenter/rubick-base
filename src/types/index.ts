
export interface RubickDefaultHooks {
  // mouse/keyboard event listen
  ioio_hook: (event: DeviceEvent) => {}
}

export interface RubickExtendAPI {
  // capture screen and down to path
  screenCapture: (path: string) => Promise<string>
    
  // Get pixel color at the cursur position
  getPixelColor: () => Promise<Color>
}

export interface RGBA {
  r: number
  g: number
  b: number
  a: number
}

export interface Color {
  hex16: string
  rgba: RGBA
}

export interface DeviceEvent {
  device: string
  action: string
  info: string | PointPosition
}

export interface PointPosition {
  x: number
  y: number
}

export interface RubickServerSettings {
  port: number
  logger?: Logger
  env?: string
  silent?: boolean
}

export interface Logger {
  error: Function
  debug: Function
  info: Function
  success: Function
  warn: Function
}