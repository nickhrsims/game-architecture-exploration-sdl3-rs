impl From<crate::geom::Rect> for sdl3::render::FRect {
    fn from(value: crate::geom::Rect) -> Self {
        sdl3::render::FRect {
            x: value.position.0 as f32,
            y: value.position.1 as f32,
            w: value.size.0 as f32,
            h: value.size.1 as f32,
        }
    }
}

impl From<crate::geom::Rect> for Option<sdl3::render::FRect> {
    fn from(value: crate::geom::Rect) -> Self {
        Some(sdl3::render::FRect {
            x: value.position.0 as f32,
            y: value.position.1 as f32,
            w: value.size.0 as f32,
            h: value.size.1 as f32,
        })
    }
}
