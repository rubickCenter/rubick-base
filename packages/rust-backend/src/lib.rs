mod imgtools;
mod ioio;
use neon::prelude::*;
use std::thread;

fn color_picker_start(mut cx: FunctionContext) -> JsResult<JsObject> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let obj = cx.empty_object();
    let color = imgtools::color_picker(path, x as u32, y as u32).expect("color pick error");
    let r = cx.number(color[0]);
    let g = cx.number(color[1]);
    let b = cx.number(color[2]);
    let a = cx.number(color[3]);
    obj.set(&mut cx, "r", r)?;
    obj.set(&mut cx, "g", g)?;
    obj.set(&mut cx, "b", b)?;
    obj.set(&mut cx, "a", a)?;
    Ok(obj)
}

fn capture_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    imgtools::screen_capture(arg0);
    Ok(cx.undefined())
}

fn ioio_start(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    thread::spawn(move || ioio::start(arg0.as_str()).expect("Rpc client error!"));
    Ok(cx.boolean(true))
}

// todo error handling
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("ioio_start", ioio_start)?;
    cx.export_function("capture_start", capture_start)?;
    cx.export_function("color_picker_start", color_picker_start)?;
    Ok(())
}
