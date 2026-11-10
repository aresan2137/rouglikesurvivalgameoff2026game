use bevy_ecs::prelude::*;
use glam::Vec2;
use smq_engine::*;
use winit::keyboard::KeyCode;

use crate::renderer::{draw_free_left_text};

const LONG_DURATION: f32 = 0.1;
const RESET_ALL_DURATION: f32 = 0.4;
const MEMORY_LENGTH: usize = 4;

#[derive(PartialEq, Eq, Debug)]
pub enum InputOwner {
    Gameplay,
    Inventory,
    InventoryContextMenu,
    HealthManagment
}

impl InputOwner {
    fn command_list(&self) -> &str {
        return match self {
            InputOwner::Gameplay => { "__ = Walk\n_._ = Main Use\n_.. = Second Use\n.__ = Rotate Right\n._. = ???\n.._ = Rotate Left\n... = Inventory" }
            InputOwner::Inventory => { "__ = Take\n_._ = Contex Menu\n_.. = Simple Use\n.__ = Rotate Right\n._. = Drop\n.._ = Rotate Left\n... = Gameplay" }
            InputOwner::InventoryContextMenu => { "... = Inventory" }
            InputOwner::HealthManagment => { "... = Inventory" }
        };
    }
}

#[derive(Resource)]
pub struct InputGetan {
    pub morse_input: [bool; MEMORY_LENGTH], // true = _ false = .
    pub now_setting: usize,
    pub still_holding: bool,
    pub owner: InputOwner,
    request_clear: bool,
    time_secs: f32,
    none_timer: f32
}

impl InputGetan {
    pub fn reset_long(&mut self) {
        self.request_clear = true;
    }

    pub fn reset_short(&mut self) {
        self.request_clear = true;
        self.now_setting = 0;
        self.still_holding = false;
    }
}

impl Default for InputGetan {
    fn default() -> Self {
        return Self { 
            morse_input: [false; MEMORY_LENGTH], 
            now_setting: 0, 
            still_holding: false,
            owner: InputOwner::Gameplay,
            request_clear: false,
            time_secs: 0.0,
            none_timer: 0.0
        };
    }
}

pub fn read_input(mut input_getan: ResMut<InputGetan>, input: Res<Inputs>, time: Res<Delta>) {
    let is_down = input.down(KeyCode::ShiftLeft) || input.down(KeyCode::ShiftRight);
    
    if input_getan.time_secs > LONG_DURATION && !input_getan.still_holding {
        let mut i = input_getan.now_setting;
        if i >= MEMORY_LENGTH {
            i = 0;
            input_getan.now_setting = 0;
        }
        input_getan.morse_input[i] = true;
        input_getan.now_setting += 1;
        input_getan.still_holding = true;
    }

    if !is_down && input_getan.time_secs > 0.0 && input_getan.time_secs < LONG_DURATION {
        let mut i = input_getan.now_setting;
        if i >= MEMORY_LENGTH {
            i = 0;
            input_getan.now_setting = 0;
        }
        input_getan.morse_input[i] = false;
        input_getan.now_setting += 1;
    }

    if input_getan.none_timer > RESET_ALL_DURATION {
        input_getan.now_setting = 0;
    }

    if is_down {
        input_getan.time_secs += time.delta;
        input_getan.none_timer = 0.0;
    } else {
        input_getan.time_secs = 0.0;
        input_getan.none_timer += time.delta;
        input_getan.still_holding = false;
        if input_getan.request_clear {
            input_getan.request_clear = false;
            input_getan.now_setting = 0;
        }
    }

    if input_getan.now_setting == 0 {
        input_getan.none_timer = 0.0;
    }

    let mut text = "".to_string();
    for i in 0..input_getan.now_setting {
        text += if input_getan.morse_input[i] { "_" } else { "." };
    }

    draw_free_left_text(&format!("code: {}\nstill holding: {}\nmode: {:?}", text, input_getan.still_holding, input_getan.owner), Vec2::new(-1.0, -0.8), Vec2::new(4.0, 0.0), 24.0);
    draw_free_left_text(input_getan.owner.command_list(), Vec2::new(-1.0, -0.8), Vec2::new(4.0, 180.0), 24.0);
}
