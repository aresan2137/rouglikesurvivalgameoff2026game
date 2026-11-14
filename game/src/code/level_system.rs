use bevy_ecs::prelude::*;
use glam::*;
use rand::Rng;

use crate::code::*;

pub const MAP_SIZE: USizeVec2 = USizeVec2::new(200, 200);
pub const TILE_SCALE: f32 = 1.0;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LevelElement {
    None,
    Sand,
    Tile,
    Wall,
    END
}

#[derive(Resource)]
pub struct Level {
    pub map: [LevelElement; MAP_SIZE.x * MAP_SIZE.y],
    pub resend_data: bool,
    pub level: usize
}

impl Default for Level {
    fn default() -> Self {
        return Self { 
            map: [LevelElement::None; MAP_SIZE.x * MAP_SIZE.y],
            resend_data: true,
            level: 0
        };
    }
}

impl Level {
    pub fn get_element_at_global_pos(&self, global_pos: Vec2) -> LevelElement {
        let tile = get_player_posion_tile(global_pos).round();

        if tile.x < -0.5 || tile.y < -0.5 || tile.x >= MAP_SIZE.x as f32 - 0.5 || tile.y >= MAP_SIZE.y as f32 - 0.5 {
            return LevelElement::None;
        }

        let idx = ((tile.y + 0.5) as usize) * MAP_SIZE.x + ((tile.x + 0.5) as usize);
        return self.map[idx];
    }
}

pub fn get_player_posion_tile(position: Vec2) -> Vec2 {
    return ((Vec2::new(position.x, -position.y) * (1.0 / TILE_SCALE)) + (MAP_SIZE.as_vec2()) - Vec2::ONE) / 2.0;
}

pub fn level_system(mut level: ResMut<Level>, mut player: Query<&mut PlayerSettings>) {
    let Ok(mut settings) = player.single_mut() else { return; };

    let tilepos = get_player_posion_tile(settings.pos.truncate());

    if tilepos.y < 0.5 {
        level.level += 1;
        level.generate_new_map();
        settings.pos = Vec3::new(0.0, (MAP_SIZE.x as f32 * -TILE_SCALE) + 5.0, 0.0);
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: isize,
    y: isize,
    w: isize,
    h: isize
}

impl Rect {
    fn right(&self) -> isize {
        self.x + self.w - 1
    }
    fn top(&self) -> isize {
        self.y + self.h - 1
    }
}

impl Level {
    pub fn generate_new_map(&mut self) {
        self.resend_data = true;
        let mut rng = rand::thread_rng();

        for y in 1..MAP_SIZE.y {
            for x in 0..MAP_SIZE.x {
                self.map[x + y * MAP_SIZE.x] = LevelElement::Sand;
            }
        }

        for x in 0..MAP_SIZE.x {
            self.map[x] = LevelElement::END;
        }
        

        let complex_count = rng.gen_range(35..50);

        for _ in 0..complex_count {
            let room_count = rng.gen_range(2..=5);
            let mut rooms: Vec<Rect> = Vec::with_capacity(room_count);
            let mut internal_doors: Vec<(isize, isize)> = Vec::new();

            let start_w = rng.gen_range(6..=14);
            let start_h = rng.gen_range(6..=14);
            let start_x = rng.gen_range(15..(MAP_SIZE.x as isize - 25));
            let start_y = rng.gen_range(15..(MAP_SIZE.y as isize - 25));

            let root_room = Rect {
                x: start_x,
                y: start_y,
                w: start_w,
                h: start_h,
            };

            if !self.is_area_free(&root_room, 2) {
                continue;
            }

            rooms.push(root_room);

            for _ in 1..room_count {
                let mut placed = false;

                for _ in 0..12 {
                    let parent = rooms[rng.gen_range(0..rooms.len())];
                    let w = rng.gen_range(6..=14);
                    let h = rng.gen_range(6..=14);
                    let side = rng.gen_range(0..4);

                    let (cand, door) = match side {
                        0 => {
                            let ox = rng.gen_range((parent.x - w + 3)..=(parent.right() - 2));
                            let rect = Rect { x: ox, y: parent.top(), w, h };
                            let overlap_min = parent.x.max(rect.x) + 1;
                            let overlap_max = parent.right().min(rect.right()) - 1;
                            let door_pos = if overlap_min <= overlap_max {
                                (rng.gen_range(overlap_min..=overlap_max), parent.top())
                            } else {
                                (parent.x + 1, parent.top())
                            };
                            (rect, door_pos)
                        }
                        1 => {
                            let ox = rng.gen_range((parent.x - w + 3)..=(parent.right() - 2));
                            let rect = Rect { x: ox, y: parent.y - h + 1, w, h };
                            let overlap_min = parent.x.max(rect.x) + 1;
                            let overlap_max = parent.right().min(rect.right()) - 1;
                            let door_pos = if overlap_min <= overlap_max {
                                (rng.gen_range(overlap_min..=overlap_max), parent.y)
                            } else {
                                (parent.x + 1, parent.y)
                            };
                            (rect, door_pos)
                        }
                        2 => {
                            let oy = rng.gen_range((parent.y - h + 3)..=(parent.top() - 2));
                            let rect = Rect { x: parent.x - w + 1, y: oy, w, h };
                            let overlap_min = parent.y.max(rect.y) + 1;
                            let overlap_max = parent.top().min(rect.top()) - 1;
                            let door_pos = if overlap_min <= overlap_max {
                                (parent.x, rng.gen_range(overlap_min..=overlap_max))
                            } else {
                                (parent.x, parent.y + 1)
                            };
                            (rect, door_pos)
                        }
                        _ => {
                            let oy = rng.gen_range((parent.y - h + 3)..=(parent.top() - 2));
                            let rect = Rect { x: parent.right(), y: oy, w, h };
                            let overlap_min = parent.y.max(rect.y) + 1;
                            let overlap_max = parent.top().min(rect.top()) - 1;
                            let door_pos = if overlap_min <= overlap_max {
                                (parent.right(), rng.gen_range(overlap_min..=overlap_max))
                            } else {
                                (parent.right(), parent.y + 1)
                            };
                            (rect, door_pos)
                        }
                    };

                    if cand.x < 5 || cand.right() >= (MAP_SIZE.x as isize - 5) ||
                       cand.y < 5 || cand.top() >= (MAP_SIZE.y as isize - 5) {
                        continue;
                    }

                    if self.is_area_free_for_complex(&cand, &rooms) {
                        rooms.push(cand);
                        internal_doors.push(door);
                        placed = true;
                        break;
                    }
                }

                if !placed {
                    break;
                }
            }

            for room in &rooms {
                for y in room.y..=room.top() {
                    for x in room.x..=room.right() {
                        let is_wall = x == room.x || x == room.right() || y == room.y || y == room.top();
                        let idx = x as usize + y as usize * MAP_SIZE.x;

                        if is_wall {
                            if self.map[idx] != LevelElement::Tile {
                                self.map[idx] = LevelElement::Wall;
                            }
                        } else {
                            self.map[idx] = LevelElement::Tile;
                        }
                    }
                }
            }

            for (dx, dy) in internal_doors {
                let idx = dx as usize + dy as usize * MAP_SIZE.x;
                self.map[idx] = LevelElement::Tile;
            }

            let mut outer_doors_placed = 0;
            let first_room = rooms[0];
            for _ in 0..15 {
                let side = rng.gen_range(0..4);
                let (ex, ey) = match side {
                    0 => (rng.gen_range((first_room.x + 1)..first_room.right()), first_room.y),
                    1 => (rng.gen_range((first_room.x + 1)..first_room.right()), first_room.top()),
                    2 => (first_room.x, rng.gen_range((first_room.y + 1)..first_room.top())),
                    _ => (first_room.right(), rng.gen_range((first_room.y + 1)..first_room.top())),
                };

                let idx = ex as usize + ey as usize * MAP_SIZE.x;
                if self.map[idx] == LevelElement::Wall {
                    self.map[idx] = LevelElement::Tile;
                    outer_doors_placed += 1;
                    if outer_doors_placed >= 1 {
                        break;
                    }
                }
            }
        }
    }

    fn is_area_free(&self, rect: &Rect, margin: isize) -> bool {
        let x_start = (rect.x - margin).max(0) as usize;
        let x_end = (rect.right() + margin).min(MAP_SIZE.x as isize - 1) as usize;
        let y_start = (rect.y - margin).max(0) as usize;
        let y_end = (rect.top() + margin).min(MAP_SIZE.y as isize - 1) as usize;

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                if self.map[x + y * MAP_SIZE.x] != LevelElement::Sand {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_area_free_for_complex(&self, rect: &Rect, current_rooms: &[Rect]) -> bool {
        let x_start = (rect.x - 2).max(0) as usize;
        let x_end = (rect.right() + 2).min(MAP_SIZE.x as isize - 1) as usize;
        let y_start = (rect.y - 2).max(0) as usize;
        let y_end = (rect.top() + 2).min(MAP_SIZE.y as isize - 1) as usize;

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                let elem = self.map[x + y * MAP_SIZE.x];
                if elem != LevelElement::Sand {
                    let in_current = current_rooms.iter().any(|r| {
                        (x as isize) >= r.x && (x as isize) <= r.right() &&
                        (y as isize) >= r.y && (y as isize) <= r.top()
                    });
                    if !in_current {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}