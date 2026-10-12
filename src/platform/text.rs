// -----------------------------------------------------------------------------
//                                                                       Imports
// -----------------------------------------------------------------------------

// -- Crate

use crate::geom::Size;

// -----------------------------------------------------------------------------
//                                                                  Type Aliases
// -----------------------------------------------------------------------------

pub type AnyError = Box<dyn std::error::Error>;

// -----------------------------------------------------------------------------
//                                                                   Text System
// -----------------------------------------------------------------------------

pub struct TextSystem {
    ttf: sdl3::ttf::Sdl3TtfContext,
    font_registry: FontRegistry,
}

impl TextSystem {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let ttf = sdl3::ttf::init()?;
        let font_registry = FontRegistry::init()?;

        Ok(Self { ttf, font_registry })
    }

    pub fn register_font(&mut self, id: FontId, path: &str) {
        self.font_registry.register(id, path);
    }

    pub fn load_registered_font(
        &mut self,
        id: FontId,
        pt: u8,
    ) -> Result<&sdl3::ttf::Font<'static>, AnyError> {
        self.font_registry.load(&self.ttf, id, pt)
    }

    pub fn get_text_size(&mut self, id: FontId, pt: u8, text: &str) -> Result<Size, AnyError> {
        self.font_registry.size_of(&self.ttf, id, pt, text)
    }
}

// -----------------------------------------------------------------------------
//                                                                 Font Registry
// -----------------------------------------------------------------------------

pub struct FontRegistry {
    index: std::collections::HashMap<FontId, String>,
    cache: std::collections::HashMap<(FontId, u8), sdl3::ttf::Font<'static>>,
}

impl FontRegistry {
    pub fn init() -> Result<Self, AnyError> {
        let index = std::collections::HashMap::new();
        let cache = std::collections::HashMap::new();
        Ok(Self { index, cache })
    }

    pub fn register(&mut self, id: FontId, path: &str) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.index.entry(id) {
            e.insert(String::from(path));
        } else {
            let string = self.index.get_mut(&id).unwrap();
            string.clear();
            string.push_str(path);
        }
    }

    pub fn load(
        &mut self,
        ttf: &sdl3::ttf::Sdl3TtfContext,
        id: FontId,
        pt: u8,
    ) -> Result<&sdl3::ttf::Font<'static>, AnyError> {
        let key = (id, pt);

        if let std::collections::hash_map::Entry::Vacant(e) = self.cache.entry(key) {
            let filepath = self.index.get(&id).ok_or(LoadError::UnregisteredId(id.0))?;
            let pt = pt.into();
            let font = ttf.load_font(filepath, pt)?;
            e.insert(font);
        }

        Ok(self.cache.get(&key).unwrap())
    }

    pub fn size_of(
        &mut self,
        ttf: &sdl3::ttf::Sdl3TtfContext,
        id: FontId,
        pt: u8,
        text: &str,
    ) -> Result<Size, AnyError> {
        Ok(self.load(ttf, id, pt)?.size_of(text)?.into())
    }
}

// -----------------------------
// Identity
// -----------------------------

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FontId(pub &'static str);

impl FontId {
    pub const DEFAULT: FontId = FontId("DEFAULT");
}

// -----------------------------
// Errors
// -----------------------------

#[derive(Debug, Clone)]
pub enum LoadError {
    UnregisteredId(&'static str),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnregisteredId(id) => {
                write!(f, "{id}")
            }
        }
    }
}

impl std::error::Error for LoadError {}
