#![allow(unused_imports)]

use bevy_ecs::{message::message_update_system, prelude::*};
use glam::*;
use smq_engine::*;

use crate::code::InventorySettings;

mod tick;
pub use tick::*;

mod texture;
pub use texture::*;

mod name_description;
pub use name_description::*;

mod use_;
pub use use_::*;

mod context;
pub use context::*;

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
    Antiserum,
    Coagulant,
    Sterid
}

pub fn start_items(world: &mut World, schedule: &mut Schedule) {
    world.init_resource::<Messages<UseItemMessage>>();

    schedule.add_systems((
        tick_items_inventory, 
        tick_items_ground,
        handle_simple_use_item_system,
        message_update_system.after(handle_simple_use_item_system)
    ));
}

pub fn tick_items_inventory(mut inventory: Query<&mut InventorySettings>, time: Res<Delta>) {
    if let Ok(mut sett) = inventory.single_mut() {
        for item in &mut sett.item {
            item.tick(time.delta);
        }
    }
}

pub fn tick_items_ground(mut inventory: Query<&mut DropedItem>, time: Res<Delta>) {
    if let Ok(mut item) = inventory.single_mut() {
        item.item.tick(time.delta);
    }
}

#[derive(Component)]
pub struct DropedItem {
    pub item: Item,
    pub position: Vec2
}

