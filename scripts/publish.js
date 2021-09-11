#!/usr/bin/env zx
import { cd } from 'zx'

// publish rust-backend
cd('packages/rust-backend')
await $`pnpm publish-platform`


// publish rubickbase
if (process.platform === 'linux') {
  // build
  await $`pnpm build`

  cd('packages/rubickbase')
  await $`pnpx ncu -u`
  await $`pnpm publish --access public --no-git-checks`
}