## rubick-grpc-server

rubick 的 API 拓展层基座

## Built-in APIs
 - [x] 键鼠事件监听

## TODO

 - [ ] electron 重编译
 - [ ] 统一 proto 文件
 - [ ] npm 发布

## Getting start

```js
import RubickServer from "rubick-grpc-server"

const rubickServer = new RubickServer({
  port: 50055
}, {
  listen_event_hook: async (e) => {
    console.log(e)
  }
})

async function main() {
  await rubickServer.start()
  setTimeout(async() => { await rubickServer.close()}, 5000)
}

console.log("The service will close after 5 sec!")
main()

```
