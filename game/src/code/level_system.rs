use bevy_ecs::prelude::*;
use glam::*;
use rand::Rng;

pub const MAP_SIZE: USizeVec2 = USizeVec2::new(200, 200);
pub const TILE_SCALE: f32 = 0.5;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LevelElement {
    None,
    Sand,
    Tile,
    Wall
}

#[derive(Resource)]
pub struct Level {
    pub map: [LevelElement; MAP_SIZE.x * MAP_SIZE.y],
    pub resend_data: bool
}

impl Default for Level {
    fn default() -> Self {
        return Self { 
            map: [LevelElement::None; MAP_SIZE.x * MAP_SIZE.y],
            resend_data: true
        };
    }
}

impl Level {
    pub fn generate_new_map(&mut self) {
        let mut rng = rand::thread_rng();

        const TILES: [LevelElement; 2] = [LevelElement::Sand, LevelElement::Tile];

        for y in 0..MAP_SIZE.y {
            for x in 0..MAP_SIZE.x {
                let idx = rng.gen_range(0..TILES.len());
                self.map[x + y * MAP_SIZE.x] = TILES[idx];
            }
        }

        self.resend_data = true;
    }
}