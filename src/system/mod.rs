pub mod state;
pub mod keyinput_list;

use ggez::{event::EventHandler, GameError, graphics::{self, Color, Rect, StrokeOptions}, glam::vec2, GameResult, mint::Point2};

use crate::{entity::Entity, scece::{Scenes, DefaultScene, SceneTickAction}};

use self::state::{GameState, KeyPressTiming};

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
            let Some(action) = self.scene.inner().tick(ctx, &mut self.state) else { continue; };
            match action {
                SceneTickAction::ChangeScene(scene) => {
                    self.scene = scene;
                    self.scene.inner().prepare(&mut self.state);
                }
            }
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

            let draw = entity.inner().draw(&mut img_canvas, &self.state)?;

            img_canvas.finish(ctx)?;

            let offset = vec2(draw.size.x / 2.0, draw.size.y / 2.0);
            canvas.draw(
                &canvas_image.image(ctx),
                graphics::DrawParam::new()
                    .src(Rect::new(0.0, 0.0, draw.size.x / 1920.0, draw.size.y / 1080.0))   
                    .dest(draw.position + offset)
                    .rotation(draw.angle)
                    .offset(Point2 { x: 0.5, y: 0.5 })
                    // .color(Color::from((255, 255, 255, 128)))
            );

            canvas.draw(
                &graphics::Mesh::new_rectangle(
                    &ctx.gfx,
                    graphics::DrawMode::Stroke(StrokeOptions::default()),
                    Rect::new(draw.position.x, draw.position.y, draw.size.x, draw.size.y),
                    Color::RED
                )?,
                graphics::DrawParam::default()
            );

        }

        canvas.finish(ctx)
    }
}
