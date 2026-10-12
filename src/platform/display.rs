// -----------------------------------------------------------------------------
//                                                                       Imports
// -----------------------------------------------------------------------------

use crate::geom;

use super::*;

// -----------------------------------------------------------------------------
//                                                                  Type Aliases
// -----------------------------------------------------------------------------

pub type AnyError = Box<dyn std::error::Error>;

// -----------------------------------------------------------------------------
//                                                                   Entry-Point
// -----------------------------------------------------------------------------

/// Create a builder object for the Display System.
pub fn configure() -> DisplayBuilder {
    DisplayBuilder::new()
}

// -----------------------------------------------------------------------------
//                                                                       Display
// -----------------------------------------------------------------------------

/// Abstraction over Window & Rendering facilities.
pub struct DisplaySystem {
    pub canvas: sdl3::render::WindowCanvas,
    pub text_engine: sdl3::ttf::TextEngine,
}

impl DisplaySystem {
    fn init(sdl: &sdl3::Sdl, config: Config) -> Result<Self, AnyError> {
        // SDL3 Display Subsystem
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

        let text_engine = sdl3::ttf::TextEngine::new(&canvas.texture_creator())?;

        Ok(Self {
            canvas,
            text_engine,
        })
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.set_draw_color(sdl3::pixels::Color::BLACK);
        self.canvas.clear();
    }

    pub fn present_canvas(&mut self) -> Result<(), sdl3::Error> {
        if self.canvas.present() {
            Ok(())
        } else {
            Err(sdl3::get_error())
        }
    }

    pub fn draw_rect(
        &mut self,
        rect: geom::Rect,
        color: sdl3::pixels::Color,
    ) -> Result<(), sdl3::Error> {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(color);
        canvas.draw_rect(rect)
    }

    pub fn fill_rect(
        &mut self,
        rect: geom::Rect,
        color: sdl3::pixels::Color,
    ) -> Result<(), sdl3::Error> {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(color);
        canvas.fill_rect(rect)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_text(
        &mut self,
        text_system: &mut text::TextSystem,
        font_id: text::FontId,
        font_pt: u8,
        color: sdl3::pixels::Color,
        text: &str,
        position: geom::Point,
        anchor: geom::Anchor,
    ) -> Result<(), AnyError> {
        let font = text_system.load_registered_font(font_id, font_pt)?;
        let text = self.text_engine.create_text(font, text)?;
        let position = position + anchor.offset(text.size().into());
        self.canvas.set_draw_color(color);
        text.draw(position.0 as f32, position.1 as f32)?;
        Ok(())
    }
}

// -----------------------------------------------------------------------------
//                                                               Display Builder
// -----------------------------------------------------------------------------

pub struct DisplayBuilder {
    window: WindowConfigBuilder,
    render: RenderConfigBuilder,
}

#[allow(unused)]
impl DisplayBuilder {
    fn new() -> Self {
        Self {
            window: WindowConfigBuilder::default(),
            render: RenderConfigBuilder::default(),
        }
    }

    pub fn build(self, sdl: &sdl3::Sdl) -> Result<DisplaySystem, AnyError> {
        DisplaySystem::init(sdl, Config::from(self))
    }

    pub fn window<F>(mut self, f: F) -> Self
    where
        F: FnOnce(WindowConfigBuilder) -> WindowConfigBuilder,
    {
        self.window = f(self.window);
        self
    }

    pub fn rendering<F>(mut self, f: F) -> Self
    where
        F: FnOnce(RenderConfigBuilder) -> RenderConfigBuilder,
    {
        self.render = f(self.render);
        self
    }
}

// -----------------------------------------------------------------------------
//                                                         Window Config Builder
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct WindowConfigBuilder {
    config: WindowConfig,
}

#[allow(unused)]
impl WindowConfigBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub fn size(mut self, size: impl Into<geom::Size>) -> Self {
        self.config.size = size.into();
        self
    }
}

// -----------------------------------------------------------------------------
//                                                         Render Config Builder
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct RenderConfigBuilder {
    config: RenderConfig,
}

#[allow(unused)]
impl RenderConfigBuilder {
    // -- Blend Mode

    pub fn blend_mode(mut self, blend_mode: sdl3::render::BlendMode) -> Self {
        self.config.blend_mode = blend_mode;
        self
    }

    pub fn no_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::None)
    }

    pub fn alpha_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Blend)
    }

    pub fn additive_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Add)
    }

    pub fn color_modulated_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Mod)
    }

    pub fn color_multiply_blending(self) -> Self {
        self.blend_mode(sdl3::render::BlendMode::Mul)
    }
}

// -----------------------------------------------------------------------------
//                                                         Configuration Structs
// -----------------------------------------------------------------------------

#[derive(Default, PartialEq, Debug)]
struct Config {
    window: WindowConfig,
    render: RenderConfig,
}

impl From<DisplayBuilder> for Config {
    fn from(value: DisplayBuilder) -> Self {
        Self {
            window: value.window.config,
            render: value.render.config,
        }
    }
}

#[derive(PartialEq, Debug)]
struct WindowConfig {
    title: String,
    size: geom::Size,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "SDL3 Application".into(),
            size: geom::Size::new(320, 240),
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

// -----------------------------------------------------------------------------
//                                                                  Integrations
// -----------------------------------------------------------------------------

// -----------------------------
// geom::Rect
// -----------------------------

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

// -----------------------------------------------------------------------------
//                                                                         Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_configures_display_system() {
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
