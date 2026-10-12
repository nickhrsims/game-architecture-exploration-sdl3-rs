pub mod commands;
pub mod display;
pub mod event;
pub mod text;
pub mod timer;

pub use commands::DisplayCommands;
pub use display::DisplaySystem;
pub use event::EventPump;
pub use text::FontId;
pub use text::FontRegistry;
pub use text::TextSystem;
pub use timer::HighResolutionTimer;

pub struct Platform {
    #[allow(unused)]
    sdl_context: sdl3::Sdl,
    display_system: DisplaySystem,
    event_pump: EventPump,
    text_system: TextSystem,
}

impl Platform {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl3::init()?;

        let display_system = display::configure()
            .window(|w| w.title("Snake").size((640, 480)))
            .rendering(|r| r.alpha_blending())
            .build(&sdl_context)?;

        let event_pump = event::EventPump::init(&sdl_context)?;

        let text_system = text::TextSystem::init()?;

        Ok(Self {
            sdl_context,
            display_system,
            event_pump,
            text_system,
        })
    }

    pub fn register_font(&mut self, id: FontId, path: &str) {
        self.text_system.register_font(id, path);
    }

    pub fn timer(&self) -> HighResolutionTimer {
        HighResolutionTimer::new()
    }

    pub fn events(&mut self) -> sdl3::event::EventPollIterator<'_> {
        self.event_pump.iter()
    }

    pub fn display_commands(&mut self) -> DisplayCommands<'_> {
        DisplayCommands::new(&mut self.display_system, &mut self.text_system)
    }
}
