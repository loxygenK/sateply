use ggez::{input::keyboard::KeyCode, Context};

use crate::system::keyinput_list::{KeyTypeMatch, KeyTypeMatchMap};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Control {
    EnableBurst,
    DisableBurst,
    Bursting,
}

impl Control {
    pub fn get_binding() -> KeyTypeMatchMap<Control> {
        KeyTypeMatchMap::new([
            (KeyTypeMatch::JustNow(KeyCode::W), Control::EnableBurst),
            (KeyTypeMatch::JustNow(KeyCode::S), Control::DisableBurst),
            (KeyTypeMatch::Holded(KeyCode::Space), Control::Bursting),
        ])
    }
}
