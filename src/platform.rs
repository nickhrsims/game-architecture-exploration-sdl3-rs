use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Platform {
    pub config: Config,
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub events: EventPump,
}

pub struct Config {
    pub window_title: &'static str,
    pub window_width: u32,
    pub window_height: u32,
}

impl Platform {
    /// Initialize the platform, consuming the configuration.
    pub fn init(config: Config) -> Self {
        let sdl = sdl3::init().expect("SDL3 initializes");
        let video = sdl.video().expect("SDL3 Video Subsystem initializes");
        let window = video
            .window(
                config.window_title,
                config.window_width,
                config.window_height,
            )
            .resizable()
            .position_centered()
            .build()
            .expect("SDL3 Window is created");

        let canvas: Canvas<Window> = window.into_canvas();

        let events = sdl
            .event_pump()
            .expect("SDL3 Event Pump is ready and acquirable");

        Self {
            config,
            sdl,
            canvas,
            events,
        }
    }
}
