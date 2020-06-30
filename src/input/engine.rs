use crate::input::*;
use miniquad::*;

use std::collections::HashMap;

pub struct InputEngine {
    keys: HashMap<KeyCode, ButtonState>,
}
impl InputEngine {
    pub fn new() -> Self {
        InputEngine {
            keys: HashMap::new(),
        }
    }

    #[inline]
    pub fn rollover(&mut self) {
        for (_key, state) in &mut self.keys {
            state.rollover();
        }
    }

    #[inline]
    pub fn get_key_state(&mut self, keycode: KeyCode) -> ButtonState {
        if let Some(key) = self.keys.get(&keycode) {
            return key.clone();
        }
        
        self.keys.insert(keycode, ButtonState::new());
        return self.get_key_state(keycode);
    }

    #[inline]
    pub fn set_key_down(&mut self, keycode: KeyCode, repeat: bool) {
        self.set_key_pressed(keycode, true)
    }

    #[inline]
    pub fn set_key_up(&mut self, keycode: KeyCode) {
        self.set_key_pressed(keycode, false)
    }

    #[inline]
    fn set_key_pressed(&mut self, keycode: KeyCode, is_pressed: bool) {
        if let Some(mut key) = self.keys.get_mut(&keycode) {
            key.is_pressed = is_pressed;
        } else {
            self.keys.insert(keycode, ButtonState::new());
            self.set_key_pressed(keycode, is_pressed);
        }
    }
}