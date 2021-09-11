#!/usr/bin/env zx
import { fs } from 'zx'
const platform = process.platform

let target
let suffix

switch (platform) {
    case 'win32': {
        suffix = '.dll'
        target = 'x86_64-pc-windows-gnu'
        break
    }
    case 'darwin': {
        suffix = '.dylib'
        target = 'x86_64-apple-darwin'
        break
    }
    case 'linux': {
        suffix = '.so'
        target = 'x86_64-unknown-linux-gnu'
        break
    }
}

const targetPath = `target/${target}/release/librubick_backend${suffix}`

await $`npm run build-${platform}`
await fs.copyFile(targetPath, 'index.node')