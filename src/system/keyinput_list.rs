use std::collections::HashMap;

use ggez::{input::keyboard::KeyCode, Context};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum KeyTypeMatch {
    JustNow(KeyCode),
    JustNowAllowRepeat(KeyCode),
    Holded(KeyCode),
    Released(KeyCode),
}

pub struct KeyTypeMatchMap<C: Clone>(HashMap<KeyTypeMatch, C>);
impl<C: Clone> KeyTypeMatchMap<C> {
    pub fn new<const N: usize>(matches: [(KeyTypeMatch, C); N]) -> Self {
        KeyTypeMatchMap(HashMap::from(matches))
    }

    pub fn get_active_controls(&self, ctx: &Context) -> Vec<C> {
        self.0
            .iter()
            .filter(|(key, _)| is_typed(ctx, **key))
            .map(|(_, control)| control.clone())
            .collect::<Vec<_>>()
    }
}

pub fn is_typed(ctx: &Context, typed_key: KeyTypeMatch) -> bool {
    use KeyTypeMatch::*;

    match typed_key {
        JustNow(code) => ctx.keyboard.is_key_just_pressed(code) && !ctx.keyboard.is_key_repeated(),
        JustNowAllowRepeat(code) => ctx.keyboard.is_key_just_pressed(code),
        Holded(code) => ctx.keyboard.is_key_pressed(code),
        Released(code) => ctx.keyboard.is_key_just_released(code),
    }
}
