use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, gen_bake::*, items::*, renderer::{DrawedObject, draw_free_uniform_text}};

#[derive(Component)]
pub struct InventorySettings {
    pub item: [Item; 7],
    pub selected_item: usize
}

#[derive(Component)]
pub struct InventoryItemDisplay {
    pub itemid: usize
}

#[derive(Component)]
pub struct InventorySelector {}

pub fn setup_inventory(world: &mut World) {
    world.spawn((DrawedObject {
        position: Vec3::new(0.0, 0.0, -0.5),
        rotation: Quat::IDENTITY,
        size: Vec3::new(1.8, 1.8, 1.8),
        meshid: Meshes::mesh_inventoryMenu_smf,
        textureid: Textures::sprites_gray_color,
        is_gui: true,
        enabled: false
    }, InventorySettings {
       item: [Item::Cloth, Item::Glock19((4, 0.0)), Item::Flashlight((0.73, false)), Item::Bottle300ml((ItemLiquidType::AppleJuice, 220.0)), Item::None, Item::ProteinBar, Item::None],
       selected_item: 0
    }));

    world.spawn((DrawedObject {
        position: Vec3::new(0.0, 0.0, -0.5),
        rotation: Quat::IDENTITY,
        size: Vec3::new(1.8, 1.8, 1.8),
        meshid: Meshes::mesh_inventorySelector_smf,
        textureid: Textures::sprites_gray_color,
        is_gui: true,
        enabled: false
    }, InventorySelector {}));


    for i in 0..7 {
        let radius: f32 = 5.5;

        let angle = (std::f32::consts::PI / 6.0) - (i as f32) * (std::f32::consts::PI / 3.0);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        let mut position = Vec3::new(x, y, -0.51);

        if i == 6 {
            position = Vec3::new(0.0, 2.0, -0.51);
        }

        world.spawn((DrawedObject {
            position,
            rotation: Quat::IDENTITY,
            size: Vec3::new(2.0, 2.0, 2.0),
            meshid: Meshes::mesh_Plane_smf,
            textureid: Textures::sprites_items_ProteinBar_png,
            is_gui: true,
            enabled: false
        }, InventoryItemDisplay {
            itemid: i
        }));
    }
}

pub fn controler_inventory(
    mut commands: Commands, 
    mut input_getan: ResMut<InputGetan>,
    mut inventory: Query<(&mut InventorySettings, &mut DrawedObject)>, 
    mut item_display: Query<(&InventoryItemDisplay, &mut DrawedObject), Without<InventorySettings>>,
    mut item_selector: Query<&mut DrawedObject, (With<InventorySelector>, Without<InventorySettings>, Without<InventoryItemDisplay>)>,
    mut player: Query<&FreeCamera, (Without<InventorySelector>, Without<InventorySettings>, Without<InventoryItemDisplay>)>,
    mut use_item_writer: MessageWriter<UseItemMessage>,
) {
    let gui_enabled = input_getan.owner == InputOwner::Inventory;

    if let Ok((mut sett, mut obj)) = inventory.single_mut() {
        if let Ok(freecam) = player.single_mut() {
            obj.enabled = gui_enabled;

            for (display, mut obj) in item_display.iter_mut() {
                obj.enabled = gui_enabled;
                if gui_enabled {
                    if let Some(tex) = sett.item[display.itemid].get_texture() {
                        obj.textureid = tex;
                    } else {
                        obj.enabled = false;
                    }     

                    obj.size = if display.itemid == sett.selected_item { Vec3::new(1.8, 1.8, 1.8) } else { Vec3::new(1.3, 1.3, 1.3) } 
                }
            }

            if let Ok(mut objed) = item_selector.single_mut() {
                objed.enabled = gui_enabled;
                if gui_enabled {
                    objed.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, (sett.selected_item + 1) as f32 * ((std::f32::consts::PI * -2.0) / 6.0))
                }
            }
            
            if gui_enabled {
                if sett.item[sett.selected_item] != Item::None {    
                    let itemstat = sett.item[sett.selected_item].get_name_description();

                    draw_free_uniform_text(&format!("{}\n{}", itemstat.0, itemstat.1), Vec2::new(0.0, -0.9), Vec2::ZERO, 24.0);
                }

                let ine = input_getan.morse_input;

                if input_getan.now_setting == 2 {
                    if ine[0] == true && ine[1] == true { // __
                        input_getan.reset_short();
                        if sett.item[sett.selected_item] != Item::None {
                            input_getan.owner = InputOwner::InventoryContextMenu;
                        }
                    }
                    if ine[0] == false && ine[1] == true { // ._
                        input_getan.reset_short();
                        let idx = sett.selected_item;
                        sett.item.swap(idx, 6);
                    }
                } else if input_getan.now_setting == 3 {
                    if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                        input_getan.reset_short();
                        let slot = sett.selected_item;
                        if Item::use_(&mut use_item_writer, vec![slot], &sett.item[slot]) {
                            input_getan.reset_long();
                        } else {
                            input_getan.reset_short();
                        }
                    }
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
                    if ine[0] == true && ine[1] == false && ine[2] == false { // _..
                        input_getan.reset_short();
                        input_getan.owner = InputOwner::Gameplay;
                    }
                }
            } else if input_getan.owner == InputOwner::Gameplay {
                if sett.item[6] != Item::None {
                    let pos = freecam.position.xy();
                    commands.spawn((DrawedObject {
                        position: pos.extend(-0.2),
                        rotation: Quat::IDENTITY,
                        size: Vec3::ONE,
                        meshid: Meshes::mesh_Plane_smf,
                        textureid: sett.item[6].get_texture().unwrap(),
                        is_gui: false,
                        enabled: true
                    }, DropedItem {
                        item: sett.item[6].clone(),
                        position: pos
                    }
                    ));

                    sett.item[6] = Item::None;
                }
            }
        }
    }
}