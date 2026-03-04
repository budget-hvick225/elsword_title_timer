use crate::types::loader::*;

use crate::MyKey;

use crate::helpers::*;

use eframe::egui;

use std::collections::HashSet;

pub fn show(ui: &mut egui::Ui, tracker: &mut Tracker) {

    ui.heading("Settings");

    ui.label("In case of specific keys not working, consider checking Debug->Duplicate Key Check section!");
    
    ui.separator();

    ui.collapsing("Layout", |ui| {
        ui.checkbox(&mut tracker.config.horizontal, "Horizontal Layout");
        ui.checkbox(&mut tracker.config.fs, "Enable FS Cooldown");
        ui.checkbox(&mut tracker.config.tss, "Enable TSS Cooldown");
        ui.checkbox(&mut tracker.config.np, "Enable NP Cooldown");
        ui.checkbox(&mut tracker.config.flow, "Enable Flow Cooldown");
        ui.checkbox(&mut tracker.config.grove, "Enable Grove Cooldown");
        ui.checkbox(&mut tracker.config.lithia_awk, "Enable Lithia Cooldown");
    });

    ui.separator();

    // ---------------- Layout ----------------
    ui.collapsing("Keybinds", |ui| {

        ui.collapsing("Skills", |ui| {
            let skill_waiting_id = ui.make_persistent_id("waiting_for_skill");
            let mut waiting_for_skill = ui.data_mut(|d| {
                d.get_temp::<Option<usize>>(skill_waiting_id).unwrap_or(None)
            });

            let modifier_capture_id = ui.make_persistent_id("modifier_captured");
            let mut captured_modifier = ui.data_mut(|d| {
                d.get_temp::<Option<String>>(modifier_capture_id).unwrap_or(None)
            });

            // Show all skills
            for (i, skill) in tracker.config.skills.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Skill{}: {}", i + 1, skill));

                    if styled_button(ui, "Change Keybind").clicked() {
                        waiting_for_skill = Some(i);
                    }
                });
            }

            if let Some(identifier) = waiting_for_skill {
                ui.label(format!("Press any key for Skill{}...", identifier + 1));

                let key_result = capture::capture_egui_keys(ui, &mut true, &mut captured_modifier);

                if let Some((key_opt, modifier_opt)) = key_result {
                    if let Some(key) = key_opt {
                        // Key (or converted modifier) has been selected
                        tracker.config.skills[identifier] = key.to_string();
                        tracker.config.update();

                        waiting_for_skill = None;
                        captured_modifier = None;
                    } else if let Some(modifier) = modifier_opt {
                        captured_modifier = Some(modifier);
                    }
                }
            }
            
            ui.data_mut(|d| {
                d.insert_temp(skill_waiting_id, waiting_for_skill);
                d.insert_temp(modifier_capture_id, captured_modifier.clone());
            });
        });

        ui.collapsing("Consumables", |ui| {
            
            let consumable_waiting_id = ui.make_persistent_id("waiting_for_consumable");
            let mut waiting_for_consumable = ui.data_mut(|d| d.get_temp::<Option<usize>>(consumable_waiting_id).unwrap_or(None));

            let modifier_capture_id = ui.make_persistent_id("modifier_captured");
            let mut captured_modifier = ui.data_mut(|d| {
                d.get_temp::<Option<String>>(modifier_capture_id).unwrap_or(None)
            });

            for (i, consumable) in tracker.config.consumables.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Consumable{}: {}", i+1, consumable));

                    if styled_button(ui, "Change Keybind").clicked() {
                        waiting_for_consumable = Some(i);
                    }
                });
            }

            if let Some(identifier) = waiting_for_consumable {
                ui.label(format!("Press any key for Consumable{}...", identifier + 1));

                let key_result = capture::capture_egui_keys(ui, &mut true, &mut captured_modifier);

                if let Some((key_opt, modifier_opt)) = key_result {
                    if let Some(key) = key_opt {
                        // Key (or converted modifier) has been selected
                        tracker.config.consumables[identifier] = key.to_string();
                        tracker.config.update();

                        waiting_for_consumable = None;
                        captured_modifier = None;
                    } else if let Some(modifier) = modifier_opt {
                        captured_modifier = Some(modifier);
                    }
                }
            }

            ui.data_mut(|d| {
                d.insert_temp(consumable_waiting_id, waiting_for_consumable);
                d.insert_temp(modifier_capture_id, captured_modifier.clone());
            });
        });

        ui.collapsing("Awakening", |ui| {
            let key_waiting_id = ui.make_persistent_id("waiting_for_key");
            let mut waiting_for_key = ui.data_mut(|d| d.get_temp::<Option<&str>>(key_waiting_id).unwrap_or(None));

            let modifier_capture_id = ui.make_persistent_id("modifier_captured");
            let mut captured_modifier = ui.data_mut(|d| {
                d.get_temp::<Option<String>>(modifier_capture_id).unwrap_or(None)
            });

            ui.horizontal(|ui| {
                ui.label(format!("Awakening Key: {}", tracker.config.awakening));
                if styled_button(ui, "Change Awakening Key").clicked() {
                    waiting_for_key = Some("Awakening");
                }
            });

            ui.horizontal(|ui| {
                let onion_label = if tracker.config.onion.is_empty() {
                    "Empty".to_string()
                } else {
                    tracker.config.onion.clone()
                };

                ui.label(format!("Onion Slot: {}", onion_label));

                if styled_button(ui, "Change Onion Key").clicked() {
                    waiting_for_key = Some("Onion");
                }

                if styled_button(ui, "Reset Onion").clicked() {
                    tracker.config.onion = "".to_string();
                    tracker.config.update();
                }
            });

            if let Some(slot) = waiting_for_key {
                ui.label(format!("Press any key to change {}...", slot));
                
                let key_result = capture::capture_egui_keys(ui, &mut true, &mut captured_modifier);

                if let Some((key_opt, modifier_opt)) = key_result {
                    if let Some(key) = key_opt {
                        match slot {
                            "Awakening" => tracker.config.awakening = key.to_string(),
                            "Onion" => tracker.config.onion = key.to_string(),
                            _ => {}
                        }
                        tracker.config.update();
                        waiting_for_key = None;
                    } else if let Some(modifier) = modifier_opt {
                        captured_modifier = Some(modifier);
                    }
                }
            }

            ui.data_mut(|d| {
                d.insert_temp(key_waiting_id, waiting_for_key);
                d.insert_temp(modifier_capture_id, captured_modifier.clone());
            });
        });

        ui.collapsing("Titles", |ui| {
            let waiting_id = ui.make_persistent_id("waiting_for_title");
            let mut waiting_for_title = ui.data_mut(|d| d.get_temp::<Option<&str>>(waiting_id).unwrap_or(None));

            let modifier_capture_id = ui.make_persistent_id("modifier_captured");
            let mut captured_modifier = ui.data_mut(|d| {
                d.get_temp::<Option<String>>(modifier_capture_id).unwrap_or(None)
            });

            ui.horizontal(|ui| {
                ui.label(format!("FS Key: {}", tracker.config.fskey));
                if styled_button(ui, "Change FS Key").clicked() {
                    waiting_for_title = Some("FS");
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("NP Key: {}", tracker.config.npkey));
                if styled_button(ui, "Change NP Key").clicked() {
                    waiting_for_title = Some("NP");
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("TSS Key: {}", tracker.config.tsskey));
                if styled_button(ui, "Change TSS Key").clicked() {
                    waiting_for_title = Some("TSS");
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("Another Key: {}", tracker.config.anotherkey));
                if styled_button(ui, "Change Another Key").clicked() {
                    waiting_for_title = Some("Another");
                }
            });

            ui.horizontal(|ui| {
                ui.label(format!("Select Title: {}", tracker.config.select));
                if styled_button(ui, "Edit Select Title").clicked() {
                    waiting_for_title = Some("Select")
                }
            });

            let input = ui.input(|i| i.clone());

            if let Some(title) = waiting_for_title {
                ui.label(format!("Press any key for title {}...", title));

                let key_result = capture::capture_egui_keys(ui, &mut true, &mut captured_modifier);

                if let Some((key_opt, modifier_opt)) = key_result {
                    if let Some(key) = key_opt {
                        match title {
                            "FS" => tracker.config.fskey = key.to_string(),
                            "NP" => tracker.config.npkey = key.to_string(),
                            "TSS" => tracker.config.tsskey = key.to_string(),
                            "Another" => tracker.config.anotherkey = key.to_string(),
                            "Select" => tracker.config.select = key.to_string(),
                            _ => {}
                        }
                        tracker.config.update();
                        waiting_for_title = None;
                    } else if let Some(modifier) = modifier_opt {
                        captured_modifier = Some(modifier);
                    }
                }
            }

            ui.data_mut(|d| {
                d.insert_temp(waiting_id, waiting_for_title);
                d.insert_temp(modifier_capture_id, captured_modifier.clone());
            });
        });

        ui.collapsing("Misc", |ui| {
            let misc_id = ui.make_persistent_id("modifier_captured");
            let mut waiting_for_misc = ui.data_mut(|d| d.get_temp::<Option<&str>>(misc_id).unwrap_or(None));

            let modifier_capture_id = ui.make_persistent_id("captured_modifier");
            let mut captured_modifier = ui.data_mut(|d| d.get_temp::<Option<String>>(modifier_capture_id).unwrap_or(None));

            ui.horizontal(|ui| {
                ui.label(format!("Reset Key: {}", tracker.config.resetkey));
                if styled_button(ui, "Change Reset Key").clicked() {
                    waiting_for_misc = Some("Reset");
                }
            });

            if let Some(misc) = waiting_for_misc {
                ui.label(format!("Press any key for {} Key...", misc));

                let key_result = capture::capture_egui_keys(ui, &mut true, &mut captured_modifier);

                if let Some((key_opt, modifier_opt)) = key_result {
                    if let Some(key) = key_opt {
                        match misc {
                            "Reset" => tracker.config.resetkey = key.to_string(),
                            _ => {}
                        }
                        tracker.config.update();
                        waiting_for_misc = None;
                    } else if let Some(modifier) = modifier_opt {
                        captured_modifier = Some(modifier);
                    }
                }
            }

            ui.data_mut(|d| {
                d.insert_temp(misc_id, waiting_for_misc);
                d.insert_temp(modifier_capture_id, captured_modifier.clone());
            });
        });
    
    });

    ui.separator();

    ui.collapsing("Image Size", |ui| {
        // Persistent editing strings stored locally in this frame
        // Use `Ui::data_mut` to store temporary strings between frames
        let width_id = ui.make_persistent_id("image_width_str");
        let height_id = ui.make_persistent_id("image_height_str");

        let mut width_str = ui.data_mut(|d| {
            d.get_temp::<String>(width_id).unwrap_or_else(|| tracker.config.imagesize[0].to_string())
        });
        let mut height_str = ui.data_mut(|d| {
            d.get_temp::<String>(height_id).unwrap_or_else(|| tracker.config.imagesize[1].to_string())
        });

        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.text_edit_singleline(&mut width_str);
        });

        ui.horizontal(|ui| {
            ui.label("Height:");
            ui.text_edit_singleline(&mut height_str);
        });

        if styled_button(ui, "Update Size").clicked() {
            if let Ok(width) = width_str.parse::<f32>() {
                tracker.config.imagesize[0] = width;
            }
            if let Ok(height) = height_str.parse::<f32>() {
                tracker.config.imagesize[1] = height;
            }
        }

        // Save the editing strings so they persist while typing
        ui.data_mut(|d| {
            d.insert_temp(width_id, width_str);
            d.insert_temp(height_id, height_str);
        });
    });

    ui.separator();

    // ---------------- Reset ----------------

    if styled_button(ui, "Save Settings").clicked() {
        tracker.config.save();
    }

    if styled_button(ui, "Reset All Cooldowns").clicked() {
        tracker.fs.reset();
        tracker.tss.reset();
        tracker.np.reset();
        tracker.flow.reset();
    }
}

fn styled_button(
    ui: &mut egui::Ui,
    text: &str,
) -> egui::Response {
    let button = egui::Button::new(
        egui::RichText::new(text).color(egui::Color32::BLACK)
    )
    .fill(egui::Color32::from_gray(200));

    ui.add(button)
}