pub mod state;
pub mod keyinput_list;

use std::collections::HashMap;
use ggez::{event::EventHandler, GameError, graphics::{self, Color, Rect, StrokeOptions}, glam::vec2, GameResult, mint::Point2};
use ggez::graphics::ScreenImage;

use crate::{entity::Entity, scece::{Scenes, DefaultScene, SceneTickAction}};
use crate::entity::map::{EntityMap, EntityMapKey, EntityMapValue};

use self::state::{GameState, KeyPressTiming};

pub struct GameSystem {
    pub entities: EntityMap,
    pub state: GameState,
    pub scene: Scenes,
    pub images: HashMap<EntityMapKey, ScreenImage>
}

impl GameSystem {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let mut state = GameState::new(ctx)?;
        let mut entities = EntityMap::default();
        let scene = Scenes::DefaultScene(DefaultScene);
        scene.inner().prepare(ctx, &mut state, &mut entities);

        Ok( Self {
            entities,
            state,
            scene,
            images: HashMap::new(),
        })
    }

    pub fn mut_state(&mut self) -> &mut GameState {
        &mut self.state
    }
}


impl EventHandler<GameError> for GameSystem {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {
            let Some(action) = self.scene.inner().tick(ctx, &mut self.state, &mut self.entities) else { continue; };
            match action {
                SceneTickAction::ChangeScene(scene) => {
                    self.scene = scene;
                    self.scene.inner().prepare(ctx, &mut self.state, &mut self.entities);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.2, 1.0]));

        let map = &mut self.entities;

        map
            .iter_mut_entity()
            .try_for_each(|EntityMapValue { entity, ref mut screen_image }| {
                let mut img_canvas = graphics::Canvas::from_image(
                    ctx,
                    screen_image.image(ctx),
                    graphics::Color::from_rgba(0, 0, 0, 0)
                );

                let draw = entity.inner().draw(ctx, &mut img_canvas, &self.state)?;

                img_canvas.finish(ctx)?;

                let offset = vec2(draw.size.x / 2.0, draw.size.y / 2.0);
                canvas.draw(
                    &screen_image.image(ctx),
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

                GameResult::Ok(())
            })?;

        canvas.finish(ctx)
    }
}
