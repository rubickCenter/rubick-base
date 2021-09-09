## rubickbase

基于 Rust 提供原生能力 API 的现代 Nodejs 模块，大小 2.2M，可取代 iohook 和 robotjs

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
    await api.screenCapture("./capture.png")
}

main()
```
