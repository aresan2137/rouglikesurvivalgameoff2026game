use bevy_ecs::prelude::*;
use glam::*;
use smq_engine::*;

use crate::{code::*, renderer::DrawedObject};

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

#[derive(Component, Clone, Copy)]
pub struct BodyPartHealth {
    pub infection: f32,
    pub virus: f32,
    pub bleeding: f32,
    pub antiseptic: f32,
    pub soap: f32,
    pub health: f32
}

#[derive(Component)]
pub struct PlayerHealth {
    pub hunger: f32,
    pub thirst: f32,
    pub antibiotic: f32,
    pub surowica: f32,
    pub blod_volume: f32,
    pub body_parts: [BodyPartHealth; 6]
}