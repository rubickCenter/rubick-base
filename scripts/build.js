import fs from 'fs'

if (!fs.existsSync("dist/worker/rust-backend/")) {
    fs.mkdirSync("dist/worker/rust-backend/")
}

fs.copyFileSync('src/worker/rust-backend/index.node', "dist/worker/rust-backend/index.node")