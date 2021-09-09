## rubick-grpc-server

基于 Rust 提供各种拓展能力的原生 Node 包

## Built-in APIs

 - [x] 键鼠事件监听
 - [x] 截图
 - [ ] 取色

## Getting start

```js
const RubickServer = require('rubick-grpc-server').default

let server = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

const api = server.getAPI()

async function main() {
    await server.start()
    await api.screenCapture("./ .png")
}

main()
```
