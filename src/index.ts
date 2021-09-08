import Mali from 'mali'
import path from 'path'
import type { DeviceEvent, RubickDefaultHooks, RubickServerSettings } from './global'
import { spawn, Worker } from "threads"

const proto_path = './static/rubick.proto'
const listen_event_worker_path = './lib/listen_event/index'

export default class RubickServer {
  server: Mali<any>
  port: string
  env: string
  silent: boolean
  workers: any[]
  defaultHooks: RubickDefaultHooks
  constructor(settings: RubickServerSettings, defaultHooks: RubickDefaultHooks) {
    const { port, env, silent } = settings
    this.server = new Mali(path.resolve(__dirname, proto_path), 'Rubick')
    this.port = port.toString()
    this.env = env || 'development'
    this.silent = silent || false
    this.workers = []
    this.defaultHooks = defaultHooks
    this.init()
  }

  async start() {
    let grpcServer = await this.server.start(`0.0.0.0:${this.port}`)
    await this.afterStart()
    return grpcServer
  }

  async close() {
    await this.beforeClose()
    await this.server.close()
  }

  private async afterStart() {
    // open workers
    // listen event
    const listen_event_worker = await spawn(new Worker(listen_event_worker_path))
    listen_event_worker.start(this.port)
  
    this.workers.push(...[
      listen_event_worker
    ])
  }

  private async beforeClose() {
    // close all workers
    this.workers.map(async (worker) => await worker?.stop())
  }

  // registe default RPC services
  private init() {
    this.server.use('listen_event', (ctx: any) => {
      const event: DeviceEvent = ctx.request.req
      // info is still string here
      if ((event.info as string).startsWith("{")) {
        event.info = JSON.parse((event.info as string))
      }
      this.defaultHooks.listen_event_hook(event)
      ctx.res = { ok: true }
    })
  }

  // async addService(proto: string | object, name: string, func: Function) {
  //   this.mali.addService(proto, name)
  //   this.mali.use(name, func)
  // }
}
