use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, gen_bake::*, renderer::DrawedObject};

#[derive(Component)]
pub struct InventorySettings {
    pub item: [Item; 6],
    pub selected_item: usize
}

#[derive(Component)]
pub struct InventoryItemDisplay {
    pub itemid: usize
}

pub fn setup_inventory(world: &mut World) {
    world.spawn((DrawedObject {
        position: Vec3::new(0.0, 0.0, -0.5),
        rotation: Quat::IDENTITY,
        size: Vec3::new(2.0, 2.0, 2.0),
        meshid: Meshes::mesh_inventoryMenu_smf,
        textureid: Textures::sprites_gray_color,
        is_gui: true,
        enabled: false
    }, InventorySettings {
       item: [Item::Cloth, Item::Glock19(4), Item::Flashlight(0.73), Item::Bottle300ml((ItemLiquidType::AppleJuice, 220.0)), Item::None, Item::ProteinBar],
       selected_item: 0
    }));

    for i in 0..6 {
        let radius: f32 = 6.0;

        let angle = (std::f32::consts::PI / 6.0) - (i as f32) * (std::f32::consts::PI / 3.0);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        let position = Vec3::new(x, y, -0.51);

        world.spawn((DrawedObject {
            position,
            rotation: Quat::IDENTITY,
            size: Vec3::new(2.0, 2.0, 2.0),
            meshid: Meshes::mesh_Plane_smf,
            textureid: Textures::sprites_ProteinBar_png,
            is_gui: true,
            enabled: false
        }, InventoryItemDisplay {
            itemid: i
        }));
    }
}

pub fn controler_inventory(mut input_getan: ResMut<InputGetan>, time: Res<Delta>, mut inventory: Query<(&mut InventorySettings, &mut DrawedObject)>, mut item_display: Query<(&InventoryItemDisplay, &mut DrawedObject), Without<InventorySettings>>) {
    let gui_enabled = input_getan.owner == InputOwner::Inventory;
    
    if let Ok((mut sett, mut obj)) = inventory.single_mut() {
        obj.enabled = gui_enabled;

        for (display, mut obj) in item_display.iter_mut() {
            obj.enabled = gui_enabled;
            if gui_enabled {
                if let Some(tex) = get_item_textures(&sett.item[display.itemid]) {
                    obj.textureid = tex;
                } else {
                    obj.enabled = false;
                }     

                obj.size = if display.itemid == sett.selected_item { Vec3::new(2.2, 2.2, 2.2) } else { Vec3::new(1.8, 1.8, 1.8) } 
            }
        }

        if gui_enabled {
            egui::Window::new("ItemStat")
                .default_width(600.0)
                .resizable(true)
                .show(&ui(), |ui| {
                ui.label("item over");
                ui.label(format!("selected: {}", sett.selected_item));
                let itemstat = get_item_name_description(&sett.item[sett.selected_item]);
                ui.label(itemstat.0);
                ui.label(itemstat.1);
            });

            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_short();
                    log::info!("inventory: __");
                }
                if ine[0] == false && ine[1] == true { // ._
                    input_getan.reset_short();
                    log::info!("inventory: ._");
                }
            } else if input_getan.now_setting == 3 {
                if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    input_getan.reset_short();
                    if sett.selected_item == 5 {
                        sett.selected_item = 0;
                    } else {
                        sett.selected_item += 1;
                    }
                }
                if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                    input_getan.reset_short();
                    if sett.selected_item == 0 {
                        sett.selected_item = 5;
                    } else {
                        sett.selected_item -= 1;
                    }
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    input_getan.reset_short();
                    input_getan.owner = InputOwner::Gameplay;
                    log::info!("inventory OFF");
                }
                if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                    input_getan.reset_short();
                    log::info!("inventory: ...");
                }
            }
        }
    }
}