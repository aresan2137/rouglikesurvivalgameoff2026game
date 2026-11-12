use crate::items::*;

impl Item {
    pub fn tick(&mut self, delta: f32) {
        match self {
            Item::Backpack(items) | Item::PlasticBag(items) => { for item in items { item.tick(delta); } }
            Item::Glock19((_, cooldown)) | Item::AK47((_, cooldown)) | Item::Shotgun((_, cooldown)) => { *cooldown -= delta; }
            Item::Bottle300ml(ins) | Item::Bottle500ml(ins) | Item::Bottle1l(ins) | Item::Bottle2l(ins) | Item::Syringe(ins) 
                => { if ins.1 <= 0.0 { ins.0 = ItemLiquidType::None } }
            _ => {}
        }
    }
}