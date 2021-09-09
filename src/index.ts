import Mali from 'mali'
import path from 'path'
import { spawn, Worker, Thread } from "threads"
import signale from 'signale'
import { RubickDefaultHooks, Logger, RubickServerSettings, DeviceEvent } from './types'
import RubickServerClass from './types'

const proto_path = '../proto/rubick.proto'
const backend_path = '../lib/index'

export default class RubickServer implements RubickServerClass {
  server: Mali<any>
  port: string
  env: string
  silent: boolean
  defaultHooks: RubickDefaultHooks
  worker: any
  logger: Logger
  // logger: any
  constructor(settings: RubickServerSettings, defaultHooks: RubickDefaultHooks) {
    const { port, logger, env, silent } = settings
    this.server = new Mali(path.resolve(__dirname, proto_path), 'Rubick')
    this.port = port.toString()
    this.defaultHooks = defaultHooks
    this.logger = logger || signale
    this.env = env || 'development'
    this.silent = silent || false
    this.initBuiltinService()
  }

  async start() {
    await this.server.start(`0.0.0.0:${this.port}`)
    this.worker = await spawn(new Worker(backend_path))
    await this.afterStart()
  }

  async close() {
    await this.beforeClose()
    await this.server.close()
  }

  private async afterStart() {
    const log = (success: boolean, name: string) => { if (success) { this.logger.success(`Start ${name} worker`) } else { this.logger.error(`Start ${name} worker`) } }
    // start workers
    log(await this.worker.ioioStart(this.port), "ioio")
  }

  private async beforeClose() {
    // close worker
    await Thread.terminate(this.worker)
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

  // async addService(proto: string | object, name: string, func: Function) {
  //   this.mali.addService(proto, name)
  //   this.mali.use(name, func)
  // }
}
