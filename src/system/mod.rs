use ggez::{event::EventHandler, GameError, graphics::{self, Color, Rect}, glam::vec2, timer, GameResult};

use crate::{entity::{EntityMap, Entity}, scece::{Scene, Scenes, DefaultScene}};

pub struct GameState {
    pub entities: EntityMap,
    pub satelite_svg: graphics::Image,
    pub scene: Scenes
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let satelite_svg = graphics::Image::from_path(ctx, "/imgs/satelite.png")?;

        Ok(
            Self {
                entities: EntityMap::default(),
                satelite_svg,
                scene: Scenes::DefaultScene(DefaultScene)
            }
        )
    }
}


impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {
            self.scene.inner().prepare(&mut self);

            // TODO: Make Gamestate splitted between scene and other information

        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.2, 1.0]));

        for entity in self.entities.iter_entity() {
            let mut canvas_image = graphics::ScreenImage::new(ctx, None, 1.0, 1.0, 1);
            let img_canvas = graphics::Canvas::from_image(
                ctx,
                canvas_image.image(ctx),
                graphics::Color::from_rgba(0, 0, 0, 0) 
            );

            let size = entity.inner().draw(&mut canvas, self)?;

            img_canvas.finish(ctx)?;

            canvas.draw(
                &canvas_image.image(ctx),
                graphics::DrawParam::new()
                    .src(Rect::new(0.0, 0.0, size.x / 1920.0, size.y / 1080.0))   
                    .dest(vec2(100.0, 100.0))
                    .color(Color::from((255, 255, 255, 128)))
            );
        }

        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}
