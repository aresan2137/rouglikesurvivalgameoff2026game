use bevy_ecs::prelude::*;
use smq_engine::*;
use winit::keyboard::KeyCode;

const LONG_DURATION: f32 = 0.1;
const RESET_ALL_DURATION: f32 = 0.5;
const MEMORY_LENGTH: usize = 4;

#[derive(PartialEq, Eq)]
pub enum InputOwner {
    Gameplay,
    Inventory
}

#[derive(Resource)]
pub struct InputGetan {
    pub morse_input: [bool; MEMORY_LENGTH], // true = _ false = .
    pub now_setting: usize,
    pub still_holding: bool,
    pub owner: InputOwner,
    time_secs: f32,
    none_timer: f32
}

impl InputGetan {
    pub fn reset_colective(&mut self) {
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
    }

    if input_getan.now_setting == 0 {
        input_getan.none_timer = 0.0;
    }

    egui::Window::new("Input")
        .default_width(400.0)
        .show(&ui(), |ui| {
            let mut text = "".to_string();
            for i in 0..input_getan.now_setting {
                text += if input_getan.morse_input[i] { "_" } else { "." };
            }
            ui.label(format!("code: {}", text));
            ui.label(format!("still holding: {}", input_getan.still_holding));
        });
}
