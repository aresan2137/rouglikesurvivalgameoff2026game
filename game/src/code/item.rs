use crate::gen_bake::Textures;

#[derive(Clone)]
pub enum Item {
    None,

    // guns
    Glock19(u8),
    AK47(u8),
    Shotgun(u8),

    // ammo
    Ammo9mm(u8),
    Ammo7_62mm(u8), 
    Shotgunshells(u8),

    // tools
    Flashlight(f32),
    Torch(f32),
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

pub fn get_item_liquid_name_description(liquid: ItemLiquidType) -> (String, String) {
    return match liquid {
        ItemLiquidType::None => { ("none".to_string(), "none".to_string()) }
        ItemLiquidType::Water => { ("water".to_string(), "normal clean water".to_string()) }
        ItemLiquidType::ContaminatedWater => { ("contaminated water".to_string(), "you can use a filter to clean it".to_string()) }
        ItemLiquidType::AppleJuice => { ("apple juice".to_string(), "juice".to_string()) }
        ItemLiquidType::OrangeJuice => { ("orange juice".to_string(), "juice".to_string()) }
        ItemLiquidType::Gasoline => { ("gasoline".to_string(), "you can use it on Torch to refuel it".to_string()) }
        ItemLiquidType::Antiserum => { ("antiserum".to_string(), "antidote for zombie virus. the latest 10 minutes afther a bite. if serum get's applyed post 10 minutes it will only extend your lifespan".to_string()) }
    };
}

fn process_bottle(liquid: ItemLiquidType, fill: f32, capacity: &str, ty: &str, desc: &str) -> (String, String) {
    let liquid_name_description = get_item_liquid_name_description(liquid);
    return (
        format!("{} {} ({} {:.0}ml)", ty, capacity, liquid_name_description.0, fill),
        format!("{}. {}. {}'s capacity: {}\n{}: {}", ty, desc, ty, capacity, liquid_name_description.0, liquid_name_description.1)
    );
}

pub fn get_item_name_description(item: &Item) -> (String, String) {
    return match item {
        Item::None => { ("none".to_string(), "none".to_string()) }
        
        // guns
        Item::Glock19(ammo) => { (format!("glock 19 ({} bullets)", ammo), "starter gun".to_string()) }
        Item::AK47(ammo) => { (format!("ak-47 ({} bullets)", ammo), "distance gun".to_string()) }
        Item::Shotgun(ammo) => { (format!("shotgun ({} shells)", ammo), "amount gun".to_string()) }

        // ammo
        Item::Ammo9mm(ammo) => { (format!("9mm ammo box ({} bullets)", ammo), "amunition for glock 19".to_string()) }
        Item::Ammo7_62mm(ammo) => { (format!("7.62mm ammo box ({} bullets)", ammo), "amunition for ak-47".to_string()) }
        Item::Shotgunshells(ammo) => { (format!("shotgun ammo box ({} shells)", ammo), "amunition for shotgun".to_string()) }

        // tools
        Item::Flashlight(energy) => { (format!("flashlight ({:.0}%)", energy * 100.0), "flashlight. won't belive it. it releases light".to_string()) }
        Item::Torch(fuel) => { (format!("torch ({:.0}%)", fuel * 100.0), "torch. gives off less light than an flashlight. can be refueled with gasoline".to_string()) }
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
        Item::Backpack(_) => { ("backpack".to_string(), "can store items press \"...\" to open it".to_string()) }
        Item::PlasticBag(_) => { ("plastic bag".to_string(), "can store items press \"...\" to open it".to_string()) }
    };
}

pub fn get_item_textures(item: &Item) -> Option<Textures> {
    return match item {
        Item::None => { None }
        
        // guns
        Item::Glock19(_) => { Some(Textures::sprites_Glock19_png) }
        Item::AK47(_) => { Some(Textures::sprites_AK47_png) }
        Item::Shotgun(_) => { Some(Textures::sprites_Shotgun_png) }

        // ammo
        Item::Ammo9mm(_) => { Some(Textures::sprites_Ammo9mm_png) }
        Item::Ammo7_62mm(_) => { Some(Textures::sprites_Ammo7_62mm_png) }
        Item::Shotgunshells(_) => { Some(Textures::sprites_Shotgunshells_png) }

        // tools
        Item::Flashlight(_) => { Some(Textures::sprites_Flashlight_png) }
        Item::Torch(_) => { Some(Textures::sprites_Torch_png) }
        Item::Crowbar => { Some(Textures::sprites_Crowbar_png) }
        Item::Lighter(_) => { Some(Textures::sprites_Lighter_png) }
        Item::Knife => { Some(Textures::sprites_Knife_png) }
        Item::FlintAndSteel => { Some(Textures::sprites_FlintAndSteel_png) }

        // food
        Item::ProteinBar => { Some(Textures::sprites_ProteinBar_png) }
        Item::Banana => { Some(Textures::sprites_Banana_png) }
        Item::Apple => { Some(Textures::sprites_Apple_png) }
        Item::Cereal(_) => { Some(Textures::sprites_Cerial_png) }
        Item::Bread(_) => { Some(Textures::sprites_Bread_png) }
        Item::FoodRation(_) => { Some(Textures::sprites_FoodRation_png) }
        Item::RottenFood => { Some(Textures::sprites_RottenFood_png) }

        // health
        Item::Bandage(_) => { Some(Textures::sprites_Bandage_png) }
        Item::SterileBandage(_) => { Some(Textures::sprites_SterileBandage_png) }
        Item::AntiBiotic(_) => { Some(Textures::sprites_AntiBiotic_png) }
        Item::Antiseptic(_) => { Some(Textures::sprites_Antiseptic_png) }
        Item::Soap(_) => { Some(Textures::sprites_Soap_png) }
        Item::Syringe(_) => { Some(Textures::sprites_Syringe_png) }

        // items
        Item::Bottle300ml(_) => { Some(Textures::sprites_Bottle300ml_png) }
        Item::Bottle500ml(_) => { Some(Textures::sprites_Bottle500ml_png) }
        Item::Bottle1l(_) => { Some(Textures::sprites_Bottle1l_png) }
        Item::Bottle2l(_) => { Some(Textures::sprites_Bottle2l_png) }
        Item::Cloth => { Some(Textures::sprites_Cloth_png) }
        Item::WaterFilter(_) => { Some(Textures::sprites_WaterFilter_png) }
        Item::Backpack(_) => { Some(Textures::sprites_Backpack_png) }
        Item::PlasticBag(_) => { Some(Textures::sprites_PlasticBag_png) }
    };
}
