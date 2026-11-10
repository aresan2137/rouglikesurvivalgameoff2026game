use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::Delta;

use crate::{code::{BodyPart, Bullet, InventorySettings, PlayerHealth}, gen_bake::{Meshes, Textures}, renderer::DrawedObject};

const GLOCK_CAPACITY: u8 = 15;
const AK47_CAPACITY: u8 = 30;
const SHOTGUN_CAPACITY: u8 = 8;
const SHOTGUN_PELLETS: usize = 6;

#[derive(Clone, PartialEq)]
pub enum Item {
    None,

    // guns
    Glock19((u8, f32)),
    AK47((u8, f32)),
    Shotgun((u8, f32)),

    // ammo
    Ammo9mm(u8),
    Ammo7_62mm(u8), 
    Shotgunshells(u8),

    // tools
    Flashlight((f32, bool)),
    Torch((f32, bool)),
    Crowbar,
    Lighter(f32),
    Knife,
    FlintAndSteel,

    // food
    ProteinBar,
    Banana,
    Apple,
    Cereal(u8),
    Bread(u8),
    FoodRation(u8),
    RottenFood,

    // health
    Bandage(f32),
    SterileBandage(f32),
    AntiBiotic(u8),
    Antiseptic(f32),
    Soap(f32),
    Syringe((ItemLiquidType, f32)),

    // items
    Bottle300ml((ItemLiquidType, f32)),
    Bottle500ml((ItemLiquidType, f32)),
    Bottle1l((ItemLiquidType, f32)),
    Bottle2l((ItemLiquidType, f32)),
    Cloth,
    WaterFilter(f32),
    Backpack(Vec<Item>),
    PlasticBag(Vec<Item>)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemLiquidType {
    None,
    Water,
    ContaminatedWater,
    AppleJuice,
    OrangeJuice,
    Gasoline,
    Antiserum
}

#[derive(Component)]
pub struct DropedItem {
    pub item: Item,
    pub position: Vec2
}

impl ItemLiquidType {
    pub fn get_name_description(&self) -> (String, String) {
        return match self {
            ItemLiquidType::None => { ("none".to_string(), "none".to_string()) }
            ItemLiquidType::Water => { ("water".to_string(), "normal clean water".to_string()) }
            ItemLiquidType::ContaminatedWater => { ("contaminated water".to_string(), "you can use a filter to clean it".to_string()) }
            ItemLiquidType::AppleJuice => { ("apple juice".to_string(), "juice".to_string()) }
            ItemLiquidType::OrangeJuice => { ("orange juice".to_string(), "juice".to_string()) }
            ItemLiquidType::Gasoline => { ("gasoline".to_string(), "you can use it on Torch to refuel it".to_string()) }
            ItemLiquidType::Antiserum => { ("antiserum".to_string(), "antidote for zombie virus. the latest 10 minutes afther a bite. if serum get's applyed post 10 minutes it will only extend your lifespan. must be injected, Don't Drink it.".to_string()) }
        };
    }
}

fn process_bottle(liquid: ItemLiquidType, fill: f32, capacity: &str, ty: &str, desc: &str) -> (String, String) {
    let liquid_name_description = liquid.get_name_description();
    return (
        format!("{} {} ({} {:.0}ml)", ty, capacity, liquid_name_description.0, fill),
        format!("{}. {}. {}'s capacity: {}\n{}: {}", ty, desc, ty, capacity, liquid_name_description.0, liquid_name_description.1)
    );
}

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
    }
}

fn inject_liquid(health: &mut PlayerHealth, injected_ml: f32, liquid: ItemLiquidType) {
    let torso = &mut health.body_parts[BodyPart::Torso as usize];

    match liquid {
        ItemLiquidType::None => {}
        ItemLiquidType::Water => {}
        ItemLiquidType::ContaminatedWater => {
            torso.infection = (torso.infection + injected_ml / 50.0).min(1.0);
            torso.health = (torso.health - injected_ml / 100.0).min(1.0);
        }
        ItemLiquidType::AppleJuice | ItemLiquidType::OrangeJuice => {
            torso.infection = (torso.infection + injected_ml / 100.0).min(1.0);
            torso.health = (torso.health - injected_ml / 80.0).min(1.0);
        }
        ItemLiquidType::Gasoline => {
            torso.health = (torso.health - injected_ml / 20.0).min(1.0);
            torso.infection = (torso.infection + injected_ml / 30.0).min(1.0);
            health.blod_volume = (health.blod_volume - injected_ml / 50.0).max(0.0);
        }
        ItemLiquidType::Antiserum => {
            health.surowica = (health.surowica + injected_ml / 25.0).min(1.0);
        }
    }
}

fn light_up_torch(item: &mut Item) -> bool {
    return match item {
        Item::Torch((fuel, on)) => { if *fuel >= 0.05 { *on = true; true } else { false } }
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

fn get_two_mut<T>(slice: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b || a >= slice.len() || b >= slice.len() {
        return None;
    }
    if a < b {
        let (left, right) = slice.split_at_mut(b);
        Some((&mut left[a], &mut right[0]))
    } else {
        let (left, right) = slice.split_at_mut(a);
        Some((&mut right[0], &mut left[b]))
    }
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
    let factor = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() % 1000) as f32 / 500.0 - 1.0;
    (max_deg * factor).to_radians()
}

impl Item {
    pub fn get_name_description(&self) -> (String, String) {
        return match self {
            Item::None => { ("none".to_string(), "none".to_string()) }
            
            // guns
            Item::Glock19((ammo, _)) => { (format!("glock 19 ({} bullets)", ammo), "starter gun".to_string()) }
            Item::AK47((ammo, _)) => { (format!("ak-47 ({} bullets)", ammo), "distance gun".to_string()) }
            Item::Shotgun((ammo, _)) => { (format!("shotgun ({} shells)", ammo), "amount gun".to_string()) }

            // ammo
            Item::Ammo9mm(ammo) => { (format!("9mm ammo box ({} bullets)", ammo), "amunition for glock 19".to_string()) }
            Item::Ammo7_62mm(ammo) => { (format!("7.62mm ammo box ({} bullets)", ammo), "amunition for ak-47".to_string()) }
            Item::Shotgunshells(ammo) => { (format!("shotgun ammo box ({} shells)", ammo), "amunition for shotgun".to_string()) }

            // tools
            Item::Flashlight((energy, _)) => { (format!("flashlight ({:.0}%)", energy * 100.0), "flashlight. won't belive it. it releases light".to_string()) }
            Item::Torch((fuel, _)) => { (format!("torch ({:.0}%)", fuel * 100.0), "torch. gives off less light than an flashlight. can be refueled with gasoline".to_string()) }
            Item::Crowbar => { ("crowbar".to_string(), "good melee weapon and allows to break in easier".to_string()) }
            Item::Lighter(fuel) => { (format!("lighter ({:.0}%)", fuel * 100.0), "lighter. can be used to light an torch. limited uses".to_string()) }
            Item::Knife => { ("knife".to_string(), "good melee weapon. tho you can't cook using it".to_string()) }
            Item::FlintAndSteel => { ("flint and steal".to_string(), "can be used to light an torch. it's not very fast".to_string()) }

            // food
            Item::ProteinBar => { ("protein bar".to_string(), "food".to_string()) }
            Item::Banana => { ("banana".to_string(), "food".to_string()) }
            Item::Apple => { ("apple".to_string(), "food".to_string()) }
            Item::Cereal(left) => { (format!("Cereal ({}%)", left * 25), "food".to_string()) }
            Item::Bread(left) => { (format!("bread ({}%)", left * 25), "food".to_string()) }
            Item::FoodRation(left) => { (format!("food ration ({}%)", left * 25), "food".to_string()) }
            Item::RottenFood => { ("rotten food".to_string(), "food?".to_string()) }

            // health
            Item::Bandage(left) => { (format!("bandage ({:.0}%)", left * 100.0), "bandage. use it on your wounds and remember to use an antiseptic. can be converted to steryle bandage when combined with an antiseptic".to_string()) }
            Item::SterileBandage(left) => { (format!("steryle bandage ({:.0}%)", left * 100.0), "bandage. use it on your wounds antiseptic build-in (:".to_string()) }
            Item::AntiBiotic(left) => { ( format!("antibiotic ({}%)", left * 10), "antibiotic. if you didn't used an antiseptic and you get infected use it".to_string()) }
            Item::Antiseptic(fill) => { (format!("antiseptic ({:.0}%)", fill * 100.0), "antiseptic. use it on your wounds to prevent infection".to_string()) }
            Item::Soap(left) => { (format!("soap ({:.0}%)", left * 100.0), "soap. when bitten by an zombie clean your wound if applyed before 2 minutes pass it will slow down the infection and there is an 25% chance to defeat the virus".to_string()) }
            Item::Syringe((liquid, fill)) => { process_bottle(*liquid, *fill, "25ml", "Syringe", "allows to inject medicyne")  }

            // items
            Item::Bottle300ml((liquid, fill)) => { process_bottle(*liquid, *fill, "300ml", "bottle", "can store liquids") }
            Item::Bottle500ml((liquid, fill)) => { process_bottle(*liquid, *fill, "500ml", "bottle", "can store liquids") }
            Item::Bottle1l((liquid, fill)) => { process_bottle(*liquid, *fill, "1l", "bottle", "can store liquids") }
            Item::Bottle2l((liquid, fill)) => { process_bottle(*liquid, *fill, "2l", "bottle", "can store liquids") }
            Item::Cloth => { ("cloth".to_string(), "can be converted to an torch or bandage".to_string()) }
            Item::WaterFilter(uses) => { (format!("water filter ({:.0}%)", uses * 100.0), "turns contaminated water into clean water".to_string()) }
            Item::Backpack(_) => { ("backpack".to_string(), "can store items. press \"...\" to open it".to_string()) }
            Item::PlasticBag(_) => { ("plastic bag".to_string(), "can store items. press \"...\" to open it".to_string()) }
        };
    }

    pub fn use_(items: &mut [Item], slot: usize, health: &mut PlayerHealth, mut commands: &mut Commands, player: &DrawedObject) -> (bool, bool) { // true = long, false = short; destroy_yourself
        return match &mut items[slot] {
            Item::None => { (false, false) }
            
            // guns
            Item::Glock19((ammo, cooldown)) => {
                if *ammo >= 1 && *cooldown <= 0.0 {
                    *cooldown = 0.22;
                    *ammo -= 1;

                    let spread = random_spread(4.0);
                    spawn_bullet(&mut commands, player.position, player.rotation, spread);
                }
                (true, false)
            }

            Item::AK47((ammo, cooldown)) => {
                if *ammo >= 1 && *cooldown <= 0.0 {
                    *cooldown = 0.11;
                    *ammo -= 1;

                    let spread = random_spread(7.0);
                    spawn_bullet(&mut commands, player.position, player.rotation, spread);
                }
                (true, false)
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
                (true, false)
            }

            // ammo
            Item::Ammo9mm(_) | Item::Ammo7_62mm(_) | Item::Shotgunshells(_) => {
                if let Some((ammo, gun)) = get_two_mut(items, slot, 0) {
                    load_gun(ammo, gun);
                }
                if let Some((ammo, gun)) = get_two_mut(items, slot, 4) {
                    load_gun(ammo, gun);
                }                

                (false, false)
            }

            // tools
            Item::Flashlight((_, on)) => { *on = !*on; (false, false) }
            Item::Torch(_) => { (false, false) }
            Item::Crowbar => { (false, false) }
            Item::Lighter(_) => {
                let mut lit = false;

                if slot != 0 && light_up_torch(&mut items[0]) {
                     if let Item::Lighter(fuel) = &mut items[slot] {
                        if *fuel >= 1.0 / 30.0 {
                            *fuel -= 1.0 / 30.0;
                        }
                    }
                } 
                if slot != 4 && light_up_torch(&mut items[4]) {
                     if let Item::Lighter(fuel) = &mut items[slot] {
                        if *fuel >= 1.0 / 30.0 {
                            *fuel -= 1.0 / 30.0;
                        }
                    }
                }

                (false, false)
            }
            Item::Knife => { (false, false) }
            Item::FlintAndSteel => { light_up_torch(&mut items[0]); light_up_torch(&mut items[4]); (false, false) }

            // food
            Item::ProteinBar => { health.hunger -= 0.25; if health.hunger < 0.0 { health.hunger = 0.0 } (false, true) }
            Item::Banana => { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } health.thirst -= 0.1; if health.thirst < 0.0 { health.thirst = 0.0 } (false, true) }
            Item::Apple => { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } health.thirst -= 0.1; if health.thirst < 0.0 { health.thirst = 0.0 } (false, true) }
            Item::Cereal(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } (false, *left <= 0) }
            Item::Bread(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } (false, *left <= 0) }
            Item::FoodRation(left) => { if *left >= 1 { health.hunger -= 0.15; if health.hunger < 0.0 { health.hunger = 0.0 } *left -= 1; } (false, *left <= 0) }
            Item::RottenFood => { health.hunger -= 0.05; if health.hunger < 0.0 { health.hunger = 0.0 } (false, true) }

            // health
            Item::Bandage(_) => { (false, false) }
            Item::SterileBandage(_) => { (false, false) }
            Item::AntiBiotic(_) => { (false, false) }
            Item::Antiseptic(_) => { (false, false) }
            Item::Soap(_) => { (false, false) }
            Item::Syringe((liquid, fill)) => { inject_liquid(health, *fill, *liquid); (false, false) }

            // items
            Item::Bottle300ml((liquid, fill)) | Item::Bottle500ml((liquid, fill)) | Item::Bottle1l((liquid, fill)) | Item::Bottle2l((liquid, fill)) =>
                { drink_liquid(health, if *fill >= 100.0 { *fill -= 100.0; 100.0 } else { *fill = 0.0; *fill }, *liquid); (false, false) }
            Item::Cloth => { (false, false) }
            Item::WaterFilter(_) => { (false, false) }
            Item::Backpack(_) => { (false, false) }
            Item::PlasticBag(_) => { (false, false) }
        };
    }

    pub fn get_texture(&self) -> Option<Textures> {
        return match self {
            Item::None => { None }
            
            // guns
            Item::Glock19(_) => { Some(Textures::sprites_items_Glock19_png) }
            Item::AK47(_) => { Some(Textures::sprites_items_AK47_png) }
            Item::Shotgun(_) => { Some(Textures::sprites_items_Shotgun_png) }

            // ammo
            Item::Ammo9mm(_) => { Some(Textures::sprites_items_Ammo9mm_png) }
            Item::Ammo7_62mm(_) => { Some(Textures::sprites_items_Ammo7_62mm_png) }
            Item::Shotgunshells(_) => { Some(Textures::sprites_items_Shotgunshells_png) }

            // tools
            Item::Flashlight(_) => { Some(Textures::sprites_items_Flashlight_png) }
            Item::Torch(_) => { Some(Textures::sprites_items_Torch_png) }
            Item::Crowbar => { Some(Textures::sprites_items_Crowbar_png) }
            Item::Lighter(_) => { Some(Textures::sprites_items_Lighter_png) }
            Item::Knife => { Some(Textures::sprites_items_Knife_png) }
            Item::FlintAndSteel => { Some(Textures::sprites_items_FlintAndSteel_png) }

            // food
            Item::ProteinBar => { Some(Textures::sprites_items_ProteinBar_png) }
            Item::Banana => { Some(Textures::sprites_items_Banana_png) }
            Item::Apple => { Some(Textures::sprites_items_Apple_png) }
            Item::Cereal(_) => { Some(Textures::sprites_items_Cerial_png) }
            Item::Bread(_) => { Some(Textures::sprites_items_Bread_png) }
            Item::FoodRation(_) => { Some(Textures::sprites_items_FoodRation_png) }
            Item::RottenFood => { Some(Textures::sprites_items_RottenFood_png) }

            // health
            Item::Bandage(_) => { Some(Textures::sprites_items_Bandage_png) }
            Item::SterileBandage(_) => { Some(Textures::sprites_items_SterileBandage_png) }
            Item::AntiBiotic(_) => { Some(Textures::sprites_items_AntiBiotic_png) }
            Item::Antiseptic(_) => { Some(Textures::sprites_items_Antiseptic_png) }
            Item::Soap(_) => { Some(Textures::sprites_items_Soap_png) }
            Item::Syringe(_) => { Some(Textures::sprites_items_Syringe_png) }

            // items
            Item::Bottle300ml(_) => { Some(Textures::sprites_items_Bottle300ml_png) }
            Item::Bottle500ml(_) => { Some(Textures::sprites_items_Bottle500ml_png) }
            Item::Bottle1l(_) => { Some(Textures::sprites_items_Bottle1l_png) }
            Item::Bottle2l(_) => { Some(Textures::sprites_items_Bottle2l_png) }
            Item::Cloth => { Some(Textures::sprites_items_Cloth_png) }
            Item::WaterFilter(_) => { Some(Textures::sprites_items_WaterFilter_png) }
            Item::Backpack(_) => { Some(Textures::sprites_items_Backpack_png) }
            Item::PlasticBag(_) => { Some(Textures::sprites_items_PlasticBag_png) }
        };
    }

    pub fn tick(&mut self, delta: f32) {
        match self {
            Item::Backpack(items) | Item::PlasticBag(items) => { for item in items { item.tick(delta); } }
            Item::Glock19((_, cooldown)) | Item::AK47((_, cooldown)) | Item::Shotgun((_, cooldown)) => { *cooldown -= delta; }
            _ => {}
        }
    }
}

pub fn tick_items(mut inventory: Query<&mut InventorySettings>, time: Res<Delta>) {
    if let Ok(mut sett) = inventory.single_mut() {
        for item in &mut sett.item {
            item.tick(time.delta);
        }
    }
}