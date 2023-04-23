#![allow(unused)]
#![allow(irrefutable_let_patterns)]

pub mod entity;
pub mod gui;
pub mod lang;
pub mod system;
pub mod theory;
pub mod traitext;
pub mod world;

use std::{env, path::PathBuf};

use ggez::{
    conf::{Conf, WindowMode},
    event, ContextBuilder,
};
use system::GameSystem;

#[tokio::main]
async fn main() {
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

    let system = GameSystem::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, system);
}
