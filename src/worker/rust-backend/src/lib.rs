#![feature(async_closure)]

mod ioio;
use ioio::start;
use neon::prelude::*;
use std::thread;

fn ioio_start(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    thread::spawn(move || start(arg0.as_str()).expect("Rpc client error!"));
    Ok(cx.boolean(true))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("ioio_start", ioio_start)?;
    Ok(())
}
