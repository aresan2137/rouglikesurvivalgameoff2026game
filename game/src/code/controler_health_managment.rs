use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, renderer::DrawedObject};

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyPart {
    ArmLeft = 0,
    ArmRight = 1,
    LegLeft = 2,
    LegRight = 3,
    Torso = 4,
    Head = 5,
}

#[derive(Component, Clone, Copy)]
pub struct BodyPartHealth {
    pub infection: f32,
    pub virus: f32,
    pub bleeding: f32,
    pub antiseptic: f32,
    pub soap: f32,
    pub health: f32
}

#[derive(Component)]
pub struct PlayerHealth {
    pub hunger: f32,
    pub thirst: f32,
    pub antibiotic: f32,
    pub surowica: f32,
    pub blod_volume: f32,
    pub body_parts: [BodyPartHealth; 6]
}

// pub fn setup_inventory_contex_menu(world: &mut World) {
    
// }

pub fn controler_health_managment(
    mut input_getan: ResMut<InputGetan>
) {
    if input_getan.owner == InputOwner::HealthManagment {
        let ine = input_getan.morse_input;
        if input_getan.now_setting == 2 {
            if ine[0] == true && ine[1] == true { // __
                input_getan.reset_short();
                log::info!("health managment: __");
            }
            if ine[0] == false && ine[1] == true { // ._
                input_getan.reset_short();
                log::info!("health managment: ._");
            }
        } else if input_getan.now_setting == 3 {
            if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                input_getan.reset_short();
                log::info!("health managment: _._");
            }
            if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                input_getan.reset_short();
                log::info!("health managment: .._");
            }

            if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                input_getan.reset_short();
                log::info!("health managment: _..");
            }
            if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                input_getan.reset_short();
                input_getan.owner = InputOwner::InventoryContextMenu;
            }
        }
    }
}