#!/usr/bin/env zx
import { fs } from 'zx'
const platform = process.platform
import { join } from 'path'

let target
let suffix

switch (platform) {
	case 'win32': {
		suffix = '.dll'
		target = 'x86_64-pc-windows-msvc'
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

const targetPath = join('target', target, "release", `${platform === 'win32' ? "" : "lib"}rubick_backend${suffix}`)

await $`pnpm build-${platform}`
await fs.copyFile(targetPath, 'index.node')
