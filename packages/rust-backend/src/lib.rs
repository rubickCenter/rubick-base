#![feature(path_try_exists)]
mod dataprocess;
mod imgtools;
mod ioio;
mod sysapp;
use neon::prelude::*;
use std::thread;
use sys_locale::get_locale;

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

// 主屏幕截图 base64
fn capture_base64_start(mut cx: FunctionContext) -> JsResult<JsString> {
    let res = imgtools::screen_capture_base64().expect("screen capture error");
    Ok(cx.string(res))
}

// 压缩
fn lzma_compress_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let frompath = cx.argument::<JsString>(0)?.value(&mut cx);
    let topath = cx.argument::<JsString>(1)?.value(&mut cx);
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
    let topath = cx.argument::<JsString>(1)?.value(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        dataprocess::lzma_decompress(frompath.as_str(), topath.as_str())
            .expect("lzma_decompress error!");
        channel.send(move |mut _cx| Ok(()))
    });
    Ok(cx.undefined())
}

// 获取屏幕矩形区域截图 base64
fn screen_capture_rect_base64_start(mut cx: FunctionContext) -> JsResult<JsString> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let width = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let height = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let res = imgtools::screen_capture_rect_base64(x as u32, y as u32, width as u32, height as u32)
        .expect("screen capture rect error");
    Ok(cx.string(res))
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

// 获取已安装的系统应用 输出JSON格式
fn find_apps_start(mut cx: FunctionContext) -> JsResult<JsString> {
    let detail_json = cx.argument::<JsBoolean>(0)?.value(&mut cx);
    let extra_dirs = if let Some(extra_dirs) = cx.argument_opt(1) {
        let dirs: Handle<JsArray> = extra_dirs.downcast_or_throw(&mut cx)?;
        let dirs: Vec<String> = dirs
            .to_vec(&mut cx)?
            .into_iter()
            .map(|dir| {
                let dir: Handle<JsString> = dir.downcast_or_throw(&mut cx).unwrap();
                dir.value(&mut cx)
            })
            .collect();
        Some(dirs)
    } else {
        None
    };

    let res = sysapp::find_apps(detail_json, extra_dirs);
    let apps = cx.string(serde_json::to_string(&res).unwrap());
    Ok(apps)
}

// 模拟输入
fn send_event_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let device = cx.argument::<JsString>(0)?.value(&mut cx);
    let action = cx.argument::<JsString>(1)?.value(&mut cx);
    let info = cx.argument::<JsValue>(2)?;
    let send_info = if let Ok(button) = info.downcast::<JsString, CallContext<JsObject>>(&mut cx) {
        ioio::Info::Button(button.value(&mut cx))
    } else {
        if let Ok(unknow_button) = info.downcast::<JsNumber, CallContext<JsObject>>(&mut cx) {
            ioio::Info::UnknownButton(unknow_button.value(&mut cx))
        } else {
            let position: Handle<JsObject> = info.downcast_or_throw(&mut cx).unwrap();
            let x = position
                .get(&mut cx, "x")?
                .downcast_or_throw::<JsNumber, CallContext<JsObject>>(&mut cx)?
                .value(&mut cx);
            let y = position
                .get(&mut cx, "y")?
                .downcast_or_throw::<JsNumber, CallContext<JsObject>>(&mut cx)?
                .value(&mut cx);
            ioio::Info::Position { x, y }
        }
    };
    ioio::send(device.as_str(), action.as_str(), &send_info);
    Ok(cx.undefined())
}

fn current_locale_language(mut cx: FunctionContext) -> JsResult<JsString> {
    let current_locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    Ok(cx.string(current_locale))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // async task
    cx.export_function("current_locale_language", current_locale_language)?;
    cx.export_function("send_event_start", send_event_start)?;
    cx.export_function("find_apps_start", find_apps_start)?;
    cx.export_function("screen_color_picker_start", screen_color_picker_start)?;
    cx.export_function("capture_base64_start", capture_base64_start)?;
    cx.export_function(
        "screen_capture_rect_base64_start",
        screen_capture_rect_base64_start,
    )?;
    // mutithread task
    cx.export_function("ioio_start", ioio_start)?;
    cx.export_function("lzma_compress_start", lzma_compress_start)?;
    cx.export_function("lzma_decompress_start", lzma_decompress_start)?;
    // Deprecated
    // cx.export_function("screen_capture_rect_start", screen_capture_rect_start)?;
    // cx.export_function("color_picker_start", color_picker_start)?;
    // cx.export_function("capture_start", capture_start)?;
    Ok(())
}

// Deprecated
// 获取图片某位置像素颜色
// fn color_picker_start(mut cx: FunctionContext) -> JsResult<JsObject> {
//     let path = cx.argument::<JsString>(0)?.value(&mut cx);
//     let x = cx.argument::<JsNumber>(1)?.value(&mut cx);
//     let y = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     let color = imgtools::color_picker(path, x as u32, y as u32).expect("color pick error");
//     let obj = cx.empty_object();
//     let r = cx.number(color[0]);
//     let g = cx.number(color[1]);
//     let b = cx.number(color[2]);
//     let a = cx.number(color[3]);
//     obj.set(&mut cx, "r", r)?;
//     obj.set(&mut cx, "g", g)?;
//     obj.set(&mut cx, "b", b)?;
//     obj.set(&mut cx, "a", a)?;
//     Ok(obj)
// }

// 获取屏幕矩形区域截图
// fn screen_capture_rect_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
//     let y = cx.argument::<JsNumber>(1)?.value(&mut cx);
//     let width = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     let height = cx.argument::<JsNumber>(3)?.value(&mut cx);
//     let path = cx.argument::<JsString>(4)?.value(&mut cx);
//     imgtools::screen_capture_rect(x as u32, y as u32, width as u32, height as u32, path)
//         .expect("screen capture rect error");
//     Ok(cx.undefined())
// }

// 主屏幕截图
// fn capture_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let path = cx.argument::<JsString>(0)?.value(&mut cx);
//     imgtools::screen_capture(path).expect("screen capture error");
//     Ok(cx.undefined())
// }
