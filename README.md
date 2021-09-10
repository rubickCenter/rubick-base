## rubickbase

基于 Rust 提供原生能力 API 的现代 Nodejs 模块，大小 2.2M，使用简单，可取代 iohook 和 robotjs

## Install

npm install --save rubickbase

## Built-in APIs

 - [x] 键鼠事件监听钩子
 - [x] 截图
 - [x] 图片取色
 - [x] 获取鼠标位置
 - [x] 获取鼠标像素颜色
 - [ ] 注册快捷键(装饰器风格)

## Getting start

```js
// cjs
const { newRubickBase } = require('rubickbase')
// esm
import { newRubickBase } from 'rubickbase'

const rubickBase = newRubickBase()

async function main() {
    // start rubickbase
    await server.start()
    const api = server.getAPI()
    // screen capture
    await api.screenCapture("./capture.png")
    // cursor Position
    let task = setInterval(async () => {
        console.log(await api.getCursorPositionPixelColor())
    }, 1000)
    // close rubickbase
    setTimeout(async () => {
        await server.close()
        clearInterval(task)
    }, 10000)
}

main()
```
