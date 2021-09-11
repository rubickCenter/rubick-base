#!/usr/bin/env zx
import { cd } from 'zx'

// build
await $`pnpm run build`

// publish rust-backend
cd('packages/rust-backend')
await $`pnpm run publish-platform`
cd('../../')

// publish rubickbase
if (process.platform === 'linux') {
  cd('packages/rubickbase')
  await $`pnpx ncu -u`
  await $`pnpm publish --access=public`
  cd('../../')
}