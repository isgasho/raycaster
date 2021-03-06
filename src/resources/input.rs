use std::collections::HashMap;

use quicksilver::prelude::*;

use crate::resources::input::Binding::*;

#[derive(PartialEq, Eq, Hash)]
pub enum Binding {
    MoveForward,
    MoveBack,
    StrafeLeft,
    StrafeRight,
    TurnLeft,
    TurnRight,
    Action,
}

#[derive(Default)]
pub struct Input {
    keys: HashMap<Binding, bool>,
}

impl Input {
    pub fn update(&mut self, window: &Window) {
        let keyboard = window.keyboard();
        let any_down =
            |bindings: &[Key]| bindings.iter().map(|k| keyboard[*k]).any(|bs| bs.is_down());

        self.keys.insert(MoveForward, any_down(&[Key::W, Key::Up]));
        self.keys.insert(MoveBack, any_down(&[Key::S, Key::Down]));
        self.keys.insert(StrafeLeft, any_down(&[Key::A]));
        self.keys.insert(StrafeRight, any_down(&[Key::D]));
        self.keys.insert(TurnLeft, any_down(&[Key::Q, Key::Left]));
        self.keys.insert(TurnRight, any_down(&[Key::E, Key::Right]));
        self.keys.insert(Action, any_down(&[Key::Space]));
    }

    pub fn is_down(&self, binding: Binding) -> bool {
        *self.keys.get(&binding).unwrap_or(&false)
    }
}
