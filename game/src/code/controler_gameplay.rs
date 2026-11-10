use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, renderer::DrawedObject};

const PLAYER_SPEED: f32 = 4.0;
const PLAYER_ROTATION: f32 = 2.0;
const PICK_UP_DISTANCE: f32 = 2.0;

const BULLET_SPEED: f32 = 25.0;
const BULLET_LIFESPAN: f32 = 1.0;

#[derive(Component)]
pub struct PlayerSettings {
    pub rotation: f32
}

pub fn controler_gameplay(
    mut commands: Commands,
    mut input_getan: ResMut<InputGetan>, 
    time: Res<Delta>, 
    mut player: Query<(&mut FreeCamera, &mut DrawedObject, &mut PlayerSettings, &mut PlayerHealth)>,
    mut inventory: Query<&mut InventorySettings>,
    mut droped_items: Query<(Entity, &mut DropedItem)>
) {
    if input_getan.owner == InputOwner::Gameplay {
        if let Ok((mut freecam, mut obj, mut sett, mut health)) = player.single_mut() {
            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_long();
                    position(&mut obj, &mut freecam, time.delta);
                }
            } else if input_getan.now_setting == 3 {
                if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    let is_long = hand_use(&mut commands, &mut inventory, &mut droped_items, 0, &mut obj, &mut health);
                    if is_long {
                        input_getan.reset_long();
                    } else {
                        input_getan.reset_short();
                    }
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    let is_long = hand_use(&mut commands, &mut inventory, &mut droped_items, 4, &mut obj, &mut health);
                    if is_long {
                        input_getan.reset_long();
                    } else {
                        input_getan.reset_short();
                    }
                }
                if ine[0] == false && ine[1] == true && ine[2] == true  { // .__
                    input_getan.reset_long();
                    rotate(-1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                    input_getan.reset_long();
                    rotate(1.0, &mut sett.rotation, &mut obj, time.delta);
                }
                if ine[0] == false && ine[1] == true && ine[2] == false  { // ._.
                    input_getan.reset_short();
                    log::info!("wolne");
                }
                if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
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


fn hand_use(commands: &mut Commands, inventory: &mut Query<&mut InventorySettings>, droped_items: &mut Query<(Entity, &mut DropedItem)>, hand_slot_id: usize, plr: &DrawedObject, health: &mut PlayerHealth) -> bool { // true = long, false = short
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
        let us = Item::use_(&mut sett.item, hand_slot_id, health, commands, plr);
        if us.1 {
            sett.item[hand_slot_id] = Item::None;
        }
        return us.0;
    }

    return false;
}

#[derive(Component)]
pub struct Bullet {
    pub lifespan: f32
}

pub fn bullet_provider(mut commands: Commands, time: Res<Delta>, mut bullets: Query<(Entity, &mut Bullet, &mut DrawedObject)>) {
    for (entity, mut bullet, mut obj) in bullets.iter_mut() {
        bullet.lifespan += time.delta;

        if bullet.lifespan >= BULLET_LIFESPAN {
            commands.entity(entity).despawn();
            continue;
        }

        let forward = obj.rotation * Vec3::Y;

        obj.position += forward * BULLET_SPEED * time.delta;
    }
}