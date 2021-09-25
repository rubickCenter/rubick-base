#![allow(dead_code)]

#[derive(Debug)]
pub enum MouseKey {
    Left,
    Right,
    Middle,
    Unknown(f64),
}

#[derive(Debug)]
pub struct MouseMove {
    pub x: f64,
    pub y: f64,
}

impl MouseMove {
    pub fn to_string(&self) -> String {
        format!("{{\"x\":{},\"y\":{}}}", self.x, self.y)
    }
}

#[derive(Debug)]
pub enum MouseWheel {
    Up,
    Down,
}

#[derive(Debug)]
pub enum MouseEvent {
    Press(MouseKey),
    Rlease(MouseKey),
    Move(MouseMove),
    Wheel(MouseWheel),
}
