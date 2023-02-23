pub mod entity;

use std::{time::Duration, path::PathBuf, env};

use ggez::{event::{EventHandler, self}, GameError, conf::{Conf, WindowMode}, ContextBuilder, graphics::{self, Color, Rect}, glam::{Vec2, vec2}, timer, GameResult};

struct GameState {
    dt: Duration,
    frame: usize,
    satelite_svg: graphics::Image
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let satelite_svg = graphics::Image::from_path(ctx, "/imgs/satelite.png")?;

        Ok(
            Self {
                dt: Duration::default(),
                frame: 0,
                satelite_svg
            }
        )
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {
            self.frame += 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.2, 1.0]));

        canvas.draw(
            graphics::Text::new("Hello, world!").set_scale(32.0),
            graphics::DrawParam::from(Vec2::new(10.0, 10.0)).color(Color::WHITE)
        );

        canvas.draw(
            graphics::Text::new(format!("Frame: {}", self.frame)).set_scale(24.0),
            graphics::DrawParam::from(Vec2::new(10.0, 40.0)).color(Color::WHITE)
        );

        canvas.draw(
            &self.satelite_svg,
            graphics::DrawParam::from(Vec2::new(100.0, 100.0)).color(Color::WHITE).scale(Vec2::new(0.5, 0.5))
        );

        // let canvas_image = graphics::Image::new_canvas_image(ctx, ctx.gfx.surface_format(), 320, 320, 1);
        let mut canvas_image = graphics::ScreenImage::new(ctx, None, 1.0, 1.0, 1);

        {
            let canvas_image = canvas_image.image(ctx);
            let mut img_canvas = graphics::Canvas::from_image(
                ctx,
                canvas_image,
                graphics::Color::from((255, 255, 255, 128)),
            );

            img_canvas.draw(
                graphics::Text::new(format!("Frame: {}", self.frame)).set_scale(256.0),
                graphics::DrawParam::from(vec2(0.0, 0.0) + vec2(15., 50.))
                    .color(Color::from((0, 0, 0, 255))),
            );
            img_canvas.finish(ctx)?;
        }

        let canvas_image = canvas_image.image(ctx);
        canvas.draw(
            &canvas_image,
            graphics::DrawParam::new()
                .src(Rect::new(0.0, 0.0, 100.0 / 1920.0, 100.0 / 1080.0))   
                .dest(vec2(100.0, 100.0))
                .color(Color::from((255, 255, 255, 128)))
        );


        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}

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

    let state = GameState::new(&mut ctx).unwrap();

    event::run(ctx, event_loop, state);
}
