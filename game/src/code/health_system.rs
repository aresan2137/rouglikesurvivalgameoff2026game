use bevy_ecs::prelude::*;
use egui::Ui;
use glam::*;
use rand::Rng;
use smq_engine::*;

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BodyPart {
    ArmLeft = 0,
    ArmRight = 1,
    LegLeft = 2,
    LegRight = 3,
    Torso = 4,
    Head = 5,
}

impl BodyPart {
    pub fn name(i: usize) -> &'static str {
        return match i {
            0 => "Left Arm",
            1 => "Right Arm",
            2 => "Left Leg",
            3 => "Right Leg",
            4 => "Torso",
            5 => "Head",
            _ => ""
        };
    }
}

const BODY_NEIGHBORS: [&[usize]; 6] = [
    &[BodyPart::Torso as usize],
    &[BodyPart::Torso as usize],
    &[BodyPart::Torso as usize],
    &[BodyPart::Torso as usize],
    &[
        BodyPart::ArmLeft as usize,
        BodyPart::ArmRight as usize,
        BodyPart::LegLeft as usize,
        BodyPart::LegRight as usize,
        BodyPart::Head as usize,
    ],
    &[BodyPart::Torso as usize],
];


#[derive(Component, Clone, Copy)]
pub struct BodyPartHealth {
    pub infection: f32,
    pub virus: f32,
    pub bleeding: f32,
    pub antiseptic: f32,
    pub soap: f32,
    pub health: f32,
    pub sterid: f32
}

#[derive(Component)]
pub struct PlayerHealth {
    pub hunger: f32,
    pub thirst: f32,
    pub antibiotic: f32,
    pub surowica: f32,
    pub blod_volume: f32,
    pub coagulant: f32,
    pub body_parts: [BodyPartHealth; 6]
}

const HUNGER_TIME: f32 = 1800.0; // 30min
const THIRST_TIME: f32 = 900.0; // 15min



pub fn player_health_system(time: Res<Delta>, mut query: Query<&mut PlayerHealth>) {
    let mut rng = rand::thread_rng();

    if let Ok(mut health) = query.single_mut() {
        // egui::Window::new("Health")
        //     .default_width(400.0)
        //     .show(&ui(), |ui| {
        //     health_food(time.delta, &mut health, ui);
        //     ui.separator();
        //     health_infection(time.delta, &mut health, ui);
        // });        
    }
}

fn health_food(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {
    health.hunger += delta * (1.0 / HUNGER_TIME);
    health.thirst += delta * (1.0 / THIRST_TIME);

    if health.hunger >= 1.0 {
        log::info!("you died: hunger");
    }

    if health.thirst >= 1.0 {
        log::info!("you died: thirst");
    }

    ui.label(format!("hunger: {:.4}; thirst: {:.4};", health.hunger, health.thirst));
}

fn health_infection(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {


    
    ui.label(format!("antibiotic: {:.4};", health.antibiotic));
    for (i, part) in health.body_parts.iter().enumerate() {
        ui.label(format!("{}; infection: {:.4}; antiseptic: {:.4};", BodyPart::name(i), part.infection, part.antiseptic));
    }
}