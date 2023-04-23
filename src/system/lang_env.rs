use ggez::{input::keyboard::KeyboardContext, winit::event::VirtualKeyCode};

use crate::lang::{ClientError, ModKey, ProgramEnvironment};

macro_rules! match_keycode {
    ( $value: expr => $($key: ident),+ ) => {
        (match $value {
            $( stringify!($key) => Some(VirtualKeyCode::$key), )+
            _ => None
        })
    }
}

fn map_char_to_keycode(key: &str) -> Option<VirtualKeyCode> {
    let key = key.to_uppercase();
    let key = key.as_str();

    if let Some(matched) = match_keycode!(
        key => A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
    ) {
        return Some(matched);
    }

    match key {
        " " => Some(VirtualKeyCode::Space),
        "SPACE" => Some(VirtualKeyCode::Space),
        _ => None,
    }
}

pub struct Environment<'ctx> {
    keyboard_ctx: &'ctx KeyboardContext,
}
impl ProgramEnvironment for Environment<'_> {
    fn is_pressed(&self, char: &str, _mods: ModKey) -> Result<bool, ClientError> {
        let keycode = map_char_to_keycode(char).ok_or(ClientError::ValidationFailure {
            performing: "Key press check".to_string(),
            part: "char".to_string(),
            reason: format!("No such key: {char}"),
        })?;

        Ok(self.keyboard_ctx.is_key_pressed(keycode))
    }
}
impl<'ctx> Environment<'ctx> {
    pub fn new(keyboard_ctx: &'ctx KeyboardContext) -> Self {
        Self { keyboard_ctx }
    }
}
