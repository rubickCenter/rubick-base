#![feature(async_closure)]
use rdev::listen;
mod devices;
use crate::devices::DeviceEvent;
use chrono::prelude::*;
use rubick_client::rubick_client::RubickClient;
use rubick_client::DeviceEvent as GRPCDE;
use std::sync::mpsc;
use std::thread;
extern crate chrono;
use neon::prelude::*;

pub mod rubick_client {
    tonic::include_proto!("rubick");
}

#[allow(dead_code)]
pub struct Listener {
    timestamp: String,
}

impl Listener {
    fn start_listen<T>(mut hook: T)
    where
        T: FnMut(DeviceEvent) + 'static,
    {
        if let Err(error) = listen(move |event| {
            let device_event = DeviceEvent::receive_from_keyboard_mouse_event(&event);
            hook(device_event);
        }) {
            println!("Error: {:?}", error)
        }
    }

    pub fn new() -> Listener {
        Listener {
            timestamp: Local::now().to_string(),
        }
    }
}

trait Listen {
    fn start(&self, rubick: impl FnMut(DeviceEvent) + 'static);
}

impl Listen for Listener {
    fn start(&self, mut rubick: impl FnMut(DeviceEvent) + 'static) {
        Listener::start_listen(move |event| {
            rubick(event);
        });
    }
}

#[tokio::main]
pub async fn start(port: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let mut client = RubickClient::connect(format!("https://0.0.0.0:{}", port)).await?;

    thread::spawn(|| {
        Listener::new().start(move |event| {
            let request = match event {
                DeviceEvent::KeyBoardEvent(k) => match k {
                    devices::keyboard::KeyBoardEvent::Press(k1) => {
                        if let devices::keyboard::KeyBoardKey::Unknown(k2) = k1 {
                            tonic::Request::new(GRPCDE {
                                device: String::from("KeyBoard"),
                                action: String::from("Press"),
                                info: k2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDE {
                                device: String::from("KeyBoard"),
                                action: String::from("Press"),
                                info: format!("{:?}", k1),
                            })
                        }
                    }
                    devices::keyboard::KeyBoardEvent::Release(k1) => {
                        if let devices::keyboard::KeyBoardKey::Unknown(k2) = k1 {
                            tonic::Request::new(GRPCDE {
                                device: String::from("KeyBoard"),
                                action: String::from("Release"),
                                info: k2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDE {
                                device: String::from("KeyBoard"),
                                action: String::from("Release"),
                                info: format!("{:?}", k1),
                            })
                        }
                    }
                },
                DeviceEvent::MouseEvent(m) => match m {
                    devices::mouse::MouseEvent::Press(m1) => {
                        if let devices::mouse::MouseKey::Unknown(m2) = m1 {
                            tonic::Request::new(GRPCDE {
                                device: String::from("Mouse"),
                                action: String::from("Press"),
                                info: m2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDE {
                                device: String::from("Mouse"),
                                action: String::from("Press"),
                                info: format!("{:?}", m1),
                            })
                        }
                    }
                    devices::mouse::MouseEvent::Rlease(m1) => {
                        if let devices::mouse::MouseKey::Unknown(m2) = m1 {
                            tonic::Request::new(GRPCDE {
                                device: String::from("Mouse"),
                                action: String::from("Rlease"),
                                info: m2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDE {
                                device: String::from("Mouse"),
                                action: String::from("Rlease"),
                                info: format!("{:?}", m1),
                            })
                        }
                    }
                    devices::mouse::MouseEvent::Move(m1) => tonic::Request::new(GRPCDE {
                        device: String::from("Mouse"),
                        action: String::from("Move"),
                        info: m1.to_string(),
                    }),
                    devices::mouse::MouseEvent::Wheel(m1) => tonic::Request::new(GRPCDE {
                        device: String::from("Mouse"),
                        action: String::from("Wheel"),
                        info: format!("{:?}", m1),
                    }),
                },
            };

            tx.send(request).expect("Send error");
        });
    });

    for received in rx {
        client.listen_event(received).await?;
    }

    Ok(())
}

fn js_start(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg0 = cx.argument::<JsString>(0)?.value(&mut cx);
    start(arg0.as_str()).expect("error!");
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("start", js_start)?;
    Ok(())
}
