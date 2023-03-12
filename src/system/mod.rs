pub mod state;

use ggez::{event::EventHandler, GameError, graphics::{self, Color, Rect}, glam::vec2, timer, GameResult};

use crate::{entity::Entity, scece::{Scene, Scenes, DefaultScene}};

use self::state::GameState;

pub struct GameSystem {
    pub state: GameState,
    pub scene: Scenes
}

impl GameSystem {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let mut state = GameState::new(ctx)?;
        let scene = Scenes::DefaultScene(DefaultScene);
        scene.inner().prepare(&mut state);

        Ok( Self { state, scene })
    }
}


impl EventHandler<GameError> for GameSystem {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {
            self.scene.inner().tick(&mut self.state);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.2, 1.0]));

        for entity in self.state.entities.iter_entity() {
            let mut canvas_image = graphics::ScreenImage::new(ctx, None, 1.0, 1.0, 1);
            let mut img_canvas = graphics::Canvas::from_image(
                ctx,
                canvas_image.image(ctx),
                graphics::Color::from_rgba(0, 0, 0, 0) 
            );

            let area = entity.inner().draw(&mut img_canvas, &self.state)?;

            img_canvas.finish(ctx)?;

            canvas.draw(
                &canvas_image.image(ctx),
                graphics::DrawParam::new()
                    .src(Rect::new(0.0, 0.0, area.w / 1920.0, area.y / 1080.0))   
                    .dest(vec2(area.x, area.y))
                    .color(Color::from((255, 255, 255, 128)))
            );
        }

        canvas.finish(ctx)?;

        timer::yield_now();
        Ok(())
    }
}
