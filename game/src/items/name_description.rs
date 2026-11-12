use crate::items::*;

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
}

impl ItemLiquidType {
    pub fn get_name_description(&self) -> (&str, &str) {
        return match self {
            ItemLiquidType::None => { ("", "") }
            ItemLiquidType::Water => { ("water", "normal clean water") }
            ItemLiquidType::ContaminatedWater => { ("contaminated water", "you can use a filter to clean it") }
            ItemLiquidType::AppleJuice => { ("apple juice", "juice") }
            ItemLiquidType::OrangeJuice => { ("orange juice", "juice") }
            ItemLiquidType::Gasoline => { ("gasoline", "you can use it on Torch to refuel it") }
            ItemLiquidType::Antiserum => { ("antiserum", "antidote for zombie virus. the latest 10 minutes afther a bite. if serum get's applyed post 10 minutes it will only extend your lifespan. must be injected, Don't Drink it.") }
            ItemLiquidType::Coagulant => { ("coagulant", "promotes bload clothing") }
            ItemLiquidType::Sterid => { ("sterid", "promotes healing") }
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