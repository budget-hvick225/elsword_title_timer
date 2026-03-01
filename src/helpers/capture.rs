use crate::types::loader::*;

use crate::MyKey;

use crate::helpers::*;

use eframe::egui;

pub fn capture_egui_keys(
    ui: &mut egui::Ui,
    waiting: &mut bool,
    captured_modifier: &mut Option<String>, // <- pass the context
) -> Option<(Option<MyKey>, Option<String>)> {
    if !*waiting {
        return None;
    }

    let input = ui.input(|i| i.clone());

    // Capture normal keys
    for event in &input.events {
        if let egui::Event::Key { key, pressed: true, repeat: false, .. } = event {
            if let Some(mykey) = MyKey::egui_to_mykey(key) {
                *waiting = false;
                *captured_modifier = None; // reset modifier when key is pressed
                return Some((Some(mykey), None));
            }
        }
    }

    if input.modifiers.shift {
        *captured_modifier = Some("Shift".to_string());
    } else if input.modifiers.ctrl {
        *captured_modifier = Some("Control".to_string());
    } else if input.modifiers.alt {
        *captured_modifier = Some("Alt".to_string());
    }

    if let Some(modifier) = captured_modifier.clone() {
        ui.label("Modifier detected!");
        if modifier == "Alt" {
            ui.label("Left Alt will display as Alt, Right Alt as AltGr.");
        }

        ui.label(format!("Choose Left{} or Right{}:", modifier, modifier));

        let mut selected: Option<MyKey> = None;

        ui.horizontal(|ui| {
            if ui.button(format!("{}Left", modifier)).clicked() {
                selected = if modifier == "Alt" {
                    MyKey::parse_key("Alt")
                } else {
                    MyKey::parse_key(&format!("{}Left", modifier))
                };
            }

            if ui.button(format!("{}Right", modifier)).clicked() {
                selected = if modifier == "Alt" {
                    MyKey::parse_key("AltGr")
                } else {
                    MyKey::parse_key(&format!("{}Right", modifier))
                };
            }
        });

        if let Some(key) = selected {
            *captured_modifier = None; // reset modifier once a key is selected
            return Some((Some(key), None)); // converted modifier into key
        } else {
            return Some((None, Some(modifier))); // modifier as string if no left/right was selected
        }
    }

    None
}