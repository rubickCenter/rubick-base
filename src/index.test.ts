import RubickServer from "."

const rubickServer = new RubickServer({
  port: 50055
}, {
  listen_event_hook: async (e) => {
    console.log(e)
  }
})

async function main() {
  await rubickServer.start()
  setTimeout(async() => { await rubickServer.close()}, 5000)
}

console.log("The service will close after 5 sec!")
main()
