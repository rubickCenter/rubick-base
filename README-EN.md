# rubickbase

Based on Rust / WASM, a modern asynchronous Nodejs module that provides cross-platform functions such as screenshots, color picking, keyboard and mouse event monitoring simulation, image processing, and access to installed applications. It occupies a small space, is easy to install, simple to use, high performance, and consumes very little resources. , Can replace iohook and robotjs

## Features

**Device event listening and simulation**

-   [x] Get mouse position
-   [x] Keyboard and mouse event monitoring
-   [x] Keyboard event simulation
-   [x] Mouse event simulation
-   [x] Subscribe to shortcut key events

**Image and Screen**

-   [x] Screenshot
-   [x] Get mouse pixel color (main screen)
-   [x] Image zoom
-   [x] Picture color selection
-   [x] Picture cropping
-   [x] Multiple screenshots

**System info**

-   [x] Get the list of installed applications (linux✅/macos✅/windows✅)
-   [x] Get detailed information of installed applications (linux✅)
-   [x] Get system language

**Other tools**

-   [x] asar package compression and decompression (zstd algorithm)

## Install

Unlike iohook and robotjs, you don't need to recompile tediously for different versions, everything works out of the box

Whether you are in node or electron, you can install it directly with your favorite package manager:

```
# npm
npm install --save rubickbase

# yarn
yarn add rubickbase

# pnpm
pnpm add rubickbase
```

<details>
<summary>Notes</summary>

rubickbase is based on [N-API](https://nodejs.org/api/n-api.html) v6, so the following versions are recommended for Nodejs environment

v10.x ,v12.x ,14.x, 15.x, **16.x**

Electron environment recommends the following versions

v13.x,v14.x ,**v15.x** ,16.x

</details>

## Quick start

### Introducing dependencies

rubickbase supports both cjs and esm specifications, of course you can and recommend using it in TypeScript

```js
// cjs
const { newRubickBase } = require('rubickbase')
// esm / typescript
import { newRubickBase } from 'rubickbase'
```

### Basic usage

In this example, you get the rubickbase service instance through `newRubickbase`, you can get all the functions of rubickbase through `getAPI`

Here get the current mouse position every second

```js
const { newRubickBase } = require('rubickbase')

// init rubickbase
const rubickBase = newRubickBase()

setInterval(async () => {
	// start rubickbase and get APIs
	const api = await rubickBase.getAPI()
	// print Cursor Position
	console.log(api.getCursorPosition())
}, 1000)
```

<details>
<summary> Optional initialization parameters</summary>

| Parameter name  | Parameter meaning                                   | Type          |
| --------------- | --------------------------------------------------- | ------------- |
| port            | Port of the GRPC server                             | number        |
| logger          | Logger                                              | Logger        |
| tmpdir          | Temporary file directory                            | string        |
| workerBoot      | Whether to start workers together                   | boolean       |
| ioEventCallback | Callback function that listens to all device events | EventCallback |

</details>

<details>
<summary> Advanced startup</summary>

rubickbase is run by a combination of the GRPC server master and multiple workers that provide different functions

Generally speaking, when you call `getAPI`, rubickbase will automatically turn on all services, but if you need to run them in a different place or time, you can manually control their life cycle to achieve more refined control

First of all, you need to choose not to start the workers when the master starts. At this time, the master will listen to messages from the workers.

```js
// init rubickbase
const rubickBase = newRubickBase({ workerBoot: false })
rubickBase.start()
```

Then manually start workers where needed

```js
const rubickWorker = newRubickWorker()
// start all workers
rubickWorker.start()
// Start ioio worker separately
rubickWorker.start('ioio')
```

Note that the life cycle (existence time) of the worker must be shorter than that of the master, otherwise the GRPC client in the worker will throw an exception that the server cannot be found

And if you change the port when starting the master, then pass the port to the worker

```js
// init rubickbase
const rubickBase = newRubickBase({ port: 8001, workerBoot: false })
rubickBase.start()
// then
const rubickWorker = newRubickWorker({ port: 8001 })
rubickWorker.start()
```

</details>

<details>
<summary> Use the underlying stateless API directly </summary>

Allows you to directly call some basic APIs without starting the master and workers

```js
const {
	language,
	sendEvent,
	getInstalledApps,
	screenCapture,
	screenCaptureAll,
	screenCaptureAroundPosition,
} = await newRubickBase().getBasicAPI()
```

</details>

### Device input event simulation

It is very simple to simulate mouse and keyboard input events, just call `sendEvent`

Since rubickbase is written in TypeScript, the editor will automatically prompt when writing Event

```js
// This will simulate pressing the F1 key
api.sendEvent({
	device: 'KeyBoard',
	action: 'Press',
	info: 'F1',
})

// This will simulate pressing the middle mouse button
api.sendEvent({
	device: 'Mouse',
	action: 'Press',
	info: 'Middle',
})
```

### Device input event listening

Create a target event channel through the `setEventChannel` API, and get the subscriber of the corresponding event

```js
// Created here to monitor the left mouse button channel
const register = api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
	info: 'Left',
})

// View all currently created event channels
console.log(api.allEventChannels())

// Register the printing function through `registerHook`
register('myeventchannel', async (e) => {
	console.log(e)
})

// delete event channel
api.delEventChannel('myeventchannel')

console.log(api.hasEventChannel('myeventchannel'), api.allEventChannels())
```

<details>
<summary> Retrieve and close channels</summary>

`allEventChannels` can get all existing event channels

`hasEventChannel` can determine whether there is a channel with a certain name

`delEventChannel` can delete the created event channel

</details>

### Event fuzzy matching

A device event has three constraints: `device` `action` `info`, you can remove any of these conditions to complete event fuzzy matching

```js
// Match the press event of the left mouse button
api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
	info: 'Left',
})

// Match the mouse movement event
api.setEventChannel({
	device: 'Mouse',
	action: 'Move',
})

// Match the press event of all the mouse buttons
api.setEventChannel({
	device: 'Mouse',
	action: 'Press',
})

// Match all key press events of all devices
api.setEventChannel({
	action: 'Press',
})
```

### Image Processing

rubickbase is based on the high-performance WASM module of [Photon](https://silvia-odwyer.github.io/photon/) for image processing

1. Take color Image.colorAt

```js
const color = img.colorAt({ x: 1, y: 1 })
```

2. Resize Image.resize

Input width and height, output scaled image

```js
const newImg = img.resize(100, 100)
```

<details>
<summary> Optional scaling algorithm</summary>

The default nearest neighbor difference algorithm, the image results of other algorithms have smoother edges, you can choose according to your needs

Nearest Neighbor Difference Algorithm = 1, Binary Finding Algorithm = 2, CatmullRom Interpolation Algorithm = 3, Gaussian Algorithm = 4, Interpolation Algorithm = 5

```js
const newImg = img.resize(100, 100, 1)
```

</details>

3. Crop Image.crop

Input the point, width and height of the upper left corner, and output the cropped image

```js
const newImg = img.crop({ x: 5, y: 5 }, 10, 10)
```

### Features at a glance

Rubickbase also has the following features:

1.  Get the current coordinates of the mouse  
    getCursorPosition: () => Position

2.  Get the pixel value of the current coordinates of the mouse  
    _This API is only available for the home screen_  
    getCursorPositionPixelColor: () => Promise< Color>

3.  Main screen screenshot  
    screenCapture: () => Promise< Image>

4.  All screenshots  
    screenCaptureAll: () => Promise< Image[]>

5.  Get the image around the mouse  
    _This API is only available for the home screen_  
    screenCaptureAroundPosition: (position: Position, width: number, height: number) => Promise< Image>

6.  Get the list of installed applications in the system  
    getInstalledApps: (getDetailInfo: boolean = false, extraDirs?: Array< string >) => Promise< string>

    `getDetailInfo` Whether to obtain application detailed information. Default no (currently only available on Linux)  
     `extraDirs` additional directories to be scanned  
    `extraDirs` additional directories to be scanned  
     `extraDirs` additional directories to be scanned  
     Return a list of shortcut paths in JSON format. If getDetailInfo is true, then return a list of application details

        <details>
        <summary> Application details field explanation</summary>

    name: name  
     icon_path: list of icons of various sizes  
    icon_path: list of icons of various sizes  
     icon_path: list of icons of various sizes  
     description: application description  
    description: application description  
     description: application description  
     command: application start command  
    command: application start command  
     command: application start command  
     desktop_entry_path: shortcut path

        </details>

        <details>
        <summary> Scanning principle</summary>

    Scan the directory where the system stores the shortcuts to get all the applications installed in the system, including the scan format:

    | Platform | Suffix       |
    | -------- | ------------ |
    | linux    | desktop      |
    | macos    | app,prefPane |
    | windows  | lnk          |

        </details>

7.  Get system language
    language: () => Promise< string>

8.  asar + zstd compression

    It is a superset of electron's official asar format, and zstd compression algorithm is added when packaging

    asarList(path: string): Promise< Array <string> | undefined>  
    asarExtractFile(path: string, dest: string): Promise< undefined>  
    asarExtract(path: string, dest: string): Promise< undefined>  
    asarPack(path: string, dest: string, level?: number): Promise< undefined>

## Contribution and contact

Any kind of contribution and open source collaboration are welcome!

The project depends on the `pnpm` package manager, you need to install it first

`npm install -g pnpm`

The project adopts fully automated code inspection and construction, and you can use the following commands to develop

| Action  | Command          |
| ------- | ---------------- |
| Install | · `pnpm i`       |
| Build   | · `pnpm build`   |
| Commit  | · `pnpm commit`  |
| Release | · `pnpm release` |

After paying attention to the official account, send the `contact` keyword to add me on WeChat:

![wechat](https://z3.ax1x.com/2021/09/26/4yRpN9.jpg)

## LISENCE

MPLv2
