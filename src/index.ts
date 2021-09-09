import Mali from 'mali'
import path from 'path'
import signale from 'signale'
import { RubickDefaultHooks, Logger, RubickServerSettings, DeviceEvent, RubickExtendAPI } from './types'
import worker, { WorkerAPI } from './worker'
import apis from './apis'

const proto_path = '../proto/rubick.proto'

export default class RubickServer {
  server: Mali<any>
  port: string
  env: string
  silent: boolean
  defaultHooks: RubickDefaultHooks
  logger: Logger
  worker: WorkerAPI
  apis: RubickExtendAPI
  constructor(settings: RubickServerSettings, defaultHooks: RubickDefaultHooks) {
    const { port, logger, env, silent } = settings
    this.port = port.toString()
    this.defaultHooks = defaultHooks
    this.logger = logger || signale
    this.env = env || 'development'
    this.silent = silent || false
    this.server = new Mali(path.resolve(__dirname, proto_path), 'Rubick')
    this.worker = worker
    this.apis = apis
    this.initBuiltinService()
  }

  async start() {
    await this.server.start(`0.0.0.0:${this.port}`)
    await this.afterStart()
  }

  async close() {
    await this.server.close()
  }

  getAPI() {
    return apis
  }

  private async afterStart() {
    const log = (success: boolean, name: string) => { if (success) { this.logger.success(`Start ${name} worker`) } else { this.logger.error(`Start ${name} worker`) } }
    // start workers
    log(await this.worker.ioioStart(this.port), "ioio")
  }

  // registe builtin RPC services
  private initBuiltinService() {
    this.server.use('ioio', (ctx: any) => {
      const event: DeviceEvent = ctx.request.req
      // info is still string here
      if ((event.info as string).startsWith("{")) {
        event.info = JSON.parse((event.info as string))
      }
      this.defaultHooks.ioio_hook(event)
      ctx.res = { ok: true }
    })
  }
}
