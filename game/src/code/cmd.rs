use bevy_ecs::prelude::*;
use egui::{self, Color32, ScrollArea, TextEdit};
use smq_engine::*;
use winit::keyboard::KeyCode;

use crate::{code::*, items::*};

#[derive(Resource)]
pub struct Console {
    pub is_open: bool,
    pub input_buffer: String,
    pub history: Vec<String>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            is_open: false,
            input_buffer: String::new(),
            history: Vec::new(),
        }
    }
}

pub fn console_system(
    mut console: ResMut<Console>,
    input: Res<Inputs>,
    mut inventory: Query<&mut InventorySettings>,
    mut health: Query<&mut PlayerHealth>,
    mut map: ResMut<Level>,
) {
    if input.pressed(KeyCode::Backquote) {
        console.is_open = !console.is_open;
        if console.is_open {
            console.input_buffer.retain(|c| c != '`' && c != '~');
        }
    }

    if !console.is_open {
        return;
    }

    let mut execute_cmd: Option<String> = None;

    egui::Window::new("Console")
        .default_width(600.0)
        .default_height(350.0)
        .collapsible(false)
        .resizable(true)
        .show(&ui(), |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(ui.available_height() - 35.0)
                .show(ui, |ui| {
                    for line in &console.history {
                        if line.starts_with(">") {
                            ui.colored_label(Color32::from_rgb(100, 200, 255), line);
                        } else if line.starts_with("[error]") {
                            ui.colored_label(Color32::from_rgb(255, 100, 100), line);
                        } else {
                            ui.label(line);
                        }
                    }
                });

            ui.separator();

            let response = ui.add(
                TextEdit::singleline(&mut console.input_buffer)
                    .hint_text("Type a command...")
                    .desired_width(f32::INFINITY),
            );

            if !response.has_focus() {
                response.request_focus();
            }

            if response.has_focus() && input.pressed(KeyCode::Enter) {
                let trimmed = console.input_buffer.trim().to_string();
                if !trimmed.is_empty() {
                    execute_cmd = Some(trimmed);
                    console.input_buffer.clear();
                }
                response.request_focus();
            }
        });

    if let Some(cmd) = execute_cmd {
        console.history.push(format!("> {cmd}"));
        process_command(&cmd, &mut console, &mut inventory, &mut health, &mut map);
    }
}

fn parse_body_part(raw: &str) -> Option<usize> {
    match raw.to_lowercase().as_str() {
        "arm_left" | "armleft" | "larm" | "0" => Some(BodyPart::ArmLeft as usize),
        "arm_right" | "armright" | "rarm" | "1" => Some(BodyPart::ArmRight as usize),
        "leg_left" | "legleft" | "lleg" | "2" => Some(BodyPart::LegLeft as usize),
        "leg_right" | "legright" | "rleg" | "3" => Some(BodyPart::LegRight as usize),
        "torso" | "chest" | "4" => Some(BodyPart::Torso as usize),
        "head" | "5" => Some(BodyPart::Head as usize),
        _ => None,
    }
}

fn parse_liquid(raw: &str) -> Option<ItemLiquidType> {
    match raw.to_lowercase().as_str() {
        "none" => Some(ItemLiquidType::None),
        "water" => Some(ItemLiquidType::Water),
        "contaminatedwater" | "cont_water" | "cwater" => Some(ItemLiquidType::ContaminatedWater),
        "applejuice" | "apple_juice" => Some(ItemLiquidType::AppleJuice),
        "orangejuice" | "orange_juice" => Some(ItemLiquidType::OrangeJuice),
        "gasoline" | "gas" => Some(ItemLiquidType::Gasoline),
        "antiserum" | "serum" => Some(ItemLiquidType::Antiserum),
        "coagulant" => Some(ItemLiquidType::Coagulant),
        "sterid" => Some(ItemLiquidType::Sterid),
        _ => None,
    }
}

fn process_command(
    raw_input: &str,
    console: &mut Console,
    inventory: &mut Query<&mut InventorySettings>,
    health: &mut Query<&mut PlayerHealth>,
    map: &mut Level,
) {
    let mut parts = raw_input.split_whitespace();
    let command = parts.next().unwrap_or("").to_lowercase();
    let args: Vec<&str> = parts.collect();

    match command.as_str() {
        "help" => {
            console.history.push("commands:".to_string());
            console.history.push("  clear              - clears console log".to_string());
            console.history.push("  set_item help      - help for set_item command".to_string());
            console.history.push("  set_health help    - help for health & body parts".to_string());
            console.history.push("  heal               - fully heals player & cures diseases".to_string());
            console.history.push("  godmode [on/off]   - toggles invincibility".to_string());
            console.history.push("  reset_map          - resets map".to_string());
        }
        "clear" => {
            console.history.clear();
        }
        "resetmap" | "reset_map" => {
            map.generate_new_map();
        }
        "godmode" | "god" => {
            let Ok(mut hp) = health.single_mut() else {
                console.history.push("[error] Could not find PlayerHealth entity.".to_string());
                return;
            };

            hp.godmode = match args.first().map(|s| s.to_lowercase()).as_deref() {
                Some("on") | Some("true") | Some("1") => true,
                Some("off") | Some("false") | Some("0") => false,
                None => !hp.godmode,
                Some(other) => {
                    console.history.push(format!(
                        "[error] Invalid argument '{other}'. Use on/off or leave empty."
                    ));
                    return;
                }
            };

            console
                .history
                .push(format!("Godmode: {}", if hp.godmode { "ON" } else { "OFF" }));
        }
        "heal" => {
            let Ok(mut hp) = health.single_mut() else {
                console.history.push("[error] Could not find PlayerHealth entity.".to_string());
                return;
            };

            hp.hunger = 100.0;
            hp.thirst = 100.0;
            hp.blod_volume = 5000.0;
            for part in hp.body_parts.iter_mut() {
                part.health = 100.0;
                part.bleeding = 0.0;
                part.infection = 0.0;
                part.virus = 0.0;
            }
            console.history.push("Player fully healed.".to_string());
        }
        "set_health" | "sethealth" => {
            if args.is_empty() || args[0].eq_ignore_ascii_case("help") {
                console.history.push("usage:".to_string());
                console.history.push("  set_health <global_stat> <value>".to_string());
                console
                    .history
                    .push("  set_health <body_part|all> <part_stat> <value>".to_string());
                console.history.push(
                    "global_stats: hunger, thirst, antibiotic, surowica, blood, coagulant".to_string(),
                );
                console.history.push(
                    "body_parts:   head, torso, larm, rarm, lleg, rleg, all".to_string(),
                );
                console.history.push(
                    "part_stats:   health, bleeding, infection, virus, antiseptic, soap, sterid"
                        .to_string(),
                );
                console.history.push("examples:".to_string());
                console.history.push("  set_health blood 3500".to_string());
                console.history.push("  set_health head health 50".to_string());
                console.history.push("  set_health larm bleeding 0.8".to_string());
                console.history.push("  set_health all infection 0".to_string());
                return;
            }

            let Ok(mut hp) = health.single_mut() else {
                console.history.push("[error] Could not find PlayerHealth entity.".to_string());
                return;
            };

            // Tryb 1: statystyka globalna
            if args.len() == 2 {
                let stat = args[0].to_lowercase();
                let Ok(val) = args[1].parse::<f32>() else {
                    console
                        .history
                        .push(format!("[error] Invalid float value: '{}'", args[1]));
                    return;
                };

                match stat.as_str() {
                    "hunger" => hp.hunger = val,
                    "thirst" => hp.thirst = val,
                    "antibiotic" => hp.antibiotic = val,
                    "surowica" | "antiserum" => hp.surowica = val,
                    "blood" | "blod" | "blod_volume" => hp.blod_volume = val,
                    "coagulant" => hp.coagulant = val,
                    _ => {
                        console
                            .history
                            .push(format!("[error] Unknown global stat: '{stat}'"));
                        return;
                    }
                }
                console.history.push(format!("Set {stat} to {val}."));
                return;
            }

            // Tryb 2: statystyka części ciała
            if args.len() == 3 {
                let target = args[0].to_lowercase();
                let stat = args[1].to_lowercase();
                let Ok(val) = args[2].parse::<f32>() else {
                    console
                        .history
                        .push(format!("[error] Invalid float value: '{}'", args[2]));
                    return;
                };

                let indices: Vec<usize> = if target == "all" {
                    (0..6).collect()
                } else if let Some(idx) = parse_body_part(&target) {
                    vec![idx]
                } else {
                    console
                        .history
                        .push(format!("[error] Unknown body part: '{target}'"));
                    return;
                };

                for idx in indices {
                    let part = &mut hp.body_parts[idx];
                    match stat.as_str() {
                        "health" | "hp" => part.health = val,
                        "bleeding" | "bleed" => part.bleeding = val,
                        "infection" => part.infection = val,
                        "virus" => part.virus = val,
                        "antiseptic" => part.antiseptic = val,
                        "soap" => part.soap = val,
                        "sterid" => part.sterid = val,
                        _ => {
                            console
                                .history
                                .push(format!("[error] Unknown body part stat: '{stat}'"));
                            return;
                        }
                    }
                }
                console
                    .history
                    .push(format!("Set {target} {stat} to {val}."));
                return;
            }

            console.history.push(
                "[error] Invalid arguments. Type 'set_health help' for usage.".to_string(),
            );
        }
        "set_item" | "setitem" => {
            if args.is_empty() || args[0].eq_ignore_ascii_case("help") {
                console
                    .history
                    .push("usage: set_item <slot 0..5> <item_name> [args...]".to_string());
                console.history.push("examples:".to_string());
                console.history.push("  set_item 0 none".to_string());
                console.history.push("  set_item 0 glock19 15".to_string());
                console.history.push("  set_item 1 flashlight 0.85".to_string());
                console
                    .history
                    .push("  set_item 2 bottle300ml water 250".to_string());
                console
                    .history
                    .push("  set_item 3 syringe antiserum 25".to_string());
                console.history.push("  set_item 4 proteinbar".to_string());
                return;
            }

            if args[0].eq_ignore_ascii_case("items") {
                console.history.push("list of items:".to_string());
                console.history.push("slots: 0..5".to_string());
                console.history.push("liquids: none, water, contaminatedwater, applejuice, orangejuice, gasoline, antiserum, coagulant, sterid".to_string());
                console.history.push("items:".to_string());
                console.history.push("  none".to_string());
                console.history.push("Guns:".to_string());
                console.history.push("  glock19 [ammo: u8, def: 15]".to_string());
                console.history.push("  ak47 [ammo: u8, def: 30]".to_string());
                console.history.push("  shotgun [ammo: u8, def: 8]".to_string());
                console.history.push("Ammo:".to_string());
                console.history.push("  ammo9mm [ammo: u8, def: 30]".to_string());
                console.history.push("  ammo7_62mm [ammo: u8, def: 30]".to_string());
                console.history.push("  shotgunshells [ammo: u8, def: 12]".to_string());
                console.history.push("Tools:".to_string());
                console
                    .history
                    .push("  flashlight [energy: f32 (0.0..1.0), def: 1.0]".to_string());
                console
                    .history
                    .push("  torch [fuel: f32 (0.0..1.0), def: 1.0]".to_string());
                console.history.push("  crowbar".to_string());
                console
                    .history
                    .push("  lighter [fuel: f32 (0.0..1.0), def: 1.0]".to_string());
                console.history.push("  knife".to_string());
                console.history.push("  flintandsteel".to_string());
                console.history.push("Food:".to_string());
                console.history.push("  proteinbar".to_string());
                console.history.push("  banana".to_string());
                console.history.push("  apple".to_string());
                console.history.push("  cereal [uses: u8, def: 4]".to_string());
                console.history.push("  bread [uses: u8, def: 4]".to_string());
                console.history.push("  foodration [uses: u8, def: 4]".to_string());
                console.history.push("  rottenfood".to_string());
                console.history.push("Health:".to_string());
                console
                    .history
                    .push("  bandage [left: f32 (0.0..1.0), def: 1.0]".to_string());
                console.history.push(
                    "  sterilebandage [left: f32 (0.0..1.0), def: 1.0]".to_string(),
                );
                console
                    .history
                    .push("  antibiotic [uses: u8, def: 10]".to_string());
                console
                    .history
                    .push("  antiseptic [fill: f32 (0.0..1.0), def: 1.0]".to_string());
                console
                    .history
                    .push("  soap [left: f32 (0.0..1.0), def: 1.0]".to_string());
                console.history.push(
                    "  syringe [liquid, def: none] [fill: f32, def: 25.0]".to_string(),
                );
                console.history.push("Containers / Misc:".to_string());
                console.history.push(
                    "  bottle300ml [liquid, def: none] [fill: f32, def: 300.0]".to_string(),
                );
                console.history.push(
                    "  bottle500ml [liquid, def: none] [fill: f32, def: 500.0]".to_string(),
                );
                console.history.push(
                    "  bottle1l [liquid, def: none] [fill: f32, def: 1000.0]".to_string(),
                );
                console.history.push(
                    "  bottle2l [liquid, def: none] [fill: f32, def: 2000.0]".to_string(),
                );
                console.history.push("  cloth".to_string());
                console.history.push(
                    "  waterfilter [uses: f32 (0.0..1.0), def: 1.0]".to_string(),
                );
                console.history.push("  backpack".to_string());
                console.history.push("  plasticbag".to_string());
                return;
            }

            if args.len() < 2 {
                console
                    .history
                    .push("[error] set_item requires at least slot and item name.".to_string());
                return;
            }

            let slot: usize = match args[0].parse() {
                Ok(s) if s < 6 => s,
                _ => {
                    console
                        .history
                        .push("[error] Slot must be an integer between 0 and 5.".to_string());
                    return;
                }
            };

            let item_name = args[1].to_lowercase();
            let sub_args = &args[2..];

            let parsed_item = match item_name.as_str() {
                "none" => Some(Item::None),

                // guns
                "glock19" | "glock" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(15);
                    Some(Item::Glock19((ammo, 0.0)))
                }
                "ak47" | "ak" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(30);
                    Some(Item::AK47((ammo, 0.0)))
                }
                "shotgun" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(8);
                    Some(Item::Shotgun((ammo, 0.0)))
                }

                // ammo
                "ammo9mm" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(30);
                    Some(Item::Ammo9mm(ammo))
                }
                "ammo7_62mm" | "ammo762" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(30);
                    Some(Item::Ammo7_62mm(ammo))
                }
                "shotgunshells" | "shells" => {
                    let ammo = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(12);
                    Some(Item::Shotgunshells(ammo))
                }

                // tools
                "flashlight" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Flashlight((val, false)))
                }
                "torch" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Torch((val, false)))
                }
                "crowbar" => Some(Item::Crowbar),
                "lighter" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Lighter(val))
                }
                "knife" => Some(Item::Knife),
                "flintandsteel" | "flint" => Some(Item::FlintAndSteel),

                // food
                "proteinbar" => Some(Item::ProteinBar),
                "banana" => Some(Item::Banana),
                "apple" => Some(Item::Apple),
                "cereal" => {
                    let uses = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(4);
                    Some(Item::Cereal(uses))
                }
                "bread" => {
                    let uses = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(4);
                    Some(Item::Bread(uses))
                }
                "foodration" | "ration" => {
                    let uses = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(4);
                    Some(Item::FoodRation(uses))
                }
                "rottenfood" => Some(Item::RottenFood),

                // health
                "bandage" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Bandage(val))
                }
                "sterilebandage" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::SterileBandage(val))
                }
                "antibiotic" => {
                    let uses = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(10);
                    Some(Item::AntiBiotic(uses))
                }
                "antiseptic" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Antiseptic(val))
                }
                "soap" => {
                    let val = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::Soap(val))
                }
                "syringe" => {
                    let liquid = sub_args
                        .first()
                        .and_then(|l| parse_liquid(l))
                        .unwrap_or(ItemLiquidType::None);
                    let fill = sub_args.get(1).and_then(|v| v.parse().ok()).unwrap_or(25.0);
                    Some(Item::Syringe((liquid, fill)))
                }

                // items & containers
                "bottle300ml" => {
                    let liquid = sub_args
                        .first()
                        .and_then(|l| parse_liquid(l))
                        .unwrap_or(ItemLiquidType::None);
                    let fill = sub_args.get(1).and_then(|v| v.parse().ok()).unwrap_or(300.0);
                    Some(Item::Bottle300ml((liquid, fill)))
                }
                "bottle500ml" => {
                    let liquid = sub_args
                        .first()
                        .and_then(|l| parse_liquid(l))
                        .unwrap_or(ItemLiquidType::None);
                    let fill = sub_args.get(1).and_then(|v| v.parse().ok()).unwrap_or(500.0);
                    Some(Item::Bottle500ml((liquid, fill)))
                }
                "bottle1l" => {
                    let liquid = sub_args
                        .first()
                        .and_then(|l| parse_liquid(l))
                        .unwrap_or(ItemLiquidType::None);
                    let fill = sub_args.get(1).and_then(|v| v.parse().ok()).unwrap_or(1000.0);
                    Some(Item::Bottle1l((liquid, fill)))
                }
                "bottle2l" => {
                    let liquid = sub_args
                        .first()
                        .and_then(|l| parse_liquid(l))
                        .unwrap_or(ItemLiquidType::None);
                    let fill = sub_args.get(1).and_then(|v| v.parse().ok()).unwrap_or(2000.0);
                    Some(Item::Bottle2l((liquid, fill)))
                }
                "cloth" => Some(Item::Cloth),
                "waterfilter" => {
                    let uses = sub_args.first().and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    Some(Item::WaterFilter(uses))
                }
                "backpack" => Some(Item::Backpack(Vec::new())),
                "plasticbag" => Some(Item::PlasticBag(Vec::new())),

                _ => None,
            };

            match parsed_item {
                Some(item) => match inventory.single_mut() {
                    Ok(mut inv) => {
                        inv.item[slot] = item;
                        console
                            .history
                            .push(format!("Slot {} updated successfully.", slot));
                    }
                    Err(_) => {
                        console.history.push(
                            "[error] Could not find InventorySettings entity.".to_string(),
                        );
                    }
                },
                None => {
                    console
                        .history
                        .push(format!("[error] Unknown item name: '{item_name}'"));
                }
            }
        }
        _ => {
            console
                .history
                .push(format!("[error] Unknown command: '{command}'"));
        }
    }
}