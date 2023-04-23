use std::fmt::{Debug, Formatter};
use std::io::Read;
use ggegui::{egui, Gui};
use ggez::{Context, GameResult};
use ggez::context::Has;
use ggez::glam::{Vec2, vec2};
use ggez::graphics::{Canvas, DrawParam};
use crate::entity::{DrawInstruction, DrawOrigin, Entity, TypedEntity};
use crate::file_selector::FileDialog;
use crate::system::state::GameState;

pub struct GUIEntity {
    gui: Gui,
    file_dialog: FileDialog
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
            file_dialog: FileDialog::default()
        }
    }
}

impl GUIEntity {
    pub fn update(&mut self, state: &mut GameState, ctx: &mut Context) -> GameResult {
        let gui_ctx = self.gui.ctx();

        egui::Window::new("Load program").show(&gui_ctx, |ui| {
            ui.label("Path of the program");
            if ui.button("Open the program").clicked() {
                self.file_dialog.show();
            }
        });
        self.gui.update(ctx);

        if let Some(program) = self.file_dialog.read_selected() {
            state.load_lua_program(&program);
            self.file_dialog.forget_selected();
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut Canvas, state: &GameState) -> GameResult<DrawInstruction> {
        canvas.draw(
            &self.gui,
            DrawParam::default().dest(Vec2::ZERO),
        );

        Ok(DrawInstruction {
            size: (1920.0, 1080.0).into(),
            draw_origin: DrawOrigin::ScreenAbsolute,
            position: Vec2::ZERO,
            angle: 0.0
        })
    }

    pub fn on_text_input(
        &mut self,
        character: char,
    ) {
        self.gui.input.text_input_event(character);
    }
}