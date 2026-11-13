#[allow(unused_imports)]
use bevy_ecs::prelude::*;
use rand::Rng;

use crate::{code::{BodyPart, Bullet, PlayerHealth}, gen_bake::{Meshes, Textures}, items::*, renderer::DrawedObject};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ItemPos {
    pub pos: Vec<usize>
}

impl ItemPos {
    pub fn get_mut<'a>(&self, items: &'a mut [Item; 7]) -> Option<&'a mut Item> {
        let mut iter = self.pos.iter();
        let first_index = *iter.next()?;
        let mut item = items.get_mut(first_index)?;

        for &index in iter {
            match item {
                Item::Backpack(inside) | Item::PlasticBag(inside) => {
                    item = inside.get_mut(index)?;
                }
                _ => return Some(item)
            }
        }

        return Some(item);
    }
}

#[derive(Message)]
pub struct UseItemMessage {
    pub location: ItemPos
}

pub fn handle_simple_use_item_system(
    mut commands: Commands,
    mut use_events: MessageReader<UseItemMessage>, 
    mut inventory: Query<&mut InventorySettings>, 
    mut player: Query<(&mut PlayerHealth, &DrawedObject)>
) {
    let Ok(mut sett) = inventory.single_mut() else { return };
    let Ok((mut health, obj)) = player.single_mut() else { return };

    for message in use_events.read() {
        if let Some(item) = message.location.get_mut(&mut sett.item) {

            if use_([None, None], item, &mut health, &mut commands, obj) {
                *item = Item::None;
            }
            
        }
    }
}


impl Item {
    pub fn use_(writer: &mut MessageWriter<UseItemMessage>, path: Vec<usize>, item: &Item) -> bool {
        if *item != Item::None {
            writer.write(UseItemMessage {
                location: ItemPos { pos: path }
            });
        }

        return match item {
                Item::None => { false }
                
                // guns
                Item::Glock19(_) => { true }
                Item::AK47(_) => { true }
                Item::Shotgun(_) => { true }

                // ammo
                Item::Ammo9mm(_) => { false }
                Item::Ammo7_62mm(_) => { false }
                Item::Shotgunshells(_) => { false }

                // tools
                Item::Flashlight(_)  => { false }
                Item::Torch(_) => { false }
                Item::Crowbar => { false }
                Item::Lighter(_) => { false }
                Item::Knife => { false }
                Item::FlintAndSteel => { false }

                // food
                Item::ProteinBar => { false }
                Item::Banana => { false }
                Item::Apple => { false }
                Item::Cereal(_) => { false }
                Item::Bread(_) => { false }
                Item::FoodRation(_) => { false }
                Item::RottenFood => { false }

                // health
                Item::Bandage(_) => { false }
                Item::SterileBandage(_) => { false }
                Item::AntiBiotic(_) => { false }
                Item::Antiseptic(_) => { false }
                Item::Soap(_) => { false }
                Item::Syringe(_) => { false }

                // items
                Item::Bottle300ml(_) => { false }
                Item::Bottle500ml(_) => { false }
                Item::Bottle1l(_) => { false }
                Item::Bottle2l(_) => { false }
                Item::Cloth => { false }
                Item::WaterFilter(_) => { false }
                Item::Backpack(_) => { false }
                Item::PlasticBag(_) => { false }
            }
    }
}

fn use_(mut other: [Option<&mut Item>; 2], mut item: &mut Item, health: &mut PlayerHealth, mut commands: &mut Commands, player: &DrawedObject) -> bool { // destroy_yourself
    return match item {
        Item::None => { false }
        
        // guns
        Item::Glock19((ammo, cooldown)) => {
            if *ammo >= 1 && *cooldown <= 0.0 {
                *cooldown = 0.22;
                *ammo -= 1;

                let spread = random_spread(4.0);
                spawn_bullet(&mut commands, player.position, player.rotation, spread);
            }
            false
        }

        Item::AK47((ammo, cooldown)) => {
            if *ammo >= 1 && *cooldown <= 0.0 {
                *cooldown = 0.11;
                *ammo -= 1;

                let spread = random_spread(7.0);
                spawn_bullet(&mut commands, player.position, player.rotation, spread);
            }
            false
        }

        Item::Shotgun((ammo, cooldown)) => {
            if *ammo >= 1 && *cooldown <= 0.0 {
                *cooldown = 0.85;
                *ammo -= 1;

                for _ in 0..SHOTGUN_PELLETS {
                    let spread = random_spread(18.0);
                    spawn_bullet(&mut commands, player.position, player.rotation, spread);
                }
            }
            false
        }

        // ammo
        Item::Ammo9mm(_) | Item::Ammo7_62mm(_) | Item::Shotgunshells(_) => {
            if let Some(oth) = other[0].as_deref_mut() { 
                load_gun(item, oth);
            }
            if let Some(oth) = other[1].as_deref_mut() { 
                load_gun(item, oth);
            }
            false
        } 

        // tools
        Item::Flashlight((_, on)) => { *on = !*on; false }
        Item::Torch(_) => { false }
        Item::Crowbar => { false }
        Item::Lighter(_) => {
            if let Some(oth) = other[0].as_deref_mut() { 
                if light_up_torch(oth) {
                    if let Item::Lighter(fuel) = &mut item {
                        if *fuel >= 1.0 / 30.0 {
                            *fuel -= 1.0 / 30.0;
                        }
                    }
                }  
            } 
            if let Some(oth) = other[1].as_deref_mut() {
                if light_up_torch(oth) {
                    if let Item::Lighter(fuel) = &mut item {
                        if *fuel >= 1.0 / 30.0 {
                            *fuel -= 1.0 / 30.0;
                        }
                    }
                }  
            } 

            false
        }
        Item::Knife => { false }
        Item::FlintAndSteel => { 
            if let Some(oth) = other[0].as_deref_mut() { 
                light_up_torch(oth); 
            } 
            if let Some(oth) = other[1].as_deref_mut() { 
                light_up_torch(oth); 
            } 
            false 
        }

        // food
        Item::ProteinBar => { health.hunger -= 0.25; if health.hunger < 0.0 { health.hunger = 0.0 } true }
        Item::Banana => { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } health.thirst -= 0.1; if health.thirst < 0.0 { health.thirst = 0.0 } true }
        Item::Apple => { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } health.thirst -= 0.1; if health.thirst < 0.0 { health.thirst = 0.0 } true }
        Item::Cereal(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } *left <= 0 }
        Item::Bread(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } *left <= 0 }
        Item::FoodRation(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } *left <= 0 }
        Item::RottenFood => { health.hunger -= 0.05; if health.hunger < 0.0 { health.hunger = 0.0 } true }

        // health
        Item::Bandage(_) => { false }
        Item::SterileBandage(_) => { false }
        Item::AntiBiotic(_) => { false }
        Item::Antiseptic(_) => { false }
        Item::Soap(_) => { false }
        Item::Syringe(_) => { false }

        // items
        Item::Bottle300ml((liquid, fill)) | Item::Bottle500ml((liquid, fill)) | Item::Bottle1l((liquid, fill)) | Item::Bottle2l((liquid, fill)) =>
            { drink_liquid(health, if *fill >= 100.0 { *fill -= 100.0; 100.0 } else { *fill = 0.0; *fill }, *liquid); false }
        Item::Cloth => { false }
        Item::WaterFilter(_) => { false }
        Item::Backpack(_) => { false }
        Item::PlasticBag(_) => { false }
    };
}

const GLOCK_CAPACITY: u8 = 15;
const AK47_CAPACITY: u8 = 30;
const SHOTGUN_CAPACITY: u8 = 8;
const SHOTGUN_PELLETS: usize = 6;

fn drink_liquid(health: &mut PlayerHealth, drunk: f32, liquid: ItemLiquidType) {
    let torso = &mut health.body_parts[BodyPart::Torso as usize];
    let filling = (health.thirst - drunk / 600.0).max(0.0);

    match liquid {
        ItemLiquidType::None => {}
        ItemLiquidType::Water => { health.thirst = filling; }
        ItemLiquidType::ContaminatedWater => { health.thirst = filling; torso.infection += drunk / 2000.0; }
        ItemLiquidType::AppleJuice => { health.thirst = filling; health.hunger -= drunk / 6000.0; if health.hunger < 0.0 { health.hunger = 0.0 } }
        ItemLiquidType::OrangeJuice => { health.thirst = filling; health.hunger -= drunk / 6000.0; if health.hunger < 0.0 { health.hunger = 0.0 } }
        ItemLiquidType::Gasoline => { health.thirst = (health.thirst + drunk / 300.0).min(1.0); torso.infection += drunk / 1000.0; torso.health -= drunk / 2000.0; }
        ItemLiquidType::Antiserum => { health.thirst = (health.thirst + drunk / 800.0).min(1.0); }
        ItemLiquidType::Coagulant => {}
        ItemLiquidType::Sterid => {}
    }
}

fn light_up_torch(item: &mut Item) -> bool {
    return match item {
        Item::Torch((fuel, on)) => { if *fuel >= 0.05 { let wason = *on; *on = true; !wason } else { false } }
        _ => { false }
    };
}


pub fn load_gun(ammo: &mut Item, item: &mut Item) {
    let ((current_ammo, _), max_capacity, ammo_count) = match (item, &mut *ammo) {
        (Item::Glock19(gun_ammo), Item::Ammo9mm(box_ammo)) => {
            (gun_ammo, GLOCK_CAPACITY, box_ammo)
        }
        (Item::AK47(gun_ammo), Item::Ammo7_62mm(box_ammo)) => {
            (gun_ammo, AK47_CAPACITY, box_ammo)
        }
        (Item::Shotgun(gun_ammo), Item::Shotgunshells(box_ammo)) => {
            (gun_ammo, SHOTGUN_CAPACITY, box_ammo)
        }
        _ => return
    };

    if *current_ammo >= max_capacity || *ammo_count == 0 {
        return;
    }

    let needed = max_capacity - *current_ammo;
    let to_load = needed.min(*ammo_count);

    *current_ammo += to_load;
    *ammo_count -= to_load;
}

fn spawn_bullet(commands: &mut Commands, base_pos: Vec3, base_rot: Quat, angle_offset_rad: f32) {
    let spread_rot = Quat::from_rotation_z(angle_offset_rad);
    let final_rot = base_rot * spread_rot;

    commands.spawn((
        DrawedObject {
            position: base_pos,
            rotation: final_rot,
            size: Vec3::new(0.5 / 32.0, 0.5, 0.5),
            meshid: Meshes::mesh_Plane_smf,
            textureid: Textures::sprites_bullet_stroke_png,
            is_gui: false,
            enabled: true,
        },
        Bullet {
            lifespan: 0.0,
        },
    ));
}

fn random_spread(max_deg: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let factor = rng.gen_range(-1.0..=1.0);
    return (max_deg * factor).to_radians();
}