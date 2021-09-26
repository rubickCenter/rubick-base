#![allow(dead_code)]

pub mod keyboard;
pub mod mouse;
use keyboard::{KeyBoardEvent, KeyBoardKey};
use mouse::{MouseEvent, MouseKey, MouseMove, MouseWheel};
use rdev::{simulate, Button, Event, EventType, Key};

fn send(event: &EventType) {
    // let delay = time::Duration::from_millis(20);
    if let Err(_) = simulate(event) {
        println!("We could not send {:?}", event);
    }
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}

#[derive(Debug)]
pub enum DeviceEvent {
    KeyBoardEvent(keyboard::KeyBoardEvent),
    MouseEvent(mouse::MouseEvent),
}

impl DeviceEvent {
    pub fn receive_from_keyboard_mouse_event(event: &Event) -> Self {
        let device_event = match event.event_type {
            // keyboard
            rdev::EventType::KeyPress(key) => match key {
                rdev::Key::Alt => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Alt))
                }
                rdev::Key::AltGr => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::AltGr))
                }
                rdev::Key::Backspace => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Backspace))
                }
                rdev::Key::CapsLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::CapsLock))
                }
                rdev::Key::ControlLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::ControlLeft))
                }
                rdev::Key::ControlRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::ControlRight))
                }
                rdev::Key::Delete => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Delete))
                }
                rdev::Key::DownArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::DownArrow))
                }
                rdev::Key::End => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::End))
                }
                rdev::Key::Escape => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Escape))
                }
                rdev::Key::F1 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F1)),
                rdev::Key::F10 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F10))
                }
                rdev::Key::F11 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F11))
                }
                rdev::Key::F12 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F12))
                }
                rdev::Key::F2 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F2)),
                rdev::Key::F3 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F3)),
                rdev::Key::F4 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F4)),
                rdev::Key::F5 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F5)),
                rdev::Key::F6 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F6)),
                rdev::Key::F7 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F7)),
                rdev::Key::F8 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F8)),
                rdev::Key::F9 => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::F9)),
                rdev::Key::Home => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Home))
                }
                rdev::Key::LeftArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::LeftArrow))
                }
                rdev::Key::MetaLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::MetaLeft))
                }
                rdev::Key::MetaRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::MetaRight))
                }
                rdev::Key::PageDown => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::PageDown))
                }
                rdev::Key::PageUp => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::PageUp))
                }
                rdev::Key::Return => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Return))
                }
                rdev::Key::RightArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::RightArrow))
                }
                rdev::Key::ShiftLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::ShiftLeft))
                }
                rdev::Key::ShiftRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::ShiftRight))
                }
                rdev::Key::Space => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Space))
                }
                rdev::Key::Tab => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Tab))
                }
                rdev::Key::UpArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::UpArrow))
                }
                rdev::Key::PrintScreen => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::PrintScreen))
                }
                rdev::Key::ScrollLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::ScrollLock))
                }
                rdev::Key::Pause => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Pause))
                }
                rdev::Key::NumLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::NumLock))
                }
                rdev::Key::BackQuote => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::BackQuote))
                }
                rdev::Key::Num1 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num1))
                }
                rdev::Key::Num2 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num2))
                }
                rdev::Key::Num3 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num3))
                }
                rdev::Key::Num4 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num4))
                }
                rdev::Key::Num5 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num5))
                }
                rdev::Key::Num6 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num6))
                }
                rdev::Key::Num7 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num7))
                }
                rdev::Key::Num8 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num8))
                }
                rdev::Key::Num9 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num9))
                }
                rdev::Key::Num0 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Num0))
                }
                rdev::Key::Minus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Minus))
                }
                rdev::Key::Equal => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Equal))
                }
                rdev::Key::KeyQ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyQ))
                }
                rdev::Key::KeyW => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyW))
                }
                rdev::Key::KeyE => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyE))
                }
                rdev::Key::KeyR => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyR))
                }
                rdev::Key::KeyT => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyT))
                }
                rdev::Key::KeyY => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyY))
                }
                rdev::Key::KeyU => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyU))
                }
                rdev::Key::KeyI => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyI))
                }
                rdev::Key::KeyO => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyO))
                }
                rdev::Key::KeyP => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyP))
                }
                rdev::Key::LeftBracket => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::LeftBracket))
                }
                rdev::Key::RightBracket => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::RightBracket))
                }
                rdev::Key::KeyA => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyA))
                }
                rdev::Key::KeyS => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyS))
                }
                rdev::Key::KeyD => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyD))
                }
                rdev::Key::KeyF => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyF))
                }
                rdev::Key::KeyG => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyG))
                }
                rdev::Key::KeyH => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyH))
                }
                rdev::Key::KeyJ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyJ))
                }
                rdev::Key::KeyK => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyK))
                }
                rdev::Key::KeyL => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyL))
                }
                rdev::Key::SemiColon => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::SemiColon))
                }
                rdev::Key::Quote => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Quote))
                }
                rdev::Key::BackSlash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::BackSlash))
                }
                rdev::Key::IntlBackslash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::IntlBackslash))
                }
                rdev::Key::KeyZ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyZ))
                }
                rdev::Key::KeyX => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyX))
                }
                rdev::Key::KeyC => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyC))
                }
                rdev::Key::KeyV => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyV))
                }
                rdev::Key::KeyB => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyB))
                }
                rdev::Key::KeyN => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyN))
                }
                rdev::Key::KeyM => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KeyM))
                }
                rdev::Key::Comma => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Comma))
                }
                rdev::Key::Dot => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Dot))
                }
                rdev::Key::Slash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Slash))
                }
                rdev::Key::Insert => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Insert))
                }
                rdev::Key::KpReturn => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpReturn))
                }
                rdev::Key::KpMinus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpMinus))
                }
                rdev::Key::KpPlus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpPlus))
                }
                rdev::Key::KpMultiply => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpMultiply))
                }
                rdev::Key::KpDivide => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpDivide))
                }
                rdev::Key::Kp0 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp0))
                }
                rdev::Key::Kp1 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp1))
                }
                rdev::Key::Kp2 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp2))
                }
                rdev::Key::Kp3 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp3))
                }
                rdev::Key::Kp4 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp4))
                }
                rdev::Key::Kp5 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp5))
                }
                rdev::Key::Kp6 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp6))
                }
                rdev::Key::Kp7 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp7))
                }
                rdev::Key::Kp8 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp8))
                }
                rdev::Key::Kp9 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Kp9))
                }
                rdev::Key::KpDelete => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::KpDelete))
                }
                rdev::Key::Function => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Function))
                }
                rdev::Key::Unknown(b) => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Press(KeyBoardKey::Unknown(b.into())))
                }
            },
            rdev::EventType::KeyRelease(key) => match key {
                rdev::Key::Alt => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Alt))
                }
                rdev::Key::AltGr => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::AltGr))
                }
                rdev::Key::Backspace => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Backspace))
                }
                rdev::Key::CapsLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::CapsLock))
                }
                rdev::Key::ControlLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::ControlLeft))
                }
                rdev::Key::ControlRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::ControlRight))
                }
                rdev::Key::Delete => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Delete))
                }
                rdev::Key::DownArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::DownArrow))
                }
                rdev::Key::End => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::End))
                }
                rdev::Key::Escape => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Escape))
                }
                rdev::Key::F1 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F1))
                }
                rdev::Key::F10 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F10))
                }
                rdev::Key::F11 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F11))
                }
                rdev::Key::F12 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F12))
                }
                rdev::Key::F2 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F2))
                }
                rdev::Key::F3 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F3))
                }
                rdev::Key::F4 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F4))
                }
                rdev::Key::F5 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F5))
                }
                rdev::Key::F6 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F6))
                }
                rdev::Key::F7 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F7))
                }
                rdev::Key::F8 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F8))
                }
                rdev::Key::F9 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::F9))
                }
                rdev::Key::Home => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Home))
                }
                rdev::Key::LeftArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::LeftArrow))
                }
                rdev::Key::MetaLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::MetaLeft))
                }
                rdev::Key::MetaRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::MetaRight))
                }
                rdev::Key::PageDown => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::PageDown))
                }
                rdev::Key::PageUp => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::PageUp))
                }
                rdev::Key::Return => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Return))
                }
                rdev::Key::RightArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::RightArrow))
                }
                rdev::Key::ShiftLeft => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::ShiftLeft))
                }
                rdev::Key::ShiftRight => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::ShiftRight))
                }
                rdev::Key::Space => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Space))
                }
                rdev::Key::Tab => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Tab))
                }
                rdev::Key::UpArrow => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::UpArrow))
                }
                rdev::Key::PrintScreen => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::PrintScreen))
                }
                rdev::Key::ScrollLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::ScrollLock))
                }
                rdev::Key::Pause => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Pause))
                }
                rdev::Key::NumLock => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::NumLock))
                }
                rdev::Key::BackQuote => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::BackQuote))
                }
                rdev::Key::Num1 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num1))
                }
                rdev::Key::Num2 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num2))
                }
                rdev::Key::Num3 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num3))
                }
                rdev::Key::Num4 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num4))
                }
                rdev::Key::Num5 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num5))
                }
                rdev::Key::Num6 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num6))
                }
                rdev::Key::Num7 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num7))
                }
                rdev::Key::Num8 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num8))
                }
                rdev::Key::Num9 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num9))
                }
                rdev::Key::Num0 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Num0))
                }
                rdev::Key::Minus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Minus))
                }
                rdev::Key::Equal => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Equal))
                }
                rdev::Key::KeyQ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyQ))
                }
                rdev::Key::KeyW => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyW))
                }
                rdev::Key::KeyE => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyE))
                }
                rdev::Key::KeyR => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyR))
                }
                rdev::Key::KeyT => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyT))
                }
                rdev::Key::KeyY => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyY))
                }
                rdev::Key::KeyU => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyU))
                }
                rdev::Key::KeyI => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyI))
                }
                rdev::Key::KeyO => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyO))
                }
                rdev::Key::KeyP => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyP))
                }
                rdev::Key::LeftBracket => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::LeftBracket))
                }
                rdev::Key::RightBracket => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::RightBracket))
                }
                rdev::Key::KeyA => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyA))
                }
                rdev::Key::KeyS => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyS))
                }
                rdev::Key::KeyD => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyD))
                }
                rdev::Key::KeyF => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyF))
                }
                rdev::Key::KeyG => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyG))
                }
                rdev::Key::KeyH => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyH))
                }
                rdev::Key::KeyJ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyJ))
                }
                rdev::Key::KeyK => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyK))
                }
                rdev::Key::KeyL => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyL))
                }
                rdev::Key::SemiColon => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::SemiColon))
                }
                rdev::Key::Quote => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Quote))
                }
                rdev::Key::BackSlash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::BackSlash))
                }
                rdev::Key::IntlBackslash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::IntlBackslash))
                }
                rdev::Key::KeyZ => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyZ))
                }
                rdev::Key::KeyX => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyX))
                }
                rdev::Key::KeyC => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyC))
                }
                rdev::Key::KeyV => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyV))
                }
                rdev::Key::KeyB => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyB))
                }
                rdev::Key::KeyN => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyN))
                }
                rdev::Key::KeyM => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KeyM))
                }
                rdev::Key::Comma => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Comma))
                }
                rdev::Key::Dot => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Dot))
                }
                rdev::Key::Slash => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Slash))
                }
                rdev::Key::Insert => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Insert))
                }
                rdev::Key::KpReturn => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpReturn))
                }
                rdev::Key::KpMinus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpMinus))
                }
                rdev::Key::KpPlus => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpPlus))
                }
                rdev::Key::KpMultiply => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpMultiply))
                }
                rdev::Key::KpDivide => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpDivide))
                }
                rdev::Key::Kp0 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp0))
                }
                rdev::Key::Kp1 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp1))
                }
                rdev::Key::Kp2 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp2))
                }
                rdev::Key::Kp3 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp3))
                }
                rdev::Key::Kp4 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp4))
                }
                rdev::Key::Kp5 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp5))
                }
                rdev::Key::Kp6 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp6))
                }
                rdev::Key::Kp7 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp7))
                }
                rdev::Key::Kp8 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp8))
                }
                rdev::Key::Kp9 => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Kp9))
                }
                rdev::Key::KpDelete => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::KpDelete))
                }
                rdev::Key::Function => {
                    DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(KeyBoardKey::Function))
                }
                rdev::Key::Unknown(b) => DeviceEvent::KeyBoardEvent(KeyBoardEvent::Release(
                    KeyBoardKey::Unknown(b.into()),
                )),
            },
            // mouse
            rdev::EventType::ButtonPress(button) => match button {
                rdev::Button::Left => DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Left)),
                rdev::Button::Right => DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Right)),
                rdev::Button::Middle => {
                    DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Middle))
                }
                rdev::Button::Unknown(b) => {
                    DeviceEvent::MouseEvent(MouseEvent::Press(MouseKey::Unknown(b.into())))
                }
            },
            rdev::EventType::ButtonRelease(button) => match button {
                rdev::Button::Left => DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Left)),
                rdev::Button::Right => DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Right)),
                rdev::Button::Middle => {
                    DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Middle))
                }
                rdev::Button::Unknown(b) => {
                    DeviceEvent::MouseEvent(MouseEvent::Rlease(MouseKey::Unknown(b.into())))
                }
            },
            rdev::EventType::MouseMove { x, y } => {
                DeviceEvent::MouseEvent(MouseEvent::Move(MouseMove { x, y }))
            }
            rdev::EventType::Wheel {
                delta_x: _,
                delta_y,
            } => DeviceEvent::MouseEvent(if delta_y > 0 {
                MouseEvent::Wheel(MouseWheel::Up)
            } else {
                MouseEvent::Wheel(MouseWheel::Down)
            }),
        };

        device_event
    }

    pub fn send_keyboard_mouse_event(&self) {
        match self {
            DeviceEvent::KeyBoardEvent(event) => match event {
                KeyBoardEvent::Press(key) => match key {
                    KeyBoardKey::Alt => send(&EventType::KeyPress(Key::Alt)),
                    KeyBoardKey::AltGr => send(&EventType::KeyPress(Key::AltGr)),
                    KeyBoardKey::Backspace => send(&EventType::KeyPress(Key::Backspace)),
                    KeyBoardKey::CapsLock => send(&EventType::KeyPress(Key::CapsLock)),
                    KeyBoardKey::ControlLeft => send(&EventType::KeyPress(Key::ControlLeft)),
                    KeyBoardKey::ControlRight => send(&EventType::KeyPress(Key::ControlRight)),
                    KeyBoardKey::Delete => send(&EventType::KeyPress(Key::Delete)),
                    KeyBoardKey::DownArrow => send(&EventType::KeyPress(Key::DownArrow)),
                    KeyBoardKey::End => send(&EventType::KeyPress(Key::End)),
                    KeyBoardKey::Escape => send(&EventType::KeyPress(Key::Escape)),
                    KeyBoardKey::F1 => send(&EventType::KeyPress(Key::F1)),
                    KeyBoardKey::F10 => send(&EventType::KeyPress(Key::F10)),
                    KeyBoardKey::F11 => send(&EventType::KeyPress(Key::F11)),
                    KeyBoardKey::F12 => send(&EventType::KeyPress(Key::F12)),
                    KeyBoardKey::F2 => send(&EventType::KeyPress(Key::F2)),
                    KeyBoardKey::F3 => send(&EventType::KeyPress(Key::F3)),
                    KeyBoardKey::F4 => send(&EventType::KeyPress(Key::F4)),
                    KeyBoardKey::F5 => send(&EventType::KeyPress(Key::F5)),
                    KeyBoardKey::F6 => send(&EventType::KeyPress(Key::F6)),
                    KeyBoardKey::F7 => send(&EventType::KeyPress(Key::F7)),
                    KeyBoardKey::F8 => send(&EventType::KeyPress(Key::F8)),
                    KeyBoardKey::F9 => send(&EventType::KeyPress(Key::F9)),
                    KeyBoardKey::Home => send(&EventType::KeyPress(Key::Home)),
                    KeyBoardKey::LeftArrow => send(&EventType::KeyPress(Key::LeftArrow)),
                    KeyBoardKey::MetaLeft => send(&EventType::KeyPress(Key::MetaLeft)),
                    KeyBoardKey::MetaRight => send(&EventType::KeyPress(Key::MetaRight)),
                    KeyBoardKey::PageDown => send(&EventType::KeyPress(Key::PageDown)),
                    KeyBoardKey::PageUp => send(&EventType::KeyPress(Key::PageUp)),
                    KeyBoardKey::Return => send(&EventType::KeyPress(Key::Return)),
                    KeyBoardKey::RightArrow => send(&EventType::KeyPress(Key::RightArrow)),
                    KeyBoardKey::ShiftLeft => send(&EventType::KeyPress(Key::ShiftLeft)),
                    KeyBoardKey::ShiftRight => send(&EventType::KeyPress(Key::ShiftRight)),
                    KeyBoardKey::Space => send(&EventType::KeyPress(Key::Space)),
                    KeyBoardKey::Tab => send(&EventType::KeyPress(Key::Tab)),
                    KeyBoardKey::UpArrow => send(&EventType::KeyPress(Key::UpArrow)),
                    KeyBoardKey::PrintScreen => send(&EventType::KeyPress(Key::PrintScreen)),
                    KeyBoardKey::ScrollLock => send(&EventType::KeyPress(Key::ScrollLock)),
                    KeyBoardKey::Pause => send(&EventType::KeyPress(Key::Pause)),
                    KeyBoardKey::NumLock => send(&EventType::KeyPress(Key::NumLock)),
                    KeyBoardKey::BackQuote => send(&EventType::KeyPress(Key::BackQuote)),
                    KeyBoardKey::Num1 => send(&EventType::KeyPress(Key::Num1)),
                    KeyBoardKey::Num2 => send(&EventType::KeyPress(Key::Num2)),
                    KeyBoardKey::Num3 => send(&EventType::KeyPress(Key::Num3)),
                    KeyBoardKey::Num4 => send(&EventType::KeyPress(Key::Num4)),
                    KeyBoardKey::Num5 => send(&EventType::KeyPress(Key::Num5)),
                    KeyBoardKey::Num6 => send(&EventType::KeyPress(Key::Num6)),
                    KeyBoardKey::Num7 => send(&EventType::KeyPress(Key::Num7)),
                    KeyBoardKey::Num8 => send(&EventType::KeyPress(Key::Num8)),
                    KeyBoardKey::Num9 => send(&EventType::KeyPress(Key::Num9)),
                    KeyBoardKey::Num0 => send(&EventType::KeyPress(Key::Num0)),
                    KeyBoardKey::Minus => send(&EventType::KeyPress(Key::Minus)),
                    KeyBoardKey::Equal => send(&EventType::KeyPress(Key::Equal)),
                    KeyBoardKey::KeyQ => send(&EventType::KeyPress(Key::KeyQ)),
                    KeyBoardKey::KeyW => send(&EventType::KeyPress(Key::KeyW)),
                    KeyBoardKey::KeyE => send(&EventType::KeyPress(Key::KeyE)),
                    KeyBoardKey::KeyR => send(&EventType::KeyPress(Key::KeyR)),
                    KeyBoardKey::KeyT => send(&EventType::KeyPress(Key::KeyT)),
                    KeyBoardKey::KeyY => send(&EventType::KeyPress(Key::KeyY)),
                    KeyBoardKey::KeyU => send(&EventType::KeyPress(Key::KeyU)),
                    KeyBoardKey::KeyI => send(&EventType::KeyPress(Key::KeyI)),
                    KeyBoardKey::KeyO => send(&EventType::KeyPress(Key::KeyO)),
                    KeyBoardKey::KeyP => send(&EventType::KeyPress(Key::KeyP)),
                    KeyBoardKey::LeftBracket => send(&EventType::KeyPress(Key::LeftBracket)),
                    KeyBoardKey::RightBracket => send(&EventType::KeyPress(Key::RightBracket)),
                    KeyBoardKey::KeyA => send(&EventType::KeyPress(Key::KeyA)),
                    KeyBoardKey::KeyS => send(&EventType::KeyPress(Key::KeyS)),
                    KeyBoardKey::KeyD => send(&EventType::KeyPress(Key::KeyD)),
                    KeyBoardKey::KeyF => send(&EventType::KeyPress(Key::KeyF)),
                    KeyBoardKey::KeyG => send(&EventType::KeyPress(Key::KeyG)),
                    KeyBoardKey::KeyH => send(&EventType::KeyPress(Key::KeyH)),
                    KeyBoardKey::KeyJ => send(&EventType::KeyPress(Key::KeyJ)),
                    KeyBoardKey::KeyK => send(&EventType::KeyPress(Key::KeyK)),
                    KeyBoardKey::KeyL => send(&EventType::KeyPress(Key::KeyL)),
                    KeyBoardKey::SemiColon => send(&EventType::KeyPress(Key::SemiColon)),
                    KeyBoardKey::Quote => send(&EventType::KeyPress(Key::Quote)),
                    KeyBoardKey::BackSlash => send(&EventType::KeyPress(Key::BackSlash)),
                    KeyBoardKey::IntlBackslash => send(&EventType::KeyPress(Key::IntlBackslash)),
                    KeyBoardKey::KeyZ => send(&EventType::KeyPress(Key::KeyZ)),
                    KeyBoardKey::KeyX => send(&EventType::KeyPress(Key::KeyX)),
                    KeyBoardKey::KeyC => send(&EventType::KeyPress(Key::KeyC)),
                    KeyBoardKey::KeyV => send(&EventType::KeyPress(Key::KeyV)),
                    KeyBoardKey::KeyB => send(&EventType::KeyPress(Key::KeyB)),
                    KeyBoardKey::KeyN => send(&EventType::KeyPress(Key::KeyN)),
                    KeyBoardKey::KeyM => send(&EventType::KeyPress(Key::KeyM)),
                    KeyBoardKey::Comma => send(&EventType::KeyPress(Key::Comma)),
                    KeyBoardKey::Dot => send(&EventType::KeyPress(Key::Dot)),
                    KeyBoardKey::Slash => send(&EventType::KeyPress(Key::Slash)),
                    KeyBoardKey::Insert => send(&EventType::KeyPress(Key::Insert)),
                    KeyBoardKey::KpReturn => send(&EventType::KeyPress(Key::KpReturn)),
                    KeyBoardKey::KpMinus => send(&EventType::KeyPress(Key::KpMinus)),
                    KeyBoardKey::KpPlus => send(&EventType::KeyPress(Key::KpPlus)),
                    KeyBoardKey::KpMultiply => send(&EventType::KeyPress(Key::KpMultiply)),
                    KeyBoardKey::KpDivide => send(&EventType::KeyPress(Key::KpDivide)),
                    KeyBoardKey::Kp0 => send(&EventType::KeyPress(Key::Kp0)),
                    KeyBoardKey::Kp1 => send(&EventType::KeyPress(Key::Kp1)),
                    KeyBoardKey::Kp2 => send(&EventType::KeyPress(Key::Kp2)),
                    KeyBoardKey::Kp3 => send(&EventType::KeyPress(Key::Kp3)),
                    KeyBoardKey::Kp4 => send(&EventType::KeyPress(Key::Kp4)),
                    KeyBoardKey::Kp5 => send(&EventType::KeyPress(Key::Kp5)),
                    KeyBoardKey::Kp6 => send(&EventType::KeyPress(Key::Kp6)),
                    KeyBoardKey::Kp7 => send(&EventType::KeyPress(Key::Kp7)),
                    KeyBoardKey::Kp8 => send(&EventType::KeyPress(Key::Kp8)),
                    KeyBoardKey::Kp9 => send(&EventType::KeyPress(Key::Kp9)),
                    KeyBoardKey::KpDelete => send(&EventType::KeyPress(Key::KpDelete)),
                    KeyBoardKey::Function => send(&EventType::KeyPress(Key::Function)),
                    KeyBoardKey::Unknown(k) => send(&EventType::KeyPress(Key::Unknown(*k as u32))),
                },
                KeyBoardEvent::Release(key) => match key {
                    KeyBoardKey::Alt => send(&EventType::KeyRelease(Key::Alt)),
                    KeyBoardKey::AltGr => send(&EventType::KeyRelease(Key::AltGr)),
                    KeyBoardKey::Backspace => send(&EventType::KeyRelease(Key::Backspace)),
                    KeyBoardKey::CapsLock => send(&EventType::KeyRelease(Key::CapsLock)),
                    KeyBoardKey::ControlLeft => send(&EventType::KeyRelease(Key::ControlLeft)),
                    KeyBoardKey::ControlRight => send(&EventType::KeyRelease(Key::ControlRight)),
                    KeyBoardKey::Delete => send(&EventType::KeyRelease(Key::Delete)),
                    KeyBoardKey::DownArrow => send(&EventType::KeyRelease(Key::DownArrow)),
                    KeyBoardKey::End => send(&EventType::KeyRelease(Key::End)),
                    KeyBoardKey::Escape => send(&EventType::KeyRelease(Key::Escape)),
                    KeyBoardKey::F1 => send(&EventType::KeyRelease(Key::F1)),
                    KeyBoardKey::F10 => send(&EventType::KeyRelease(Key::F10)),
                    KeyBoardKey::F11 => send(&EventType::KeyRelease(Key::F11)),
                    KeyBoardKey::F12 => send(&EventType::KeyRelease(Key::F12)),
                    KeyBoardKey::F2 => send(&EventType::KeyRelease(Key::F2)),
                    KeyBoardKey::F3 => send(&EventType::KeyRelease(Key::F3)),
                    KeyBoardKey::F4 => send(&EventType::KeyRelease(Key::F4)),
                    KeyBoardKey::F5 => send(&EventType::KeyRelease(Key::F5)),
                    KeyBoardKey::F6 => send(&EventType::KeyRelease(Key::F6)),
                    KeyBoardKey::F7 => send(&EventType::KeyRelease(Key::F7)),
                    KeyBoardKey::F8 => send(&EventType::KeyRelease(Key::F8)),
                    KeyBoardKey::F9 => send(&EventType::KeyRelease(Key::F9)),
                    KeyBoardKey::Home => send(&EventType::KeyRelease(Key::Home)),
                    KeyBoardKey::LeftArrow => send(&EventType::KeyRelease(Key::LeftArrow)),
                    KeyBoardKey::MetaLeft => send(&EventType::KeyRelease(Key::MetaLeft)),
                    KeyBoardKey::MetaRight => send(&EventType::KeyRelease(Key::MetaRight)),
                    KeyBoardKey::PageDown => send(&EventType::KeyRelease(Key::PageDown)),
                    KeyBoardKey::PageUp => send(&EventType::KeyRelease(Key::PageUp)),
                    KeyBoardKey::Return => send(&EventType::KeyRelease(Key::Return)),
                    KeyBoardKey::RightArrow => send(&EventType::KeyRelease(Key::RightArrow)),
                    KeyBoardKey::ShiftLeft => send(&EventType::KeyRelease(Key::ShiftLeft)),
                    KeyBoardKey::ShiftRight => send(&EventType::KeyRelease(Key::ShiftRight)),
                    KeyBoardKey::Space => send(&EventType::KeyRelease(Key::Space)),
                    KeyBoardKey::Tab => send(&EventType::KeyRelease(Key::Tab)),
                    KeyBoardKey::UpArrow => send(&EventType::KeyRelease(Key::UpArrow)),
                    KeyBoardKey::PrintScreen => send(&EventType::KeyRelease(Key::PrintScreen)),
                    KeyBoardKey::ScrollLock => send(&EventType::KeyRelease(Key::ScrollLock)),
                    KeyBoardKey::Pause => send(&EventType::KeyRelease(Key::Pause)),
                    KeyBoardKey::NumLock => send(&EventType::KeyRelease(Key::NumLock)),
                    KeyBoardKey::BackQuote => send(&EventType::KeyRelease(Key::BackQuote)),
                    KeyBoardKey::Num1 => send(&EventType::KeyRelease(Key::Num1)),
                    KeyBoardKey::Num2 => send(&EventType::KeyRelease(Key::Num2)),
                    KeyBoardKey::Num3 => send(&EventType::KeyRelease(Key::Num3)),
                    KeyBoardKey::Num4 => send(&EventType::KeyRelease(Key::Num4)),
                    KeyBoardKey::Num5 => send(&EventType::KeyRelease(Key::Num5)),
                    KeyBoardKey::Num6 => send(&EventType::KeyRelease(Key::Num6)),
                    KeyBoardKey::Num7 => send(&EventType::KeyRelease(Key::Num7)),
                    KeyBoardKey::Num8 => send(&EventType::KeyRelease(Key::Num8)),
                    KeyBoardKey::Num9 => send(&EventType::KeyRelease(Key::Num9)),
                    KeyBoardKey::Num0 => send(&EventType::KeyRelease(Key::Num0)),
                    KeyBoardKey::Minus => send(&EventType::KeyRelease(Key::Minus)),
                    KeyBoardKey::Equal => send(&EventType::KeyRelease(Key::Equal)),
                    KeyBoardKey::KeyQ => send(&EventType::KeyRelease(Key::KeyQ)),
                    KeyBoardKey::KeyW => send(&EventType::KeyRelease(Key::KeyW)),
                    KeyBoardKey::KeyE => send(&EventType::KeyRelease(Key::KeyE)),
                    KeyBoardKey::KeyR => send(&EventType::KeyRelease(Key::KeyR)),
                    KeyBoardKey::KeyT => send(&EventType::KeyRelease(Key::KeyT)),
                    KeyBoardKey::KeyY => send(&EventType::KeyRelease(Key::KeyY)),
                    KeyBoardKey::KeyU => send(&EventType::KeyRelease(Key::KeyU)),
                    KeyBoardKey::KeyI => send(&EventType::KeyRelease(Key::KeyI)),
                    KeyBoardKey::KeyO => send(&EventType::KeyRelease(Key::KeyO)),
                    KeyBoardKey::KeyP => send(&EventType::KeyRelease(Key::KeyP)),
                    KeyBoardKey::LeftBracket => send(&EventType::KeyRelease(Key::LeftBracket)),
                    KeyBoardKey::RightBracket => send(&EventType::KeyRelease(Key::RightBracket)),
                    KeyBoardKey::KeyA => send(&EventType::KeyRelease(Key::KeyA)),
                    KeyBoardKey::KeyS => send(&EventType::KeyRelease(Key::KeyS)),
                    KeyBoardKey::KeyD => send(&EventType::KeyRelease(Key::KeyD)),
                    KeyBoardKey::KeyF => send(&EventType::KeyRelease(Key::KeyF)),
                    KeyBoardKey::KeyG => send(&EventType::KeyRelease(Key::KeyG)),
                    KeyBoardKey::KeyH => send(&EventType::KeyRelease(Key::KeyH)),
                    KeyBoardKey::KeyJ => send(&EventType::KeyRelease(Key::KeyJ)),
                    KeyBoardKey::KeyK => send(&EventType::KeyRelease(Key::KeyK)),
                    KeyBoardKey::KeyL => send(&EventType::KeyRelease(Key::KeyL)),
                    KeyBoardKey::SemiColon => send(&EventType::KeyRelease(Key::SemiColon)),
                    KeyBoardKey::Quote => send(&EventType::KeyRelease(Key::Quote)),
                    KeyBoardKey::BackSlash => send(&EventType::KeyRelease(Key::BackSlash)),
                    KeyBoardKey::IntlBackslash => send(&EventType::KeyRelease(Key::IntlBackslash)),
                    KeyBoardKey::KeyZ => send(&EventType::KeyRelease(Key::KeyZ)),
                    KeyBoardKey::KeyX => send(&EventType::KeyRelease(Key::KeyX)),
                    KeyBoardKey::KeyC => send(&EventType::KeyRelease(Key::KeyC)),
                    KeyBoardKey::KeyV => send(&EventType::KeyRelease(Key::KeyV)),
                    KeyBoardKey::KeyB => send(&EventType::KeyRelease(Key::KeyB)),
                    KeyBoardKey::KeyN => send(&EventType::KeyRelease(Key::KeyN)),
                    KeyBoardKey::KeyM => send(&EventType::KeyRelease(Key::KeyM)),
                    KeyBoardKey::Comma => send(&EventType::KeyRelease(Key::Comma)),
                    KeyBoardKey::Dot => send(&EventType::KeyRelease(Key::Dot)),
                    KeyBoardKey::Slash => send(&EventType::KeyRelease(Key::Slash)),
                    KeyBoardKey::Insert => send(&EventType::KeyRelease(Key::Insert)),
                    KeyBoardKey::KpReturn => send(&EventType::KeyRelease(Key::KpReturn)),
                    KeyBoardKey::KpMinus => send(&EventType::KeyRelease(Key::KpMinus)),
                    KeyBoardKey::KpPlus => send(&EventType::KeyRelease(Key::KpPlus)),
                    KeyBoardKey::KpMultiply => send(&EventType::KeyRelease(Key::KpMultiply)),
                    KeyBoardKey::KpDivide => send(&EventType::KeyRelease(Key::KpDivide)),
                    KeyBoardKey::Kp0 => send(&EventType::KeyRelease(Key::Kp0)),
                    KeyBoardKey::Kp1 => send(&EventType::KeyRelease(Key::Kp1)),
                    KeyBoardKey::Kp2 => send(&EventType::KeyRelease(Key::Kp2)),
                    KeyBoardKey::Kp3 => send(&EventType::KeyRelease(Key::Kp3)),
                    KeyBoardKey::Kp4 => send(&EventType::KeyRelease(Key::Kp4)),
                    KeyBoardKey::Kp5 => send(&EventType::KeyRelease(Key::Kp5)),
                    KeyBoardKey::Kp6 => send(&EventType::KeyRelease(Key::Kp6)),
                    KeyBoardKey::Kp7 => send(&EventType::KeyRelease(Key::Kp7)),
                    KeyBoardKey::Kp8 => send(&EventType::KeyRelease(Key::Kp8)),
                    KeyBoardKey::Kp9 => send(&EventType::KeyRelease(Key::Kp9)),
                    KeyBoardKey::KpDelete => send(&EventType::KeyRelease(Key::KpDelete)),
                    KeyBoardKey::Function => send(&EventType::KeyRelease(Key::Function)),
                    KeyBoardKey::Unknown(k) => {
                        send(&EventType::KeyRelease(Key::Unknown(*k as u32)))
                    }
                },
            },
            DeviceEvent::MouseEvent(event) => match event {
                MouseEvent::Press(k) => match k {
                    MouseKey::Left => send(&EventType::ButtonPress(Button::Left)),
                    MouseKey::Right => send(&EventType::ButtonPress(Button::Right)),
                    MouseKey::Middle => send(&EventType::ButtonPress(Button::Middle)),
                    MouseKey::Unknown(k) => {
                        send(&EventType::ButtonPress(Button::Unknown(*k as u8)))
                    }
                },
                MouseEvent::Rlease(k) => match k {
                    MouseKey::Left => send(&EventType::ButtonRelease(Button::Left)),
                    MouseKey::Right => send(&EventType::ButtonRelease(Button::Right)),
                    MouseKey::Middle => send(&EventType::ButtonRelease(Button::Middle)),
                    MouseKey::Unknown(k) => {
                        send(&EventType::ButtonRelease(Button::Unknown(*k as u8)))
                    }
                },
                MouseEvent::Move(m) => send(&EventType::MouseMove { x: m.x, y: m.y }),
                MouseEvent::Wheel(w) => match w {
                    MouseWheel::Up => send(&EventType::Wheel {
                        delta_x: 0,
                        delta_y: 1,
                    }),
                    MouseWheel::Down => send(&EventType::Wheel {
                        delta_x: 0,
                        delta_y: -1,
                    }),
                },
            },
        }
    }
}
