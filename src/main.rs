pub mod app;
pub mod game;
pub mod geom;
pub mod platform;

use app::App;
use app::Config;

fn main() {
    App::init(Config {
        name: "Snake",
        grid_width: 20,
        grid_height: 15,
        grid_scale: 32,
        fps: 10,
    })
    .expect("App initializes")
    .run();
}
