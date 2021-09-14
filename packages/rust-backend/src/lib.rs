mod imgtools;
mod ioio;
use neon::prelude::*;
use std::thread;

// 获取图片某位置像素颜色
fn color_picker_start(mut cx: FunctionContext) -> JsResult<JsObject> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);
    let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let color = imgtools::color_picker(path, x as u32, y as u32).expect("color pick error");
    let obj = cx.empty_object();
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

// 主屏幕截图
fn capture_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    imgtools::screen_capture(arg0);
    Ok(cx.undefined())
}

// 开启键鼠事件侦测
fn ioio_start(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    thread::spawn(move || ioio::start(arg0.as_str()).expect("Rpc client error!"));
    Ok(cx.boolean(true))
}

// 从屏幕中取色
fn screen_color_picker_start(mut cx: FunctionContext) -> JsResult<JsObject> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let color = imgtools::screen_color_picker(x as u32, y as u32).expect("color picker error!");
    let obj = cx.empty_object();
    let r = cx.number(color[0]);
    let g = cx.number(color[1]);
    let b = cx.number(color[2]);
    obj.set(&mut cx, "r", r)?;
    obj.set(&mut cx, "g", g)?;
    obj.set(&mut cx, "b", b)?;
    Ok(obj)
}

// todo error handling
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("ioio_start", ioio_start)?;
    // todo async task
    cx.export_function("capture_start", capture_start)?;
    cx.export_function("color_picker_start", color_picker_start)?;
    cx.export_function("screen_color_picker_start", screen_color_picker_start)?;
    Ok(())
}
