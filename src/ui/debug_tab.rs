use crate::types::loader::*;
use eframe::egui;
use std::collections::HashSet;

pub fn show(ui: &mut egui::Ui, tracker: &mut Tracker) {
    ui.heading("Debug");

    ui.separator();

    // ---------------- Selected ----------------
    ui.label(format!("Selected Title: {}", tracker.selected));

    ui.separator();

    // ---------------- Cooldown States ----------------
    ui.collapsing("Cooldown State", |ui| {
        debug_cd(ui, "FS", &tracker.fs);
        debug_cd(ui, "TSS", &tracker.tss);
        debug_cd(ui, "NP", &tracker.np);
        debug_cd(ui, "Flow", &tracker.flow);
        debug_cd(ui, "Grove", &tracker.grove);
        debug_cd(ui, "Lithia", &tracker.lithia_awk);
    });

    ui.separator();

    // ---------------- Pressed Keys ----------------
    ui.collapsing("Pressed Keys", |ui| {
        ui.label("Dont stay locked on the window, it wont listen to the inputs.");
        if let Ok(pressed) = tracker.pressed_keys.lock() {
            if pressed.is_empty() {
                ui.label("None");
            } else {
                for key in pressed.iter() {
                    ui.label(format!("{:?}", key));
                }
            }
        } else {
            ui.label("Mutex poisoned.");
        }
    });

    ui.collapsing("Duplicate Key Check", |ui| {
        ui.label("Onion being a consumable slot does NOT count as duplicate.");
        ui.label("Checking for duplicate key assignments...");

        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();

        // Collect all keys
        let all_keys = tracker.config.skills.iter()
            .chain(tracker.config.consumables.iter())
            .chain(std::iter::once(&tracker.config.awakening))
            .chain(std::iter::once(&tracker.config.fskey))
            .chain(std::iter::once(&tracker.config.npkey))
            .chain(std::iter::once(&tracker.config.tsskey))
            .chain(std::iter::once(&tracker.config.anotherkey))
            .chain(std::iter::once(&tracker.config.select))
            .chain(std::iter::once(&tracker.config.resetkey));

        for key in all_keys {
            if !key.is_empty() {
                if !seen.insert(key.clone()) {
                    duplicates.push(key.clone());
                }
            }
        }

        if duplicates.is_empty() {
            ui.label("No duplicate key selections found.");
        } else {
            ui.colored_label(egui::Color32::RED, format!("Duplicates: {:?}", duplicates));
        }
    });
}

fn debug_cd(ui: &mut egui::Ui, name: &str, cd: &crate::types::cooldown::Cooldown) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.label(format!("Duration: {:.1}", cd.duration));
        ui.label(format!(
            "Active: {}",
            if cd.start_time.is_some() { "Yes" } else { "No" }
        ));
    });
}