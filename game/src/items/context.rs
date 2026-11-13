use crate::{code::*, gen_bake::Textures, items::*};

#[derive(Message)]
pub struct ContextActivateMessage {
    pub location: ItemPos,
    pub is_health: bool,
    pub action: usize
}

pub fn handle_context_activate_system(
    mut activate_events: MessageReader<ContextActivateMessage>, 
    mut inventory: Query<&mut InventorySettings>
) {
    let Ok(mut sett) = inventory.single_mut() else { return };

    for message in activate_events.read() {
        if let Some(item) = message.location.get_mut(&mut sett.item) {
            context_menu_activate(item, message);
        }
    }
}

fn context_menu_activate(item: &mut Item, message: &ContextActivateMessage) {
    match item {
        Item::None => { }
            
        // guns
        Item::Glock19(_) => { }
        Item::AK47(_) => { }
        Item::Shotgun(_) => { }

        // ammo
        Item::Ammo9mm(_) => { }
        Item::Ammo7_62mm(_) => { }
        Item::Shotgunshells(_) => { }

        // tools
        Item::Flashlight(_) => { }
        Item::Torch(_) => { }
        Item::Crowbar => { }
        Item::Lighter(_) => { }
        Item::Knife => { }
        Item::FlintAndSteel => { }

        // food
        Item::ProteinBar => { }
        Item::Banana => { }
        Item::Apple => { }
        Item::Cereal(_) => { }
        Item::Bread(_) => { }
        Item::FoodRation(_) => { }
        Item::RottenFood => { }

        // health
        Item::Bandage(_) => { }
        Item::SterileBandage(_) => { }
        Item::AntiBiotic(_) => { }
        Item::Antiseptic(_) => { }
        Item::Soap(_) => { }
        Item::Syringe(_) => { }

        // items
        Item::Bottle300ml(_) => { }
        Item::Bottle500ml(_) => { }
        Item::Bottle1l(_) => { }
        Item::Bottle2l(_) => { }
        Item::Cloth => { }
        Item::WaterFilter(_) => { }
        Item::Backpack(_) => { }
        Item::PlasticBag(_) => { }
    }
}

impl Item {
    pub fn context_menu_normal_slot_requiriment_with_aproprieate_texturing(&self) -> &[Textures] {
        return match self {
            Item::None => { &[] }
            
            // guns
            Item::Glock19(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::AK47(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Shotgun(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }

            // ammo
            Item::Ammo9mm(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Ammo7_62mm(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Shotgunshells(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }

            // tools
            Item::Flashlight(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Torch(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Crowbar => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Lighter(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Knife => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::FlintAndSteel => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }

            // food
            Item::ProteinBar => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Banana => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Apple => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Cereal(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Bread(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::FoodRation(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::RottenFood => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }

            // health
            Item::Bandage(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::SterileBandage(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::AntiBiotic(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Antiseptic(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Soap(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Syringe(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }

            // items
            Item::Bottle300ml(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Bottle500ml(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Bottle1l(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Bottle2l(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Cloth => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::WaterFilter(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::Backpack(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
            Item::PlasticBag(_) => { &[Textures::sprites_items_ProteinBar_png, Textures::sprites_items_AntiBiotic_png] }
        }
    }
}




// fn inject_liquid(health: &mut PlayerHealth, injected_ml: f32, liquid: ItemLiquidType) {
//     let torso = &mut health.body_parts[BodyPart::Torso as usize];

//     match liquid {
//         ItemLiquidType::None => {}
//         ItemLiquidType::Water => {}
//         ItemLiquidType::ContaminatedWater => {
//             torso.infection = (torso.infection + injected_ml / 50.0).min(1.0);
//             torso.health = (torso.health - injected_ml / 100.0).min(1.0);
//         }
//         ItemLiquidType::AppleJuice | ItemLiquidType::OrangeJuice => {
//             torso.infection = (torso.infection + injected_ml / 100.0).min(1.0);
//             torso.health = (torso.health - injected_ml / 80.0).min(1.0);
//         }
//         ItemLiquidType::Gasoline => {
//             torso.health = (torso.health - injected_ml / 20.0).min(1.0);
//             torso.infection = (torso.infection + injected_ml / 30.0).min(1.0);
//             health.blod_volume = (health.blod_volume - injected_ml / 50.0).max(0.0);
//         }
//         ItemLiquidType::Antiserum => {
//             health.surowica = (health.surowica + injected_ml / 25.0).min(1.0);
//         }
//         ItemLiquidType::Coagulant => {}
//         ItemLiquidType::Sterid => {}
//     }
// }