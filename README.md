# rubickbase

基于 Rust / WASM 提供截图、取色、键鼠事件监听模拟、压缩解压、图像处理、获取已安装应用等跨平台功能的现代异步 Nodejs 模块，占用空间小, 安装便捷, 使用简单, 高性能, 资源占用极小, 可取代 iohook 和 robotjs

## Features

-   [x] 键鼠事件监听钩子 mouse/keyboard event listen
-   [x] 截图 screen capture
-   [x] 图片取色 pixel color picker
-   [x] 获取鼠标位置 get cursor position
-   [x] 获取鼠标像素颜色 pick color at cursor position
-   [x] lzma2 压缩解压 lzma2 (de)compress
-   [x] 注册快捷键事件 register global hotkey
-   [x] 截图获取某位置周围图像 capture the screen around cursor
-   [x] 获取已安装的应用列表(linux✅/macos✅/windows✅) get installed app list
-   [x] 获取已安装应用的详细信息(linux✅) get installed app detail info
-   [x] 键盘事件模拟 keyboard input event simulation
-   [x] 鼠标事件模拟 mouse input event simulation

## Install

与 iohook 与 robotjs 不同, 你不需要针对不同版本进行繁琐的重新编译, 一切开箱即用

无论你是在 node 中还是在 electron 中，都可以用你喜欢的包管理器直接安装:

```
# npm
npm install --save rubickbase

# yarn
yarn add rubickbase

# pnpm
pnpm add rubickbase
```

<details>
<summary>注意事项</summary>

rubickbase 基于 [N-API](https://nodejs.org/api/n-api.html) v6 , 因此 Nodejs 环境推荐以下版本

v10.x ,v12.x ,14.x, 15.x, **16.x**

Electron 环境推荐以下版本

v13.x, **v14.x** ,v15.x ,16.x

</details>

## Getting start

### 引入依赖

rubickbase 支持 cjs 和 esm 两种规范，当然你可以并且推荐在 TypeScript 中使用它

```js
// cjs
const { newRubickBase } = require('rubickbase')
// esm / typescript
import { newRubickBase } from 'rubickbase'
```

### 基本使用

在这个例子中，你通过 `newRubickbase` 获得了 rubickbase 服务实例，并通过 `start` 函数启动了服务，rubickbase 启动后会在后台进行服务侦听，你可以通过 `getAPI` 获取到 rubickbase 所有功能

这里每隔一秒获取当前的鼠标位置，并且 10 秒后调用 `close` 将 rubickbase 服务关闭

```js
// init rubickbase
const rubickBase = newRubickBase()

async function main() {
	// start rubickbase
	await server.start()
	const api = server.getAPI()
	// cursor Position
	let task = setInterval(async () => {
		console.log(await api.getCursorPosition())
	}, 1000)
	// close rubickbase
	setTimeout(async () => {
		await server.close()
		clearInterval(task)
	}, 10000)
}

main()
```

<details>
<summary> 可选初始化参数 </summary>

| 参数名称        | 参数意义               | 类型          |
| --------------- | ---------------------- | ------------- |
| port            | GRPC 服务器的端口      | number        |
| logger          | 日志器                 | Logger        |
| tmpdir          | 临时文件目录           | string        |
| workerBoot      | 是否将 worker 一起启动 | boolean       |
| ioEventCallback | 侦听所有设备事件       | EventCallback |

</details>

<details>
<summary> 分离工作模式 </summary>

rubickbase 由 GRPC 服务器 master 与多个提供不同功能的 worker 组合运行

如果你需要在不同的地方或不同的时间运行他们, 可以依据需要分别启动

首先你需要在 master 启动时选择不启动 workers

```js
// init rubickbase
const rubickBase = newRubickBase({ workerBoot: false })
rubickBase.start()
```

然后在需要的地方手动启动 workers

```js
const rubickWorker = newRubickWorker()
rubickWorker.start()
```

注意, worker 的生命周期(存在时间)必须比 master 要短, 否则 worker 中的 GRPC client 会抛出找不到服务端的异常

并且如果你在启动 master 时更改了端口, 那么也要把端口传递给 worker

```js
// init rubickbase
const rubickBase = newRubickBase({ port: 8001, workerBoot: false })
rubickBase.start()
// then
const rubickWorker = newRubickWorker({ port: 8001 })
rubickWorker.start()
```

</details>

### 设备输入事件模拟

模拟鼠标键盘输入事件非常简单，只要调用 `sendEvent` 即可

由于 rubickbase 是用 TypeScript 书写，书写 Event 时编辑器会自动提示

```js
// 这里将会模拟按下 F1 键
api.sendEvent({
        device: "KeyBoard",
        action: "Press",
        info: "F1"
})

// 这里将会模拟按下鼠标中键
api.sendEvent({
        device: "Mouse",
        action: "Press",
        info: "Middle"
})
```


### 设备输入事件侦听

通过 `setEventChannel` API 创建目标事件频道, 获取对应事件的订阅器

```js
// 这里创建了监听鼠标左键的频道
const { registerHook } = api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
	info: 'Left',
})

// 查看目前所有已创建的事件频道
console.log(api.allEventChannels())

// 通过 `registerHook` 注册打印函数
registerHook('myeventchannel', async (e) => {
	console.log(e)
})

// 删除事件频道
api.delEventChannel('myeventchannel')

console.log(api.hasEventChannel('myeventchannel'), api.allEventChannels())
```

<details>
<summary> 检索和关闭频道 </summary>

`allEventChannels` 可以获得目前所有已存在的事件频道

`hasEventChannel` 可以判断是否有某个名字的频道

`delEventChannel` 可以删除创建的事件频道

</details>

<details>
<summary>TypeScript用法</summary>

**<summary>你可以在 TypeScript 中使用装饰器来进行事件订阅注册**

```ts
// 这里创建了监听鼠标左键的频道
const { register } = api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
	info: 'Left',
})

@register('myeventchannel')
function myCallback(event: DeviceEvent) {
	console.log(event)
}
```

</details>

### 事件模糊匹配

一个设备事件有 `device` `action` `info` 三个约束条件, 你可以去掉其中的任何条件来完成事件模糊匹配

```js
// 匹配鼠标左键的按下事件
api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
	info: 'Left',
})

// 匹配鼠标移动事件
api.setEventChannel({
	device: 'Mouse',
	action: 'Move',
})

// 匹配鼠标所有键的按下事件
api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
})

// 匹配所有设备所有键的按下事件
api.setEventChannel({
	action: 'Press',
})
```

### 图像处理

rubickbase 基于 [Photon](https://silvia-odwyer.github.io/photon/) 的高性能 WASM 模块进行图像处理

1. 取色 Image.colorAt

```js
const color = img.colorAt({ x: 1, y: 1 })
```

2. 缩放 Image.resize

输入宽和高，输出缩放后的图像

```js
const newImg = img.resize(100, 100)
```

<details>
<summary> 可选缩放算法 </summary>

默认最邻近差值算法，其他的算法的图像结果边缘更光滑，可以根据自己的需要进行选择

最邻近差值算法 = 1, 二值寻找算法 = 2, CatmullRom 插值算法 = 3, 高斯算法 = 4, 插值算法 = 5

```js
const newImg = img.resize(100, 100, 1)
```

</details>

### 功能一览

rubickbase 还有以下功能:

1.  lzma 压缩  
    compress: (fromPath: string, toPath: string) => Promise< undefined >

2.  lzma 解压  
    decompress: (fromPath: string, toPath: string) => Promise< undefined >

3.  获取鼠标当前座标  
    getCursorPosition: () => Position

4.  获取鼠标当前座标的像素值  
    getCursorPositionPixelColor: () => Promise< Color >

5.  主屏幕截屏  
    screenCapture: () => Promise< Image >

6.  获取鼠标周围图像  
    screenCaptureAroundPosition: (position: Position, width: number, height: number) => Promise< Image >

7.  获取系统内已安装的应用列表  
    getInstalledApps: (getDetailInfo: boolean = false, extraDirs?: Array< string >) => Promise< string >

    `getDetailInfo` 是否获取应用详细信息 默认否 (目前只有 Linux 有效)  
    `extraDirs` 额外要扫描的目录  
    return JSON 格式的快捷方式路径列表 如果 getDetailInfo 为 true, 那么返回应用详细信息列表

    <details>
    <summary> 应用详细信息字段解释 </summary>

    name: 名称  
    icon_path: 各个尺寸的图标列表  
    description: 应用描述  
    command: 应用启动命令  
    desktop_entry_path: 快捷方式路径

    </details>

    <details>
    <summary> 扫描原理 </summary>

    扫描系统存放快捷方式的目录来获取所有系统内安装的应用, 包含的扫描格式:

    | 平台    | 后辍名       |
    | ------- | ------------ |
    | linux   | desktop      |
    | macos   | app,prefPane |
    | windows | lnk          |

    </details>

## Contribute

欢迎任何形式的贡献与开源协作！

项目依赖 `pnpm` 包管理器, 你需要先安装它

`npm install -g pnpm`

项目采用全自动化的代码检查与构建, 使用以下命令进行开发即可

| Action  | Command          |
| ------- | ---------------- |
| Install | · `pnpm i`       |
| Build   | · `pnpm build`   |
| Commit  | · `pnpm commit`  |
| Release | · `pnpm release` |

## LICENSE

本项目为开源项目, 遵守 MPLv2 协议

如需闭源形式的修改与商用, 请联系我获得授权

## TODO

-   [x] 解决 jimp 依赖占用磁盘、CPU、内存的问题
-   [x] 抽离 utils 函数
-   [x] 解决 hex16 转换 bug
-   [x] 解决截图有黑屏的情况
-   [x] 使用 rust 进行屏幕取色 并防止超出边界
-   [x] 事件订阅模式 event emitter
-   [x] api 调用 async 模式
-   [x] 使用 base64 向 js 传输图片
-   [x] 完善 README 每个功能给个例子
-   [x] 完善文档
-   [x] 使用 consola 作为日志器
-   [ ] 更多图像操作
-   [ ] 压缩解压的回调函数
-   [ ] 对每个 API 进行测试
-   [ ] worker 关闭
