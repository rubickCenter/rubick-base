#![allow(dead_code)]
pub mod devices;

pub mod grpc_client {
    tonic::include_proto!("rubick");
}

extern crate chrono;
use chrono::prelude::*;
use devices::DeviceEvent;
use grpc_client::rubick_client::RubickClient;
use grpc_client::DeviceEvent as GRPCDeviceEvent;
use rdev::listen;
use std::sync::mpsc;
use std::thread;
use tonic::transport::Channel;

use crate::ioio::devices::mouse::{MouseEvent, MouseKey, MouseMove};

use self::devices::{
    keyboard::{KeyBoardEvent, KeyBoardKey},
    mouse::MouseWheel,
};

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
    #[allow(dead_code)]
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

// listen device send grpc event
async fn send_event(client: &mut RubickClient<Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(|| {
        Listener::new().start(move |event| {
            let request = match event {
                DeviceEvent::KeyBoardEvent(k) => match k {
                    devices::keyboard::KeyBoardEvent::Press(k1) => {
                        if let devices::keyboard::KeyBoardKey::Unknown(k2) = k1 {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("KeyBoard"),
                                action: String::from("Press"),
                                info: k2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("KeyBoard"),
                                action: String::from("Press"),
                                info: format!("{:?}", k1),
                            })
                        }
                    }
                    devices::keyboard::KeyBoardEvent::Release(k1) => {
                        if let devices::keyboard::KeyBoardKey::Unknown(k2) = k1 {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("KeyBoard"),
                                action: String::from("Release"),
                                info: k2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDeviceEvent {
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
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("Mouse"),
                                action: String::from("Press"),
                                info: m2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("Mouse"),
                                action: String::from("Press"),
                                info: format!("{:?}", m1),
                            })
                        }
                    }
                    devices::mouse::MouseEvent::Rlease(m1) => {
                        if let devices::mouse::MouseKey::Unknown(m2) = m1 {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("Mouse"),
                                action: String::from("Rlease"),
                                info: m2.to_string(),
                            })
                        } else {
                            tonic::Request::new(GRPCDeviceEvent {
                                device: String::from("Mouse"),
                                action: String::from("Rlease"),
                                info: format!("{:?}", m1),
                            })
                        }
                    }
                    devices::mouse::MouseEvent::Move(m1) => tonic::Request::new(GRPCDeviceEvent {
                        device: String::from("Mouse"),
                        action: String::from("Move"),
                        info: m1.to_string(),
                    }),
                    devices::mouse::MouseEvent::Wheel(m1) => tonic::Request::new(GRPCDeviceEvent {
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
        client.ioio(received).await?;
    }
    Ok(())
}

// start grpc client
#[tokio::main]
pub async fn start(port: &str) -> Result<RubickClient<Channel>, Box<dyn std::error::Error>> {
    let mut client = RubickClient::connect(format!("https://127.0.0.1:{}", port)).await?;
    send_event(&mut client).await?;
    Ok(client)
}

#[allow(dead_code)]
pub enum Info {
    Button(String),
    UnknownButton(f64),
    Position { x: f64, y: f64 },
}

#[allow(dead_code)]
pub fn send(device: &str, action: &str, info: &Info) {
    let event = match device {
        "KeyBoard" => match action {
            "Press" => match info {
                Info::Button(k) => match k.as_str() {
                    "Alt" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Alt,
                    ))),
                    "AltGr" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::AltGr,
                    ))),
                    "Backspace" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Backspace,
                    ))),
                    "CapsLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::CapsLock,
                    ))),
                    "ControlLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::ControlLeft,
                    ))),
                    "ControlRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::ControlRight,
                    ))),
                    "Delete" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Delete,
                    ))),
                    "DownArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::DownArrow,
                    ))),
                    "End" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::End,
                    ))),
                    "Escape" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Escape,
                    ))),
                    "F1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F1,
                    ))),
                    "F10" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F10,
                    ))),
                    "F11" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F11,
                    ))),
                    "F12" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F12,
                    ))),
                    "F2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F2,
                    ))),
                    "F3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F3,
                    ))),
                    "F4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F4,
                    ))),
                    "F5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F5,
                    ))),
                    "F6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F6,
                    ))),
                    "F7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F7,
                    ))),
                    "F8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F8,
                    ))),
                    "F9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::F9,
                    ))),
                    "Home" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Home,
                    ))),
                    "LeftArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::LeftArrow,
                    ))),
                    "MetaLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::MetaLeft,
                    ))),
                    "MetaRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::MetaRight,
                    ))),
                    "PageDown" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::PageDown,
                    ))),
                    "PageUp" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::PageUp,
                    ))),
                    "Return" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Return,
                    ))),
                    "RightArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::RightArrow,
                    ))),
                    "ShiftLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::ShiftLeft,
                    ))),
                    "ShiftRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::ShiftRight,
                    ))),
                    "Space" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Space,
                    ))),
                    "Tab" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Tab,
                    ))),
                    "UpArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::UpArrow,
                    ))),
                    "PrintScreen" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::PrintScreen,
                    ))),
                    "ScrollLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::ScrollLock,
                    ))),
                    "Pause" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Pause,
                    ))),
                    "NumLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::NumLock,
                    ))),
                    "BackQuote" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::BackQuote,
                    ))),
                    "Num1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num1,
                    ))),
                    "Num2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num2,
                    ))),
                    "Num3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num3,
                    ))),
                    "Num4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num4,
                    ))),
                    "Num5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num5,
                    ))),
                    "Num6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num6,
                    ))),
                    "Num7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num7,
                    ))),
                    "Num8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num8,
                    ))),
                    "Num9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num9,
                    ))),
                    "Num0" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Num0,
                    ))),
                    "Minus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Minus,
                    ))),
                    "Equal" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Equal,
                    ))),
                    "KeyQ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyQ,
                    ))),
                    "KeyW" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyW,
                    ))),
                    "KeyE" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyE,
                    ))),
                    "KeyR" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyR,
                    ))),
                    "KeyT" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyT,
                    ))),
                    "KeyY" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyY,
                    ))),
                    "KeyU" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyU,
                    ))),
                    "KeyI" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyI,
                    ))),
                    "KeyO" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyO,
                    ))),
                    "KeyP" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyP,
                    ))),
                    "LeftBracket" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::LeftBracket,
                    ))),
                    "RightBracket" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::RightBracket,
                    ))),
                    "KeyA" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyA,
                    ))),
                    "KeyS" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyS,
                    ))),
                    "KeyD" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyD,
                    ))),
                    "KeyF" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyF,
                    ))),
                    "KeyG" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyG,
                    ))),
                    "KeyH" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyH,
                    ))),
                    "KeyJ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyJ,
                    ))),
                    "KeyK" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyK,
                    ))),
                    "KeyL" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyL,
                    ))),
                    "SemiColon" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::SemiColon,
                    ))),
                    "Quote" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Quote,
                    ))),
                    "BackSlash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::BackSlash,
                    ))),
                    "IntlBackslash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::IntlBackslash,
                    ))),
                    "KeyZ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyZ,
                    ))),
                    "KeyX" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyX,
                    ))),
                    "KeyC" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyC,
                    ))),
                    "KeyV" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyV,
                    ))),
                    "KeyB" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyB,
                    ))),
                    "KeyN" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyN,
                    ))),
                    "KeyM" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KeyM,
                    ))),
                    "Comma" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Comma,
                    ))),
                    "Dot" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Dot,
                    ))),
                    "Slash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Slash,
                    ))),
                    "Insert" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Insert,
                    ))),
                    "KpReturn" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpReturn,
                    ))),
                    "KpMinus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpMinus,
                    ))),
                    "KpPlus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpPlus,
                    ))),
                    "KpMultiply" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpMultiply,
                    ))),
                    "KpDivide" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpDivide,
                    ))),
                    "Kp0" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp0,
                    ))),
                    "Kp1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp1,
                    ))),
                    "Kp2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp2,
                    ))),
                    "Kp3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp3,
                    ))),
                    "Kp4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp4,
                    ))),
                    "Kp5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp5,
                    ))),
                    "Kp6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp6,
                    ))),
                    "Kp7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp7,
                    ))),
                    "Kp8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp8,
                    ))),
                    "Kp9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Kp9,
                    ))),
                    "KpDelete" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::KpDelete,
                    ))),
                    "Function" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                        KeyBoardKey::Function,
                    ))),
                    key => {
                        println!("No such key {:?}", key);
                        None
                    }
                },
                Info::UnknownButton(b) => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(
                    KeyBoardKey::Unknown(*b),
                ))),
                Info::Position { x: _, y: _ } => {
                    println!("No such action!");
                    None
                }
            },
            "Release" => match info {
                Info::Button(k) => match k.as_str() {
                    "Alt" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Alt,
                    ))),
                    "AltGr" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::AltGr,
                    ))),
                    "Backspace" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Backspace,
                    ))),
                    "CapsLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::CapsLock,
                    ))),
                    "ControlLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::ControlLeft,
                    ))),
                    "ControlRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::ControlRight,
                    ))),
                    "Delete" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Delete,
                    ))),
                    "DownArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::DownArrow,
                    ))),
                    "End" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::End,
                    ))),
                    "Escape" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Escape,
                    ))),
                    "F1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F1,
                    ))),
                    "F10" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F10,
                    ))),
                    "F11" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F11,
                    ))),
                    "F12" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F12,
                    ))),
                    "F2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F2,
                    ))),
                    "F3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F3,
                    ))),
                    "F4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F4,
                    ))),
                    "F5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F5,
                    ))),
                    "F6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F6,
                    ))),
                    "F7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F7,
                    ))),
                    "F8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F8,
                    ))),
                    "F9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::F9,
                    ))),
                    "Home" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Home,
                    ))),
                    "LeftArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::LeftArrow,
                    ))),
                    "MetaLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::MetaLeft,
                    ))),
                    "MetaRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::MetaRight,
                    ))),
                    "PageDown" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::PageDown,
                    ))),
                    "PageUp" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::PageUp,
                    ))),
                    "Return" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Return,
                    ))),
                    "RightArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::RightArrow,
                    ))),
                    "ShiftLeft" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::ShiftLeft,
                    ))),
                    "ShiftRight" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::ShiftRight,
                    ))),
                    "Space" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Space,
                    ))),
                    "Tab" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Tab,
                    ))),
                    "UpArrow" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::UpArrow,
                    ))),
                    "PrintScreen" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::PrintScreen,
                    ))),
                    "ScrollLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::ScrollLock,
                    ))),
                    "Pause" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Pause,
                    ))),
                    "NumLock" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::NumLock,
                    ))),
                    "BackQuote" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::BackQuote,
                    ))),
                    "Num1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num1,
                    ))),
                    "Num2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num2,
                    ))),
                    "Num3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num3,
                    ))),
                    "Num4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num4,
                    ))),
                    "Num5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num5,
                    ))),
                    "Num6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num6,
                    ))),
                    "Num7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num7,
                    ))),
                    "Num8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num8,
                    ))),
                    "Num9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num9,
                    ))),
                    "Num0" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Num0,
                    ))),
                    "Minus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Minus,
                    ))),
                    "Equal" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Equal,
                    ))),
                    "KeyQ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyQ,
                    ))),
                    "KeyW" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyW,
                    ))),
                    "KeyE" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyE,
                    ))),
                    "KeyR" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyR,
                    ))),
                    "KeyT" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyT,
                    ))),
                    "KeyY" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyY,
                    ))),
                    "KeyU" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyU,
                    ))),
                    "KeyI" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyI,
                    ))),
                    "KeyO" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyO,
                    ))),
                    "KeyP" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyP,
                    ))),
                    "LeftBracket" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::LeftBracket,
                    ))),
                    "RightBracket" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::RightBracket,
                    ))),
                    "KeyA" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyA,
                    ))),
                    "KeyS" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyS,
                    ))),
                    "KeyD" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyD,
                    ))),
                    "KeyF" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyF,
                    ))),
                    "KeyG" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyG,
                    ))),
                    "KeyH" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyH,
                    ))),
                    "KeyJ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyJ,
                    ))),
                    "KeyK" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyK,
                    ))),
                    "KeyL" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyL,
                    ))),
                    "SemiColon" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::SemiColon,
                    ))),
                    "Quote" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Quote,
                    ))),
                    "BackSlash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::BackSlash,
                    ))),
                    "IntlBackslash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::IntlBackslash,
                    ))),
                    "KeyZ" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyZ,
                    ))),
                    "KeyX" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyX,
                    ))),
                    "KeyC" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyC,
                    ))),
                    "KeyV" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyV,
                    ))),
                    "KeyB" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyB,
                    ))),
                    "KeyN" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyN,
                    ))),
                    "KeyM" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KeyM,
                    ))),
                    "Comma" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Comma,
                    ))),
                    "Dot" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Dot,
                    ))),
                    "Slash" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Slash,
                    ))),
                    "Insert" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Insert,
                    ))),
                    "KpReturn" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpReturn,
                    ))),
                    "KpMinus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpMinus,
                    ))),
                    "KpPlus" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpPlus,
                    ))),
                    "KpMultiply" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpMultiply,
                    ))),
                    "KpDivide" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpDivide,
                    ))),
                    "Kp0" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp0,
                    ))),
                    "Kp1" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp1,
                    ))),
                    "Kp2" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp2,
                    ))),
                    "Kp3" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp3,
                    ))),
                    "Kp4" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp4,
                    ))),
                    "Kp5" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp5,
                    ))),
                    "Kp6" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp6,
                    ))),
                    "Kp7" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp7,
                    ))),
                    "Kp8" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp8,
                    ))),
                    "Kp9" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Kp9,
                    ))),
                    "KpDelete" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::KpDelete,
                    ))),
                    "Function" => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                        KeyBoardKey::Function,
                    ))),
                    key => {
                        println!("No such key {:?}", key);
                        None
                    }
                },
                Info::UnknownButton(b) => Some(DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                    KeyBoardKey::Unknown(*b),
                ))),
                Info::Position { x: _, y: _ } => {
                    println!("No such action!");
                    None
                }
            },
            name => {
                println!("No such action {:?}", name);
                None
            }
        },
        "Mouse" => match action {
            "Press" => match info {
                Info::Button(b) => match b.as_str() {
                    "Left" => Some(DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Left))),
                    "Right" => Some(DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Right))),
                    "Middle" => Some(DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Middle))),
                    name => {
                        println!("No such button {:?}", name);
                        None
                    }
                },
                Info::UnknownButton(b) => Some(DeviceEvent::MouseEvent(MouseEvent::Press(
                    MouseKey::Unknown(*b),
                ))),
                Info::Position { x: _, y: _ } => {
                    println!("No such action!");
                    None
                }
            },
            "Release" => match info {
                Info::Button(b) => match b.as_str() {
                    "Left" => Some(DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Left))),
                    "Right" => Some(DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Right))),
                    "Middle" => Some(DeviceEvent::MouseEvent(MouseEvent::Rlease(
                        MouseKey::Middle,
                    ))),
                    name => {
                        println!("No such button {:?}", name);
                        None
                    }
                },
                Info::UnknownButton(b) => Some(DeviceEvent::MouseEvent(MouseEvent::Rlease(
                    MouseKey::Unknown(*b),
                ))),
                Info::Position { x: _, y: _ } => {
                    println!("No such action!");
                    None
                }
            },
            "Move" => match info {
                Info::Button(_) => {
                    println!("No such action!");
                    None
                }
                Info::UnknownButton(_) => {
                    println!("No such action!");
                    None
                }
                Info::Position { x, y } => {
                    Some(DeviceEvent::MouseEvent(MouseEvent::Move(MouseMove {
                        x: *x,
                        y: *y,
                    })))
                }
            },
            "Wheel" => match info {
                Info::Button(b) => match b.as_str() {
                    "Up" => Some(DeviceEvent::MouseEvent(MouseEvent::Wheel(MouseWheel::Up))),
                    "Down" => Some(DeviceEvent::MouseEvent(MouseEvent::Wheel(MouseWheel::Down))),
                    name => {
                        println!("No such button {:?}", name);
                        None
                    }
                },
                Info::UnknownButton(_) => {
                    println!("No such action!");
                    None
                }
                Info::Position { x: _, y: _ } => {
                    println!("No such action!");
                    None
                }
            },
            name => {
                println!("No such action {:?}", name);
                None
            }
        },
        name => {
            println!("No such device {:?}", name);
            None
        }
    };
    if let Some(event) = event {
        event.send_keyboard_mouse_event()
    }
}
