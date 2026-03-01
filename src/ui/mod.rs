pub mod tracker_tab;
pub mod settings_tab;
pub mod debug_tab;

use crate::types::loader::*;
use eframe::egui;

pub fn show_ui(tracker: &mut Tracker, ctx: &egui::Context, now: f64) {
    let frame = if tracker.active_tab == Tab::Tracker {
        egui::Frame::NONE
    } else {
        egui::Frame::default()
            .fill(egui::Color32::from_gray(30)) // dark gray background
    };

    egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| {

            let mut visuals = ctx.style().visuals.clone();

            // ---------------- Backgrounds ----------------
            // The "fill" of the button / checkbox / slider
            visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(100);  // normal background
            visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(120);   // when mouse over
            visuals.widgets.active.bg_fill = egui::Color32::from_gray(140);    // pressed

            // ---------------- Text ----------------
            // This is the **button / label text**
            // Black normally, white on hover, yellow on active (pressed)
            visuals.widgets.inactive.fg_stroke.color = egui::Color32::WHITE;      // default text
            visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE;       // hover text
            visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE;       // pressed text

            visuals.override_text_color = Some(egui::Color32::WHITE);

            // Apply the visuals
            ctx.set_visuals(visuals);


            // ---------------- Tab Row ----------------
            ui.horizontal(|ui| {
                ui.selectable_value(&mut tracker.active_tab, Tab::Tracker, "Tracker");
                ui.selectable_value(&mut tracker.active_tab, Tab::Settings, "Settings");
                ui.selectable_value(&mut tracker.active_tab, Tab::Debug, "Debug");
            });

            ui.separator();

            // ---------------- Tab Content ----------------
            match tracker.active_tab {
                Tab::Tracker => tracker.ui.show(ui, tracker, now),
                Tab::Settings => settings_tab::show(ui, tracker),
                Tab::Debug => debug_tab::show(ui, tracker),
            }
        });
}