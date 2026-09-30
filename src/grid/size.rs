use super::Scale;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

impl Size {
    pub fn new(w: u32, h: u32) -> Self {
        if w == 0 || h == 0 {
            panic!("width({w}) and height({h}) must be non-zero");
        }
        Self(w, h)
    }

    pub fn scaled(&self, scale: Scale) -> Self {
        let w = (self.0 as f32 * scale.0) as u32;
        let h = (self.1 as f32 * scale.1) as u32;

        Self(w, h)
    }

    pub fn scale(&mut self, scale: Scale) {
        let w = (self.0 as f32 * scale.0) as u32;
        let h = (self.1 as f32 * scale.1) as u32;

        self.0 = w;
        self.1 = h;
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl std::ops::Add<Size> for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0.checked_add(rhs0).expect("lhs + rhs within bounds");
        let r1 = lhs1.checked_add(rhs1).expect("lhs + rhs within bounds");

        Size(r0, r1)
    }
}

impl std::ops::AddAssign<Size> for Size {
    fn add_assign(&mut self, rhs: Size) {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        self.0 = lhs0.checked_add(rhs0).expect("lhs + rhs within bounds");
        self.1 = lhs1.checked_add(rhs1).expect("lhs + rhs within bounds");
    }
}

impl std::ops::Sub for Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Size {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0 - rhs0;
        let r1 = lhs1 - rhs1;

        Size(r0, r1)
    }
}
