// -------------------------------------------------------------
// Imports
// -------------------------------------------------------------

use crate::geom::{Point, Size};

// -------------------------------------------------------------
// Text :: Domain API
// -------------------------------------------------------------

#[derive(PartialEq, Eq, Clone)]
pub struct Text {
    pub font: FontId,
    pub msg: String,
    pub pt: u8,
    pub position: Point,
    pub anchor: Anchor,
    pub color: sdl3::pixels::Color,
}

impl Text {
    pub fn new(
        msg: impl Into<String>,
        font: FontId,
        position: impl Into<Point>,
        pt: impl Into<u8>,
        anchor: Anchor,
        color: sdl3::pixels::Color,
    ) -> Self {
        Self {
            font,
            msg: msg.into(),
            position: position.into(),
            pt: pt.into(),
            anchor,
            color,
        }
    }

    pub fn with_msg(&self, msg: impl Into<String>) -> Self {
        let mut text = self.clone();
        text.msg = msg.into();
        text
    }

    pub fn with_position(&self, position: Point) -> Self {
        let mut text = self.clone();
        text.position = position;
        text
    }

    pub fn with_point_size(&self, size: impl Into<u8>) -> Self {
        let mut text = self.clone();
        text.pt = size.into();
        text
    }

    pub fn translated(&self, translation: Point) -> Self {
        let mut text = self.clone();
        text.position += translation;
        text
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct FontId(&'static str);

impl FontId {
    pub(super) const CALIPER: FontId = FontId("caliper.ttf");
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Anchor {
    Top,
    TopLeft,
    TopRight,

    Left,
    #[default]
    Center,
    Right,

    BottomLeft,
    Bottom,
    BottomRight,
}

impl Anchor {
    pub fn offset(&self, size: Size) -> Point {
        Size::from(match self {
            Anchor::TopLeft => (0, 0),
            Anchor::Top => (size.0 / 2u32, 0),
            Anchor::TopRight => (size.0, 0),
            Anchor::Left => (0, size.1 / 2u32),
            Anchor::Center => (size / 2u32).into(),
            Anchor::Right => (size.0, size.1 / 2u32),
            Anchor::BottomLeft => (0, size.1),
            Anchor::Bottom => (size.0 / 2u32, size.1),
            Anchor::BottomRight => size.into(),
        })
        .try_into()
        .expect("Given size fits within a Point")
    }
}

// -------------------------------------------------------------
// Text System :: Internal Video Extension API
// -------------------------------------------------------------

pub(super) struct TextSystem<'ttf> {
    ttf: sdl3::ttf::Sdl3TtfContext,
    engine: sdl3::ttf::TextEngine,
    cache: self::Cache<'ttf>,
    prefix: std::path::PathBuf,
}

impl TextSystem<'_> {
    pub(super) fn init(
        texture_creator: &sdl3::render::TextureCreator<sdl3::video::WindowContext>,
        prefix: impl Into<std::path::PathBuf>,
    ) -> Result<Self, sdl3::Error> {
        let ttf = sdl3::ttf::init()?;
        let engine = sdl3::ttf::TextEngine::new(texture_creator)?;
        let cache = self::Cache::new();
        let prefix = prefix.into();

        Ok(Self {
            ttf,
            engine,
            cache,
            prefix,
        })
    }

    pub(super) fn resolve(
        &mut self,
        msg: &str,
        id: self::FontId,
        pt: u8,
    ) -> Result<sdl3::ttf::Text, sdl3::Error> {
        let font = self.cache.get(&self.ttf, &self.prefix, id, pt)?;
        self.engine.create_text(font, msg)
    }
}

struct Cache<'ttf> {
    table: std::collections::HashMap<(FontId, u8), sdl3::ttf::Font<'ttf>>,
}

impl Cache<'_> {
    fn new() -> Self {
        Self {
            table: std::collections::HashMap::new(),
        }
    }

    fn get(
        &mut self,
        context: &sdl3::ttf::Sdl3TtfContext,
        prefix: &std::path::Path,
        id: self::FontId,
        pt: u8,
    ) -> Result<&sdl3::ttf::Font<'_>, sdl3::Error> {
        let key = (id, pt);

        if let std::collections::hash_map::Entry::Vacant(e) = self.table.entry(key) {
            let font = context.load_font(prefix.join(id.0), pt.into())?;
            e.insert(font);
        }

        Ok(self.table.get(&key).unwrap())
    }
}
