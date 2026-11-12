use crate::{gen_bake::Textures, items::*};

impl Item {
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
}
