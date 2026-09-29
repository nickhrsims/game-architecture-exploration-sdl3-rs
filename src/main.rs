pub mod app;
pub mod game;
pub mod grid;
pub mod platform;

use app::App;
use app::Config;

fn main() {
    App::new(Config {
        name: "Snake",
        grid_width: 24,
        grid_height: 24,
        grid_scale: 32,
        fps: 10,
    })
    .run();
}
