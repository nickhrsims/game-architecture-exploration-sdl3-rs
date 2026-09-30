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
