use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, renderer::DrawedObject};

const PLAYER_SPEED: f32 = 2.0;
const PLAYER_ROTATION: f32 = 2.0;

#[derive(Component)]
pub struct PlayerSettings {
    pub rotation: f32
}

pub fn controler_gameplay(mut input_getan: ResMut<InputGetan>, time: Res<Delta>, mut player: Query<(&mut FreeCamera, &mut DrawedObject, &mut PlayerSettings)>) {
    if input_getan.owner == InputOwner::Gameplay {
        if let Ok((mut freecam, mut obj, mut sett)) = player.single_mut() {
            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_long();
                    position(&mut obj, &mut freecam, time.delta);
                }
                if ine[0] == false && ine[1] == true { // ._
                    input_getan.reset_long();
                    log::info!("shoot");
                }
            } else if input_getan.now_setting == 3 {
                if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    input_getan.reset_long();
                    rotate(-1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                    input_getan.reset_long();
                    rotate(1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    input_getan.reset_short();
                    input_getan.owner = InputOwner::Inventory;
                    log::info!("inventory ON");
                }
                if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                    input_getan.reset_short();
                    log::info!("use");
                }
            }
        }
    }
}

fn rotate(order: f32, rotval: &mut f32, obj: &mut DrawedObject, delta: f32) {
    *rotval += order * delta * PLAYER_ROTATION;

    obj.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, *rotval);
}

fn position(obj: &mut DrawedObject, freecam: &mut FreeCamera, delta: f32) {
    let base_forward = Vec3::new(0.0, 1.0, 0.0);

    let forward = obj.rotation * base_forward;

    let move_vector = forward * PLAYER_SPEED * delta;

    obj.position += move_vector;
    freecam.position += move_vector;
}