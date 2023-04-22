use ggez::{Context, winit::event::VirtualKeyCode, input::keyboard::KeyboardContext};

use crate::lang::{ModKey, ProgramEnvironment, api::APIError, ClientError};

macro_rules! generate_keybind {
    ( $value: expr => $($key: ident),+ ) => {
        match $value {
            $( stringify!($key) => Some(VirtualKeyCode::$key), )+
            _ => None
        }
    }
}

#[inline]
fn map_char_to_keycode(key: &str) -> Option<VirtualKeyCode> {
    let key = key.to_uppercase();
    let key = key.as_str();

    let matched = generate_keybind!(
        key => A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
    );
    if matched.is_some() {
        return matched;
    }

    match key {
        " " => Some(VirtualKeyCode::Space),
        "SPACE" => Some(VirtualKeyCode::Space),
        _ => None
    }
}

pub struct Environment<'ctx> {
    keyboard_ctx: &'ctx KeyboardContext,
}
impl ProgramEnvironment for Environment<'_> {
    fn is_pressed(&self, char: &str, mods: ModKey) -> Result<bool, ClientError> {
        let keycode = map_char_to_keycode(char)
            .ok_or(ClientError::ValidationFailure {
                performing: "Key press check".to_string(),
                part: "char".to_string(),
                reason: format!("No such key: {char}")
            })?;

        // dbg!(&keycode, self.keyboard_ctx.is_key_pressed(keycode));

        Ok(self.keyboard_ctx.is_key_pressed(keycode))
    }
}
impl<'ctx> Environment<'ctx> {
    pub fn new(keyboard_ctx: &'ctx KeyboardContext) -> Self {
        Self { keyboard_ctx }
    }
}
