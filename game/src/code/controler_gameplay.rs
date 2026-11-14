use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use crate::{code::*, items::*, renderer::DrawedObject};

const PLAYER_SPEED: f32 = 8.0;
const PLAYER_ROTATION: f32 = 2.0;
const PICK_UP_DISTANCE: f32 = 2.0;

const CAMERA_SMOOTHNESS: f32 = 8.0;

const PIXELS_PER_UNIT: f32 = 32.0;

const PLAYER_RADIUS: f32 = 0.35 * TILE_SCALE;

#[derive(Component)]
pub struct PlayerSettings {
    pub rotation: f32,
    pub pos: Vec3,
    pub cam_pos: Vec3,
    pub turned_angle: f32
}

pub fn controler_gameplay(
    mut commands: Commands,
    mut input_getan: ResMut<InputGetan>, 
    time: Res<Delta>, 
    mut player: Query<(&mut FreeCamera, &mut DrawedObject, &mut PlayerSettings)>,
    mut inventory: Query<&mut InventorySettings>,
    mut droped_items: Query<(Entity, &mut DropedItem)>,
    mut use_item_writer: MessageWriter<UseItemMessage>,
    level: Res<Level>
) {
    if input_getan.owner == InputOwner::Gameplay {
        if let Ok((mut freecam, mut obj, mut sett)) = player.single_mut() {
            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_long();
                    position(&mut obj, &mut sett, time.delta, &level);
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
                    rotate(1.0, &mut sett, &mut obj, time.delta);
                } else if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    input_getan.reset_long();
                    rotate(-1.0, &mut sett, &mut obj, time.delta);
                } else {
                    sett.turned_angle = 0.0;
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    input_getan.reset_short();
                    input_getan.owner = InputOwner::Inventory;
                }
            }

            let blend = 1.0 - (-CAMERA_SMOOTHNESS * time.delta).exp();
            sett.cam_pos = sett.cam_pos.lerp(sett.pos, blend);

            obj.position = snap_to_pixel(sett.pos);
            freecam.position = snap_to_pixel(sett.cam_pos);
        }
    }
}
fn rotation_speed_multiplier(turned_angle: f32) -> f32 {
    let min_mult = 0.6;
    let max_mult = 1.3;

    let t = (turned_angle / FRAC_PI_2).clamp(0.0, 1.0);
    
    let smooth_t = (t * FRAC_PI_2).sin().powi(2);

    min_mult + (max_mult - min_mult) * smooth_t
}

fn rotate(order: f32, sett: &mut PlayerSettings, obj: &mut DrawedObject, delta: f32,) {
    if order == 0.0 || (order.signum() != sett.turned_angle.signum() && sett.turned_angle != 0.0) {
        sett.turned_angle = 0.0;
    }

    if order != 0.0 {
        let mult = rotation_speed_multiplier(sett.turned_angle.abs());
        let step = order * delta * PLAYER_ROTATION * mult;

        sett.rotation += step;
        sett.turned_angle += step;
    }

    obj.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, sett.rotation);
}
fn can_move_to(pos: Vec2, level: &Level) -> bool {
    let corners = [
        pos + Vec2::new(-PLAYER_RADIUS, -PLAYER_RADIUS),
        pos + Vec2::new( PLAYER_RADIUS, -PLAYER_RADIUS),
        pos + Vec2::new(-PLAYER_RADIUS,  PLAYER_RADIUS),
        pos + Vec2::new( PLAYER_RADIUS,  PLAYER_RADIUS),
    ];

    for corner in corners {
        let element = level.get_element_at_global_pos(corner);
        let collide = element == LevelElement::Wall || element == LevelElement::None;

        if collide {
            return false;
        }
    }

    return true;
}

fn slide_speed_factor(parallel_axis: f32) -> f32 {
    if parallel_axis >= FRAC_1_SQRT_2 {
        return 1.0;
    }

    let t = (parallel_axis / FRAC_1_SQRT_2).clamp(0.0, 1.0);

    (t * FRAC_PI_2).sin().powi(2)
}

fn position(obj: &mut DrawedObject, sett: &mut PlayerSettings, delta: f32, level: &Level) {
    let base_forward = Vec3::new(0.0, 1.0, 0.0);
    let forward = (obj.rotation * base_forward).normalize_or_zero();
    let base_speed = PLAYER_SPEED * delta;

    let mut move_vector = forward * base_speed;

    let can_move_x = can_move_to(Vec2::new(sett.pos.x + move_vector.x, sett.pos.y), level);
    let can_move_y = can_move_to(Vec2::new(sett.pos.x, sett.pos.y + move_vector.y), level);

    if !can_move_x && can_move_y && forward.y != 0.0 {
        let factor = slide_speed_factor(forward.y.abs());
        move_vector.y = forward.y.signum() * base_speed * factor;
    }

    if !can_move_y && can_move_x && forward.x != 0.0 {
        let factor = slide_speed_factor(forward.x.abs());
        move_vector.x = forward.x.signum() * base_speed * factor;
    }

    if can_move_x {
        sett.pos.x += move_vector.x;
    }
    if can_move_y {
        sett.pos.y += move_vector.y;
    }
}

fn snap_to_pixel(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x * PIXELS_PER_UNIT).round() / PIXELS_PER_UNIT,
        (pos.y * PIXELS_PER_UNIT).round() / PIXELS_PER_UNIT,
        pos.z,
    )
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
