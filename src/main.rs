pub mod entity;
pub mod system;
pub mod scece;
pub mod utils;

use std::{path::PathBuf, env};

use entity::{satelite::Satelite, Entity};
use ggez::{conf::{Conf, WindowMode}, event, ContextBuilder};
use system::GameSystem;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        PathBuf::from("./assets")
    };

    let config = Conf::new().window_mode(WindowMode {
        width: 1920.0,
        height: 1080.0,
        ..WindowMode::default()
    });
    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(config)
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let mut system = GameSystem::new(&mut ctx).unwrap();
    system.state.entities.insert(Satelite { x: 10.0, y: 10.0 }.typed());

    event::run(ctx, event_loop, system);
}
