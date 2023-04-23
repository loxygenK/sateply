pub mod keyinput_list;
pub mod state;

use ggez::graphics::{DrawParam, ScreenImage};
use ggez::{event::EventHandler, glam::vec2, graphics::{self, Color, Rect, StrokeOptions}, mint::Point2, GameError, GameResult, Context};
use std::collections::HashMap;

use crate::world::{World, WorldKey, WorldValue};
use crate::{entity::Entity, extract_by_entity, scece::{DefaultScene, SceneTickAction, Scenes}};
use crate::entity::DrawOrigin;
use crate::gui::GUIEntity;
use crate::lang::exec::LuaProgramExecutor;
use crate::scece::game::lang_env::Environment;
use crate::traitext::ExpectOnlyOneExt;

use self::state::{GameState, KeyPressTiming};

pub struct GameSystem {
    pub world: World,
    pub gui: GUIEntity,
    pub state: GameState,
    pub lua: LuaProgramExecutor
}

impl GameSystem {
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Self> {
        Ok(Self {
            world: World::default(),
            state: GameState::new(ctx)?,
            gui: GUIEntity::new(ctx),
            lua: LuaProgramExecutor::new(),
        })
    }

    fn update_entities(&mut self, ctx: &mut ggez::Context) {
        self.world
            .update_all_entity(ctx, &mut self.state.physical_world)
            .unwrap();
    }

    fn update_lua(&mut self, ctx: &mut ggez::Context) {
        if let Some(program) = &self.state.next_lua_program {
            self.lua.load(&program);
            self.state.next_lua_program = None;
        }

        let mut satelite = extract_by_entity!(mut self.world, Satellite)
            .unwrap_only_one();

        let result = self.lua.execute(satelite, &Environment::new(&ctx.keyboard));

        #[cfg(debug_assertions)]
        if let Err(err) = result {
            println!("{err}");
        }
    }
}

impl EventHandler<GameError> for GameSystem {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        self.gui.update(&mut self.state, ctx)?;

        while ctx.time.check_update_time(60) {
            self.update_lua(ctx);
            self.update_entities(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            Color::from([0.0, 0.0, 0.2, 1.0])
        );

        self.world.iter_mut_entity().try_for_each(
            |WorldValue {
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

        self.gui.draw(&mut canvas, &self.state)?;
        canvas.finish(ctx)
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> Result<(), GameError> {
        self.gui.on_text_input(character);
        Ok(())
    }
}
