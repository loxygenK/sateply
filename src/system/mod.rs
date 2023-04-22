pub mod keyinput_list;
pub mod state;

use ggez::graphics::{DrawParam, ScreenImage};
use ggez::{event::EventHandler, glam::vec2, graphics::{self, Color, Rect, StrokeOptions}, mint::Point2, GameError, GameResult, Context};
use std::collections::HashMap;

use crate::entity::map::{EntityMap, EntityMapKey, EntityMapValue};
use crate::{
    entity::Entity,
    scece::{DefaultScene, SceneTickAction, Scenes},
};
use crate::entity::DrawOrigin;
use crate::gui::GUIEntity;

use self::state::{GameState, KeyPressTiming};

pub struct GameSystem {
    pub entities: EntityMap,
    pub ui: GUIEntity,
    pub state: GameState,
    pub scene: Scenes,
    pub images: HashMap<EntityMapKey, ScreenImage>,
    pub frame_buffer: ScreenImage,
    pub frame_count: u64,
}

impl GameSystem {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        let mut state = GameState::new(ctx)?;
        let mut entities = EntityMap::default();
        let mut scene = Scenes::DefaultScene(DefaultScene);
        scene.inner_mut().prepare(ctx, &mut state, &mut entities);

        Ok(Self {
            entities,
            state,
            scene,
            ui: GUIEntity::new(ctx),
            images: HashMap::new(),
            frame_buffer: graphics::ScreenImage::new(&ctx.gfx, None, 1.0, 1.0, 1),
            frame_count: 0
        })
    }

    pub fn mut_state(&mut self) -> &mut GameState {
        &mut self.state
    }
}

impl EventHandler<GameError> for GameSystem {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(60) {
            self.ui.update(ctx)?;

            let Some(action) = self.scene.inner_mut().tick(ctx, &mut self.state, &mut self.entities) else { continue; };
            match action {
                SceneTickAction::ChangeScene(scene) => {
                    self.scene = scene;
                    self.scene
                        .inner_mut()
                        .prepare(ctx, &mut self.state, &mut self.entities);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let buf_image_instance = self.frame_buffer.image(&ctx.gfx);
        let mut canvas = graphics::Canvas::from_image(
            &ctx.gfx,
            buf_image_instance.clone(),
            Color::from([0.0, 0.0, 0.2, 1.0])
        );

        self.entities.iter_mut_entity().try_for_each(
            |EntityMapValue {
                 entity,
                 ref mut screen_image,
             }| {
                let mut img_canvas = graphics::Canvas::from_image(
                    ctx,
                    screen_image.image(ctx),
                    graphics::Color::from_rgba(0, 0, 0, 0),
                );

                let draw = entity.inner().draw(&mut img_canvas, &self.state)?;

                img_canvas.finish(ctx)?;

                let offset = vec2(draw.size.x / 2.0, draw.size.y / 2.0);
                let offset_screen = match draw.draw_origin {
                    DrawOrigin::World => vec2(1920.0 / 2.0, 1080.0 / 2.0),
                    DrawOrigin::ScreenAbsolute => vec2(0.0, 0.0)
                };

                canvas.draw(
                    &screen_image.image(ctx),
                    graphics::DrawParam::new()
                        .src(Rect::new(
                            0.0,
                            0.0,
                            draw.size.x / 1920.0,
                            draw.size.y / 1080.0,
                        ))
                        .dest(draw.position + offset + offset_screen)
                        .rotation(draw.angle)
                        .offset(Point2 { x: 0.5, y: 0.5 }), // .color(Color::from((255, 255, 255, 128)))
                );

                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        &ctx.gfx,
                        graphics::DrawMode::Stroke(StrokeOptions::default()),
                        Rect::new(draw.position.x, draw.position.y, draw.size.x, draw.size.y),
                        Color::RED,
                    )?,
                    graphics::DrawParam::from(offset_screen),
                );

                GameResult::Ok(())
            },
        )?;

        self.ui.draw(&mut canvas, &self.state)?;

        canvas.finish(ctx)?;

        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.draw(&buf_image_instance, DrawParam::default());
        canvas.finish(ctx)
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        self.ui.on_text_input(character);
        Ok(())
    }
}
