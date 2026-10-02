use crate::app::block::{Block, BlockStyle};
use crate::geom::Size;

// -------------------------------------------------------------
// Semi-Public API
// -------------------------------------------------------------

// -----------------------------
// Entry Function
// -----------------------------

/// Create a builder object for the Video System.
#[allow(unused)]
pub(super) fn configure() -> VideoBuilder {
    VideoBuilder::new()
}

// -----------------------------
// Public Structs
// -----------------------------

/// Abstraction over Window & Rendering facilities.
pub(super) struct Video {
    pub(super) canvas: sdl3::render::Canvas<sdl3::video::Window>,
}

pub(super) struct VideoBuilder {
    window: WindowConfigBuilder,
    render: RenderConfigBuilder,
}

#[derive(Default)]
pub(super) struct WindowConfigBuilder {
    config: WindowConfig,
}

#[derive(Default)]
pub(super) struct RenderConfigBuilder {
    config: RenderConfig,
}

// -----------------------------
// Public API Impl
// -----------------------------

#[allow(unused)]
impl Video {
    fn init(sdl: &sdl3::Sdl, config: Config) -> Result<Self, sdl3::video::WindowBuildError> {
        let sdl_video = sdl.video().expect("SDL3 Video Subsystem initializes");
        let sdl_window = sdl_video
            .window(
                config.window.title.as_str(),
                config.window.size.0,
                config.window.size.1,
            )
            .resizable()
            .position_centered()
            .build()?;

        let mut sdl_canvas: sdl3::render::Canvas<sdl3::video::Window> = sdl_window.into_canvas();

        sdl_canvas.set_blend_mode(config.render.blend_mode);

        Ok(Self { canvas: sdl_canvas })
    }

    pub(super) fn draw_block(&mut self, block: &Block) -> Result<(), sdl3::Error> {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(block.color);
        match block.style {
            BlockStyle::Outlined => canvas.draw_rect(block.rect)?,
            BlockStyle::Filled => canvas.fill_rect(block.rect)?,
        }
        Ok(())
    }
}

#[allow(unused)]
impl VideoBuilder {
    fn new() -> Self {
        Self {
            window: WindowConfigBuilder::default(),
            render: RenderConfigBuilder::default(),
        }
    }

    pub(super) fn build(self, sdl: &sdl3::Sdl) -> Result<Video, sdl3::video::WindowBuildError> {
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
// Internal Structs
// -------------------------------------------------------------

// -----------------------------
// Config + Builder Structs
// -----------------------------

#[derive(Default, PartialEq, Debug)]
struct Config {
    window: WindowConfig,
    render: RenderConfig,
}

/// A snapshot of the latest SDL3 Window information.
#[derive(PartialEq, Debug)]
struct WindowConfig {
    title: String,
    size: Size,
}

#[derive(PartialEq, Debug)]
struct RenderConfig {
    blend_mode: sdl3::render::BlendMode,
}

// -------------------------------------------------------------
// Trait Implementations
// -------------------------------------------------------------

impl From<VideoBuilder> for Config {
    fn from(value: VideoBuilder) -> Self {
        Self {
            window: value.window.config,
            render: value.render.config,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "SDL3 Application".into(),
            size: Size::new(320, 240),
        }
    }
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
