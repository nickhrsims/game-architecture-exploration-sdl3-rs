pub mod app;
pub mod game;
pub mod geom;

use app::App;
use app::Config;

fn main() {
    App::init(Config {
        name: "Snake",
        grid_width: 24,
        grid_height: 24,
        grid_scale: 32,
        fps: 10,
    })
    .expect("App initializes")
    .run();
}
