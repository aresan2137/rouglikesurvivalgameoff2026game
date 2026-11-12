use bevy_ecs::prelude::*;

use crate::code::*;

#[derive(Component)]
pub struct ContextMenu {
    pub health_tab: bool,
    pub index_normal: usize,
    pub index_health: usize
}

pub fn setup_inventory_contex_menu(world: &mut World) {
    
}

pub fn controler_inventory_context_menu(
    mut input_getan: ResMut<InputGetan>,
    mut context_menu: Query<&mut ContextMenu>
) {
    if let Ok(mut sett) = context_menu.single_mut() {
        if input_getan.owner == InputOwner::InventoryContextMenu {
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