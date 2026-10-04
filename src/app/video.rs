// -------------------------------------------------------------
// Imports
// -------------------------------------------------------------

// -- Crate

use crate::geom::{Point, Size};

// -- Sibling

use super::block::{Block, BlockStyle};
use super::text;

// -------------------------------------------------------------
// Type Aliases
// -------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

// -------------------------------------------------------------
// Primary :: Video
// -------------------------------------------------------------

// -----------------------------
// Entry-Point
// -----------------------------

/// Create a builder object for the Video System.
#[allow(unused)]
pub(super) fn configure() -> VideoBuilder {
    VideoBuilder::new()
}

// -----------------------------
// Video :: Public API
// -----------------------------

/// Abstraction over Window & Rendering facilities.
pub(super) struct Video<'ttf> {
    pub(super) canvas: sdl3::render::WindowCanvas,
    text: text::TextSystem<'ttf>,
}

impl Video<'_> {
    pub(super) fn draw_block(&mut self, block: &Block) -> Result<(), Error> {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(block.color);
        match block.style {
            BlockStyle::Outlined => canvas.draw_rect(block.rect)?,
            BlockStyle::Filled => canvas.fill_rect(block.rect)?,
        }
        Ok(())
    }

    pub(super) fn draw_text(&mut self, text: &text::Text) -> Result<(), Error> {
        let ttf_text = self.text.resolve(&text.msg, text.font, text.pt)?;
        let size: Size = ttf_text.size().into();
        let position = text.position - text.anchor.offset(size);

        self.canvas.set_draw_color(text.color);
        ttf_text.draw(position.0 as f32, position.1 as f32)?;
        Ok(())
    }
}

// -----------------------------
// Video :: Internals
// -----------------------------

impl Video<'_> {
    fn init(sdl: &sdl3::Sdl, config: Config) -> Result<Self, Error> {
        // SDL3 Video Subsystem
        let video = sdl.video()?;

        // SDL3 Window
        let window = video
            .window(
                config.window.title.as_str(),
                config.window.size.0,
                config.window.size.1,
            )
            .resizable()
            .position_centered()
            .build()?;

        // SDL3 Canvas
        let mut canvas: sdl3::render::WindowCanvas = window.into_canvas();
        canvas.set_blend_mode(config.render.blend_mode);

        let text = text::TextSystem::init(&canvas.texture_creator(), "./data/font")?;

        Ok(Self { canvas, text })
    }
}

// -------------------------------------------------------------
// Secondary :: Video Builder
// -------------------------------------------------------------

// -----------------------------
// Root Builder
// -----------------------------

pub(super) struct VideoBuilder {
    window: WindowConfigBuilder,
    render: RenderConfigBuilder,
}

#[allow(unused)]
impl VideoBuilder {
    fn new() -> Self {
        Self {
            window: WindowConfigBuilder::default(),
            render: RenderConfigBuilder::default(),
        }
    }

    pub(super) fn build<'ttf>(self, sdl: &sdl3::Sdl) -> Result<Video<'ttf>, Error> {
        Video::init(sdl, Config::from(self))
    }

    pub(super) fn window<F>(mut self, f: F) -> Self
    where
        F: FnOnce(WindowConfigBuilder) -> WindowConfigBuilder,
    {
        self.window = f(self.window);
        self
    }

    pub(super) fn rendering<F>(mut self, f: F) -> Self
    where
        F: FnOnce(RenderConfigBuilder) -> RenderConfigBuilder,
    {
        self.render = f(self.render);
        self
    }
}

// -----------------------------
// Window Config Sub-Builder
// -----------------------------

#[derive(Default)]
pub(super) struct WindowConfigBuilder {
    config: WindowConfig,
}

#[allow(unused)]
impl WindowConfigBuilder {
    pub(super) fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub(super) fn size(mut self, size: impl Into<Size>) -> Self {
        self.config.size = size.into();
        self
    }
}
// -----------------------------
// Render Config Sub-Builder
// -----------------------------

#[derive(Default)]
pub(super) struct RenderConfigBuilder {
    config: RenderConfig,
}

#[allow(unused)]
impl RenderConfigBuilder {
    // -- Blend Mode

    pub(super) fn blend_mode(mut self, blend_mode: sdl3::render::BlendMode) -> Self {
        self.config.blend_mode = blend_mode;
        self
    }

    pub(super) fn no_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::None)
    }

    pub(super) fn alpha_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Blend)
    }

    pub(super) fn additive_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Add)
    }

    pub(super) fn color_modulated_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Mod)
    }

    pub(super) fn color_multiply_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Mul)
    }
}

// -------------------------------------------------------------
// Internal :: Configuration Structs
// -------------------------------------------------------------

// -----------------------------
// Config Structs
// -----------------------------

#[derive(Default, PartialEq, Debug)]
struct Config {
    window: WindowConfig,
    render: RenderConfig,
}

impl From<VideoBuilder> for Config {
    fn from(value: VideoBuilder) -> Self {
        Self {
            window: value.window.config,
            render: value.render.config,
        }
    }
}

/// A snapshot of the latest SDL3 Window information.
#[derive(PartialEq, Debug)]
struct WindowConfig {
    title: String,
    size: Size,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "SDL3 Application".into(),
            size: Size::new(320, 240),
        }
    }
}

#[derive(PartialEq, Debug)]
struct RenderConfig {
    blend_mode: sdl3::render::BlendMode,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            blend_mode: sdl3::render::BlendMode::None,
        }
    }
}

// -------------------------------------------------------------
// Tests
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_configures_video_system() {
        let window_title = "Test Title";
        let window_size = (100, 100);
        let render_blend_mode = sdl3::render::BlendMode::Invalid;

        let target = Config {
            window: WindowConfig {
                title: window_title.into(),
                size: window_size.into(),
            },
            render: RenderConfig {
                blend_mode: render_blend_mode,
            },
        };

        let subject = Config::from(
            configure()
                .window(|w| w.title(window_title).size(window_size))
                .rendering(|r| r.blend_mode(render_blend_mode)),
        );

        assert_eq!(subject, target);
    }
}
