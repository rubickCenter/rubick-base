## rubick-grpc-server

rubick 的 API 拓展层基座

## Built-in APIs

 - [x] 键鼠事件监听
 - [ ] 截图
 - [ ] 取色

## Getting start

```js
const RubickServer = require('../dist').default

const rubickServer = new RubickServer({
    port: 50051
}, {
    ioio_hook: async (e) => {
        console.log(e)
    }
})

async function main() {
    await rubickServer.start()
    setTimeout(async () => { await rubickServer.close() }, 5000)
}

console.log("The service will close after 5 sec!")
main()
```
