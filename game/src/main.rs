use bevy_ecs::prelude::*;
use glam::*;

use smq_engine::*;

use wgpu::*;

mod gen_bake;

mod code;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

use crate::{code::*, gen_bake::*, renderer::{DrawedObject, Renderer, load_assets, render}};

mod gen_game;

mod renderer;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[allow(unused)]
fn main() {
    logger::init();

    let mut world = World::new();
    world.insert_resource(Delta::new());
    world.insert_resource(Inputs::default());
    world.insert_resource(FrameStats::default());
    world.insert_resource(InputGetan::default());
    world.insert_resource(Console::default());

    let mut schedule = Schedule::default();
    schedule.add_systems(fps_logger);
    schedule.add_systems(read_input);
    schedule.add_systems(controler_gameplay);
    schedule.add_systems(controler_inventory);
    schedule.add_systems(controler_inventory_context_menu);
    schedule.add_systems(controler_health_managment);
    schedule.add_systems(toggle_console_system);
    schedule.add_systems(tick_items);
    schedule.add_systems(bullet_provider);

    world.spawn((FreeCamera {
        position: Vec3::ZERO,
        pitch: 0.0,
        yaw: 0.0,
        speed: 0.0,
        sensitivity: 0.0,
        is_controlling: false
    }, DrawedObject {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        size: Vec3::new(0.5, 0.5, 0.5),
        meshid: Meshes::mesh_Plane_smf,
        textureid: Textures::sprites_plr_png,
        is_gui: false,
        enabled: true
    }, PlayerSettings {
        rotation: 0.0
    }, PlayerHealth {
        hunger: 0.0,
        thirst: 0.0,
        antibiotic: 0.0,
        surowica: 0.0,
        blod_volume: 5.0,
        body_parts: [
            BodyPartHealth {
                infection: 0.0,
                virus: 0.0,
                bleeding: 0.0,
                antiseptic: 0.0,
                soap: 0.0,
                health: 1.0
            }; 6
        ]
    }
    ));

    world.spawn(DrawedObject {
        position: Vec3::new(2.0, 2.0, 0.0),
        rotation: Quat::IDENTITY,
        size: Vec3::ONE,
        meshid: Meshes::mesh_Plane_smf,
        textureid: Textures::sprites_ProteinBar_png,
        is_gui: false,
        enabled: true
    });

    setup_inventory(&mut world);

    let event_loop = EventLoop::with_user_event().build().unwrap();

    let mut context: Context<'_, Renderer> = Context::new(
        world,
        schedule,
        event_loop.create_proxy(),
        ContextSettings {
            present_mode: PresentMode::AutoVsync,
        },
        ContextEvents {
            renderer: Some(render),
            on_wgpu_load: Some(load_assets),
            pre_schedule: None,
            on_exit: None,
        },
    );

    event_loop.run_app(&mut context).unwrap();
}
