mod dataprocess;
mod imgtools;
mod ioio;
use neon::prelude::*;
use std::thread;

// 开启键鼠事件侦测
fn ioio_start(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let port = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        ioio::start(port.as_str()).expect("Rpc client start error!");
        channel.send(move |mut _cx| Ok(()))
    });
    Ok(cx.boolean(true))
}

// 主屏幕截图
fn capture_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        imgtools::screen_capture(path).expect("screen capture error");
        channel.send(move |mut _cx| Ok(()))
    });
    Ok(cx.undefined())
}

// 压缩
fn lzma_compress_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let frompath = cx.argument::<JsString>(0)?.value(&mut cx);
    let topath = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        dataprocess::lzma_compress(frompath.as_str(), topath.as_str())
            .expect("lzma_compress error!");
        channel.send(move |mut _cx| Ok(()))
    });
    Ok(cx.undefined())
}

// 解压
fn lzma_decompress_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let frompath = cx.argument::<JsString>(0)?.value(&mut cx);
    let topath = cx.argument::<JsString>(0)?.value(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        dataprocess::lzma_decompress(frompath.as_str(), topath.as_str())
            .expect("lzma_decompress error!");
        channel.send(move |mut _cx| Ok(()))
    });
    Ok(cx.undefined())
}

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

// todo handle error
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // todo async task return object
    cx.export_function("color_picker_start", color_picker_start)?;
    cx.export_function("screen_color_picker_start", screen_color_picker_start)?;
    // async task
    cx.export_function("capture_start", capture_start)?;
    cx.export_function("ioio_start", ioio_start)?;
    cx.export_function("lzma_compress_start", lzma_compress_start)?;
    cx.export_function("lzma_decompress_start", lzma_decompress_start)?;
    Ok(())
}
