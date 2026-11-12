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

const HUNGER_TIME: f32 = 900.0; // 15min
const THIRST_TIME: f32 = 540.0; // 9min



pub fn player_health_system(time: Res<Delta>, mut query: Query<&mut PlayerHealth>) {
    let mut rng = rand::thread_rng();

    if let Ok(mut health) = query.single_mut() {
        egui::Window::new("Health")
            .default_width(400.0)
            .show(&ui(), |ui| {
            health_food(time.delta, &mut health, ui);
        });        
    }
}

fn health_food(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {
    
}