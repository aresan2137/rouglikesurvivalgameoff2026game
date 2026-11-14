use bevy_ecs::prelude::*;
use egui::Ui;
use glam::*;
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
    pub godmode: bool,
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

const INFECTION_PROGRESION_TIME: f32 = 1800.0; // 30min
const INFECTION_REGRESION_TIME: f32 = 1200.0; // 20min

const INFECTION_ANTISEPTIC_LOSSE: f32 = 0.6;

const ANTISEPTIC_STAY_TIME: f32 = 480.0; // 8min
const ANTIBIOTIC_STAY_TIME: f32 = 840.0; // 14min

const INFECTION_SPREAD_VALUE: f32 = 0.8;

const INFECTION_DAMAGE_START: f32 = 0.4;
const INFECTION_DAMAGE_TIME: f32 = 900.0; // 15min

const HEALTH_RECOVERY_TIME: f32 = 1800.0; // 30min
const HEALTH_STERID_MULTIPLIER: f32 = 2.0;

const STERID_STAY_TIME: f32 = 1800.0; // 30min

const BLEEDING_CLOTHING_TIME: f32 = 20.0; // 20s/(1ml/s)

const BLEEDING_COAGULANT_MULTIPLIER: f32 = 3.0;

const COAGULANT_STAY_TIME: f32 = 480.0; // 8min

const BLOD_REGENERATION_TIME: f32 = 600.0; // 10min/L

const MINIMUM_BLOD_VOLUME: f32 = 2.0;

pub fn player_health_system(time: Res<Delta>, mut query: Query<&mut PlayerHealth>) {
    if let Ok(mut health) = query.single_mut() {

        // egui::Window::new("Health")
        //     .default_width(400.0)
        //     .show(&ui(), |ui| {
        //     health_food(time.delta, &mut health, ui);
        //     ui.separator();
        //     health_infection(time.delta, &mut health, ui);
        //     ui.separator();
        //     health_virus(time.delta, &mut health, ui);
        //     ui.separator();
        //     health_damage(time.delta, &mut health, ui);
        //     ui.separator();
        //     health_bleeding(time.delta, &mut health, ui);
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
    for i in 0..health.body_parts.len() {
        let part = &mut health.body_parts[i];

        if part.infection > 0.0 {
            if part.antiseptic > 0.0 {
                if part.infection > INFECTION_ANTISEPTIC_LOSSE {
                    part.infection += delta * (1.0 / INFECTION_PROGRESION_TIME) / 2.0;
                } else {
                    part.infection -= delta * (1.0 / INFECTION_REGRESION_TIME);
                }
            } else {
                if health.antibiotic > 0.0 {
                    part.infection -= delta * (1.0 / INFECTION_REGRESION_TIME);
                } else {
                    part.infection += delta * (1.0 / INFECTION_PROGRESION_TIME);
                }
            }
            if part.infection > INFECTION_DAMAGE_START {
                part.health -= delta * (1.0 / INFECTION_DAMAGE_TIME);
            }
        }

        part.antiseptic -= delta * (1.0 / ANTISEPTIC_STAY_TIME);

        part.antiseptic = part.antiseptic.max(0.0).min(1.0);
        part.infection = part.infection.max(0.0).min(1.0);

        if part.infection > INFECTION_SPREAD_VALUE {
            for neigbour in BODY_NEIGHBORS[i] {
                let infection = &mut health.body_parts[*neigbour].infection;

                *infection = infection.max(0.1);
            }
        }
    }

    health.antibiotic -= delta * (1.0 / ANTIBIOTIC_STAY_TIME);
    health.antibiotic = health.antibiotic.max(0.0).min(1.0);
    
    ui.label(format!("antibiotic: {:.4};", health.antibiotic));
    for (i, part) in health.body_parts.iter().enumerate() {
        ui.label(format!("{}; infection: {:.4}; antiseptic: {:.4};", BodyPart::name(i), part.infection, part.antiseptic));
    }
}

fn health_virus(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {
    for i in 0..health.body_parts.len() {
        let part = &mut health.body_parts[i];

        if part.virus > 0.0 {
            if part.soap > 0.0 {
                if part.virus > INFECTION_ANTISEPTIC_LOSSE / 4.0 {
                    part.virus += delta * (1.0 / INFECTION_PROGRESION_TIME) / 2.0  / 2.0;
                } else {
                    part.virus -= delta * (1.0 / INFECTION_REGRESION_TIME);
                }
            } else {
                if health.surowica > 0.0 {
                    part.virus -= delta * (1.0 / INFECTION_REGRESION_TIME);
                } else {
                    part.virus += delta * (1.0 / INFECTION_PROGRESION_TIME) / 2.0;
                }
            }
            if part.virus > INFECTION_DAMAGE_START {
                part.health -= delta * (1.0 / INFECTION_DAMAGE_TIME);
            }
        }

        part.soap -= delta * (1.0 / ANTISEPTIC_STAY_TIME);

        part.soap = part.soap.max(0.0).min(1.0);
        part.virus = part.virus.max(0.0).min(1.0);

        if part.virus > INFECTION_SPREAD_VALUE {
            for neigbour in BODY_NEIGHBORS[i] {
                let virus = &mut health.body_parts[*neigbour].virus;

                *virus = virus.max(0.1);
            }
        }
    }

    health.surowica -= delta * (1.0 / ANTIBIOTIC_STAY_TIME);
    health.surowica = health.surowica.max(0.0).min(1.0);
    
    ui.label(format!("surowica: {:.4};", health.surowica));
    for (i, part) in health.body_parts.iter().enumerate() {
        ui.label(format!("{}; virus: {:.4}; soap: {:.4};", BodyPart::name(i), part.virus, part.soap));
    }
}

fn health_damage(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {
    for i in 0..health.body_parts.len() {
        let part = &mut health.body_parts[i];

        if part.health < 0.01 {
            if i == BodyPart::Head as usize || i == BodyPart::Torso as usize {
                log::info!("you died: damage");
            } else {
                // TODO
            }
        }

        part.health += delta * (1.0 / HEALTH_RECOVERY_TIME) * if part.sterid > 0.0 { HEALTH_STERID_MULTIPLIER } else { 1.0 };
        part.health = part.health.max(0.0).min(1.0);

        part.sterid -= delta * (1.0 / STERID_STAY_TIME);
        part.sterid = part.sterid.max(0.0).min(1.0);
    }

    for (i, part) in health.body_parts.iter().enumerate() {
        ui.label(format!("{}; health: {:.4}; sterid: {:.4};", BodyPart::name(i), part.health, part.sterid));
    }
}

fn health_bleeding(delta: f32, health: &mut PlayerHealth, ui: &mut Ui) {
    for i in 0..health.body_parts.len() {
        let part = &mut health.body_parts[i];

        health.blod_volume -= delta * part.bleeding;
        
        part.bleeding -= delta * (1.0 / BLEEDING_CLOTHING_TIME) * if health.coagulant > 0.0 { BLEEDING_COAGULANT_MULTIPLIER } else { 1.0 };
        part.bleeding = part.bleeding.max(0.0);
    }

    health.coagulant -= delta * (1.0 / COAGULANT_STAY_TIME);
    health.coagulant = health.coagulant.max(0.0).min(1.0);

    health.blod_volume += delta * (1.0 / BLOD_REGENERATION_TIME);
    health.blod_volume = health.blod_volume.max(0.0).min(5.0);

    if health.blod_volume < MINIMUM_BLOD_VOLUME {
        log::info!("you died: blod loss");   
    }

    ui.label(format!("coagulant: {:.4}; blod volume: {:.4};", health.coagulant, health.blod_volume));
    for (i, part) in health.body_parts.iter().enumerate() {
        ui.label(format!("{}; bleeding: {:.4};", BodyPart::name(i), part.bleeding));
    }
}


