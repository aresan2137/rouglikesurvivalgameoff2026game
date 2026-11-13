use bevy_ecs::prelude::*;
use glam::*;

use crate::{code::*, gen_bake::*, renderer::DrawedObject};

const MAX_CONTEXT_ELEMENTS: usize = 6;

#[derive(Component)]
pub struct ContextMenu {
    pub health_tab: bool,
    pub index_normal: usize,
    pub index_health: usize
}

#[derive(Component)]
pub struct ContextEmementBG {}


#[derive(Component)]
pub struct ContextEmementItem {}


pub fn start_inventory_context(world: &mut World, schedule: &mut Schedule) {
    world.spawn((DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::new(4.0, 4.0, 4.0),
        meshid: Meshes::mesh_Plane_smf,
        textureid: Textures::sprites_body_png,
        is_gui: true,
        enabled: true
    }, ContextMenu {
        health_tab: false,
        index_normal: 0,
        index_health: 0
    }));
    for _ in 0..MAX_CONTEXT_ELEMENTS {
        world.spawn((DrawedObject {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            size: Vec3::ONE,
            meshid: Meshes::mesh_Plane_smf,
            textureid: Textures::sprites_contextBG_png,
            is_gui: true,
            enabled: true
        }, ContextEmementBG {}
        ));

        world.spawn((DrawedObject {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            size: Vec3::ONE,
            meshid: Meshes::mesh_Plane_smf,
            textureid: Textures::sprites_items_ProteinBar_png,
            is_gui: true,
            enabled: true
        }, ContextEmementItem {}
        ));
    }

    schedule.add_systems((
        controler_inventory_context_menu,
        gui_update
    ));

}

fn gui_update(
    input_getan: Res<InputGetan>,
    mut inventory_menu: Query<&mut InventorySettings>,
    mut elements_bg: Query<&mut DrawedObject, With<ContextEmementBG>>,
    mut elements_item: Query<&mut DrawedObject, (With<ContextEmementItem>, Without<ContextEmementBG>)>,
    mut guy: Query<(&mut DrawedObject, &ContextMenu), (Without<ContextEmementBG>, Without<ContextEmementItem>)>
) {
    let gui_enabled = input_getan.owner == InputOwner::InventoryContextMenu;
    
    let Ok(inventory) = inventory_menu.single_mut() else { return; };

    let Ok((mut obj, menu)) = guy.single_mut() else { return; };

    obj.enabled = gui_enabled && menu.health_tab;

    let textures = inventory.item[inventory.selected_item].context_menu_normal_slot_requiriment_with_aproprieate_texturing();
    let slots_used = if menu.health_tab { 6 } else { textures.len() };

    let mut positions = [Vec3::ZERO; MAX_CONTEXT_ELEMENTS];

    let radius: f32 = 5.5;

    if menu.health_tab {
        for i in 0..6 {
            let angle = (std::f32::consts::PI / 6.0) - (i as f32) * (std::f32::consts::PI / 3.0);

            let x = angle.cos() * radius;
            let y = angle.sin() * radius;

            positions[i] = Vec3::new(x, y, -0.51);
        }
    } else {
        for i in 0..slots_used {
            let step = 1.8 / slots_used as f32 + 0.4;
            

            let offset = (i as f32) - (slots_used as f32 - 1.0) / 2.0;
            let angle = std::f32::consts::FRAC_PI_2 + offset * step;

            let x = angle.cos() * radius;
            let y = angle.sin() * radius;

            positions[i] = Vec3::new(x, y, -0.51);    
        }
    }


    for (i, mut element_bg) in elements_bg.iter_mut().enumerate() {
        element_bg.enabled = i < slots_used && gui_enabled;

        if i < slots_used && gui_enabled {
            element_bg.position = positions[i];
        }
    }

    for (i, mut element_item) in elements_item.iter_mut().enumerate() {
        element_item.enabled = i < slots_used && gui_enabled && !menu.health_tab;

        if i < slots_used && gui_enabled && !menu.health_tab {
            element_item.textureid = textures[i];
            
            element_item.position = positions[i];
        }
    }
} 

fn controler_inventory_context_menu(
    mut input_getan: ResMut<InputGetan>,
    mut context_menu: Query<&mut ContextMenu>,
    mut inventory_menu: Query<&mut InventorySettings>
) {
    let gui_enabled = input_getan.owner == InputOwner::InventoryContextMenu;
    
    let Ok(inventory) = inventory_menu.single_mut() else { return; };

    if let Ok(mut sett) = context_menu.single_mut() {
        if gui_enabled {
            let ine = input_getan.morse_input;
            if input_getan.now_setting == 2 {
                if ine[0] == true && ine[1] == true { // __
                    input_getan.reset_short();
                    input_getan.owner = InputOwner::Inventory;
                    sett.index_normal = 0;
                    sett.index_health = 0;
                }
                if ine[0] == false && ine[1] == true { // ._
                    input_getan.reset_short();
                    log::info!("contex menu: ._");
                }
            } else if input_getan.now_setting == 3 {
                if ine[0] == true && ine[1] == false && ine[2] == true  { // _._
                    input_getan.reset_short();
                }
                if ine[0] == false && ine[1] == false && ine[2] == true  { // .._
                    input_getan.reset_short();
                }
                if ine[0] == true && ine[1] == false && ine[2] == false  { // _..
                    input_getan.reset_short();
                    sett.health_tab = !sett.health_tab;
                    sett.index_normal = 0;
                    sett.index_health = 0;
                }
                if ine[0] == false && ine[1] == false && ine[2] == false  { // ...
                    input_getan.reset_short();
                }
            }
        }
    }
}