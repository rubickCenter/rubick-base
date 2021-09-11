#!/usr/bin/env zx
import { fs, cd } from 'zx'

const platform = process.platform
const packagejson = JSON.parse(fs.readFileSync('package.json').toString())

const dist = 'dist'
if (fs.existsSync(dist)) {
	await fs.remove(dist)
}

await fs.mkdirp(dist)
await fs.copy('index.node', `${dist}/index.node`)
await fs.copy('index.d.ts', `${dist}/index.d.ts`)

packagejson['name'] = packagejson['name'] + `-${platform}`
packagejson['os'] = [platform]

await fs.writeJSON(`${dist}/package.json`, packagejson)

cd(dist)

await $`pnpm publish --access public --no-git-checks`
