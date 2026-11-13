use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, items::*, renderer::DrawedObject};

const PLAYER_SPEED: f32 = 8.0;
const PLAYER_ROTATION: f32 = 2.0;
const PICK_UP_DISTANCE: f32 = 2.0;

#[derive(Component)]
pub struct PlayerSettings {
    pub rotation: f32
}

pub fn controler_gameplay(
    mut commands: Commands,
    mut input_getan: ResMut<InputGetan>, 
    time: Res<Delta>, 
    mut player: Query<(&mut FreeCamera, &mut DrawedObject, &mut PlayerSettings)>,
    mut inventory: Query<&mut InventorySettings>,
    mut droped_items: Query<(Entity, &mut DropedItem)>,
    mut use_item_writer: MessageWriter<UseItemMessage>,
) {
    if input_getan.owner == InputOwner::Gameplay {
        if let Ok((mut freecam, mut obj, mut sett)) = player.single_mut() {
            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_long();
                    position(&mut obj, &mut freecam, time.delta);
                }
                if ine[0] == false && ine[1] == true { // ._
                    if hand_use(&mut commands, &mut inventory, &mut droped_items, 0, &mut obj, &mut use_item_writer) {
                        input_getan.reset_long();
                    } else {
                        input_getan.reset_short();
                    }
                }
            } else if input_getan.now_setting == 3 {
                if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                    if hand_use(&mut commands, &mut inventory, &mut droped_items, 4, &mut obj, &mut use_item_writer) {
                        input_getan.reset_long();
                    } else {
                        input_getan.reset_short();
                    }
                }
                if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                    input_getan.reset_long();
                    rotate(1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    input_getan.reset_long();
                    rotate(-1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    input_getan.reset_short();
                    input_getan.owner = InputOwner::Inventory;
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

fn hand_use(commands: &mut Commands, inventory: &mut Query<&mut InventorySettings>, droped_items: &mut Query<(Entity, &mut DropedItem)>, hand_slot_id: usize, plr: &DrawedObject, mut use_item_writer: &mut MessageWriter<UseItemMessage>,) -> bool { // true = long, false = short
    let Ok(mut sett) = inventory.single_mut() else {
        return false;
    };

    if sett.item[hand_slot_id] == Item::None {
        let mut nearest: Option<(Entity, f32)> = None;

        for (entity, drop) in droped_items.iter() {
            let dist = drop.position.distance(plr.position.truncate());
            if dist < PICK_UP_DISTANCE {
                if let Some((_, best_dist)) = nearest {
                    if dist < best_dist {
                        nearest = Some((entity, dist));
                    }
                } else {
                    nearest = Some((entity, dist));
                }
            }
        }

        if let Some((entity, _)) = nearest {
            if let Ok((_, mut drop)) = droped_items.get_mut(entity) {
                sett.item[hand_slot_id] = std::mem::replace(&mut drop.item, Item::None);
                
                commands.entity(entity).despawn();
                return false;
            }
        }
    } else {
        return Item::use_(&mut use_item_writer, vec![hand_slot_id], &sett.item[hand_slot_id]);
    }

    return false;
}