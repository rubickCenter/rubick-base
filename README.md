## rubickbase

基于 Rust 提供原生能力 API 的现代 Nodejs 模块，大小 2.2M，可取代 iohook 和 robotjs

## Install

npm install --save rubickbase

## Built-in APIs

 - [x] 键鼠事件监听
 - [x] 截图
 - [ ] 取色
 - [ ] 获取鼠标位置
 - [ ] 注册快捷键

## Getting start

```js
// cjs
const RubickServer = require('rubickbase').default
// esm
import RubickServer from 'rubickbase'

let server = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

const api = server.getAPI()

async function main() {
    // start grpc service
    await server.start()
    // screen capture
    await api.screenCapture("./capture.png")
}

main()
```
