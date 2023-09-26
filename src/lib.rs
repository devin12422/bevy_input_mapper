pub mod input;

use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};
use input::{
    events::{InputActionContinuing, InputActionFinished, InputActionStarted},
    gamepad::GamepadAxis,
    mouse::MouseAxis,
};

use crate::input::events::InputActionActive;

pub trait AutoBinder<K, V>
where
    K: Eq + Hash,
{
    fn bind(&mut self, key: K, value: V) -> &mut Self;
    fn unbind(&mut self, key: K) -> &mut Self;
}

impl<K, V> AutoBinder<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn bind(&mut self, key: K, value: V) -> &mut Self {
        self.insert(key, value);
        self
    }

    fn unbind(&mut self, key: K) -> &mut Self {
        self.remove(&key);
        self
    }
}

#[derive(Default, Clone, Resource)]
pub struct InputMapper {
    pub action_value: HashMap<String, f32>,
    pub previous_action_value: HashMap<String, f32>,

    pub keyboard_binding: HashMap<KeyCode, String>,
    pub mouse_button_binding: HashMap<MouseButton, String>,
    pub mouse_axis_binding: HashMap<MouseAxis, String>,

    pub gamepad_axis_binding: HashMap<GamepadAxis, String>,
    pub gamepad_button_binding: HashMap<GamepadButton, String>,
}

#[derive(Default)]
pub struct InputMapperPlugin;

impl Plugin for InputMapperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputMapper::default())
            .add_event::<InputActionActive>()
            .add_event::<InputActionStarted>()
            .add_event::<InputActionContinuing>()
            .add_event::<InputActionFinished>()
            .add_systems(Update, InputMapper::event_cycle)
            .add_systems(Update, InputMapper::keyboard_key_press_system)
            .add_systems(
                Update,
                (
                    InputMapper::mouse_button_press_system,
                    InputMapper::mouse_axis_move_system,
                ),
            )
            .add_systems(
                Update,
                (
                    InputMapper::gamepad_button_press_system,
                    InputMapper::gamepad_axis_move_system,
                ),
            );
    }
}
