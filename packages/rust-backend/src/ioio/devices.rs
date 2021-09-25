#![allow(dead_code)]

pub mod keyboard;
pub mod mouse;
use keyboard::{KeyBoardEvent, KeyBoardKey};
use mouse::{MouseEvent, MouseKey, MouseMove, MouseWheel};
use rdev::Event;

#[derive(Debug)]
pub enum DeviceEvent {
    KeyBoardEvent(keyboard::KeyBoardEvent),
    MouseEvent(mouse::MouseEvent),
}

impl DeviceEvent {
    // pub fn json(&self) -> String {
    //     serde_json::to_string(self).expect("Serialize error!")
    // }

    pub fn receive_from_keyboard_mouse_event(event: &Event) -> DeviceEvent {
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
}
