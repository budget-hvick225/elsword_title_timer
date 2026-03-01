use crate::types::loader::*;
use eframe::egui;

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