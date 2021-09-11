#!/usr/bin/env zx
import { cd } from 'zx'

// publish rust-backend
cd('packages/rust-backend')
try {
  await $`pnpm publish-platform`
} catch (error) {
  console.error(error)
} finally {
  cd('../../')
}

// publish rubickbase
if (process.platform === 'linux') {
  // build
  await $`pnpm build`

  cd('packages/rubickbase')
  try {
    await $`pnpx ncu -u`
    await $`pnpm publish --access public --no-git-checks`
  } catch (error) {
    console.error(error)
  } finally {
    cd('../../')
  }
}