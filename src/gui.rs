use std::fmt::{Debug, Formatter};
use ggegui::{egui, Gui};
use ggez::{Context, GameResult};
use ggez::glam::{Vec2, vec2};
use ggez::graphics::{Canvas, DrawParam};
use crate::entity::{DrawInstruction, DrawOrigin, Entity, TypedEntity};
use crate::system::state::GameState;

pub struct GUIEntity {
    gui: Gui,
    typed: String
}

impl Debug for GUIEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GUIEntity")
    }
}

impl GUIEntity {
    pub fn new(ctx: &Context) -> Self {
        GUIEntity {
            gui: Gui::new(ctx),
            typed: String::new(),
        }
    }
}

impl GUIEntity {
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        egui::Window::new("Load program").show(&gui_ctx, |ui| {
            ui.label("Path of the program");
            let textbox = ui.add(egui::TextEdit::singleline(&mut self.typed));
            if textbox.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                println!("Typed: {}", self.typed);
            }
            if ui.button("button").clicked() {
                println!("button clicked");
            }
        });
        self.gui.update(ctx);

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<DrawInstruction> {
        canvas.draw(
            &self.gui,
            DrawParam::default(),
        );

        Ok(DrawInstruction {
            size: (1920.0, 1080.0).into(),
            draw_origin: DrawOrigin::ScreenAbsolute,
            ..Default::default()
        })
    }

    pub fn on_text_input(
        &mut self,
        character: char,
    ) {
        self.gui.input.text_input_event(character);
    }
}