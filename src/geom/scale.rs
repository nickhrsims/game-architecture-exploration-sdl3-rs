#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scale(pub f32, pub f32);

impl Scale {
    pub fn new(w: f32, h: f32) -> Self {
        if w <= 0f32 || h <= 0f32 {
            panic!("width({w}) and height({h}) must be positive and non-zero");
        }
        Self(w, h)
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self::new(1f32, 1f32)
    }
}

impl From<(f32, f32)> for Scale {
    fn from(value: (f32, f32)) -> Self {
        let (w, h) = value;
        Self::new(w, h)
    }
}

impl From<(i32, i32)> for Scale {
    fn from(value: (i32, i32)) -> Self {
        let (w, h) = value;
        Self::new(w as f32, h as f32)
    }
}

impl From<f32> for Scale {
    fn from(value: f32) -> Self {
        Self::new(value, value)
    }
}

impl From<i8> for Scale {
    fn from(value: i8) -> Self {
        Self::new(value as f32, value as f32)
    }
}

impl From<i16> for Scale {
    fn from(value: i16) -> Self {
        Self::new(value as f32, value as f32)
    }
}

impl From<i32> for Scale {
    fn from(value: i32) -> Self {
        Self::new(value as f32, value as f32)
    }
}

impl From<u8> for Scale {
    fn from(value: u8) -> Self {
        Self::new(value as f32, value as f32)
    }
}

impl From<u16> for Scale {
    fn from(value: u16) -> Self {
        Self::new(value as f32, value as f32)
    }
}
