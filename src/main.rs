mod types;
use types::loader::*;

mod keys;
use keys::MyKey;

mod helpers;

mod ui;

use eframe::egui;
use rdev::{EventType, Key};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::fs;
use strum_macros::EnumString;

impl eframe::App for Tracker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let now = ctx.input(|i| i.time);

        // Update all cooldowns
        self.select.update(now);
        self.fs.update(now);
        self.np.update(now);
        self.tss.update(now);
        self.flow.update(now);
        self.grove.update(now);
        self.lithia_awk.update(now);

        // -------------------- Process Events --------------------
        let mut pressed = self.pressed_keys.lock().unwrap();

        while let Ok(event) = self.rx.try_recv() {
            match event {
                EventType::KeyPress(key) => {

                    // Insert the pressed key to the list of pressed keys
                    pressed.insert(key);

                    // Reset cooldowns
                    if let Some(config_key) = MyKey::parse_key(&self.config.resetkey) {
                        if key == MyKey::to_rdev(config_key) {
                            self.fs.reset();
                            self.tss.reset();
                            self.np.reset();
                            self.flow.reset();
                            self.grove.reset();
                            self.lithia_awk.reset();
                        }
                    }

                    // If the key is the SELECT TITLE keybind, then start the Cooldown for title selection.
                    if let Some(config_key) = MyKey::parse_key(&self.config.select) {
                        if key == MyKey::to_rdev(config_key) {
                            self.select.start(now);
                        }
                    }

                    // If the key pressed WHILE the title select timer is up AND the key is present in the list of title keys, then set it as selected title.
                    if self.select.start_time.is_some() {
                        if let Some(selection) = self.config.title_keys.get(&key) {
                            self.selected = selection.clone();
                            self.select.reset(); // mark selection done
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    pressed.remove(&key);
                }
                _ => {}
            }
        }

        // -------------------- Activation Logic --------------------


        //is_disjoint: means if the hashset given (e.g consumable_keys) has nothing in common with
        //the given argument (e.g no pressed key from the pressed key list is included in the list of consumable keys)

        // Freed Shadow | Concerto
        if self.config.fs && self.selected == "FS" && !self.config.awakening_keys.is_disjoint(&pressed) {
            self.fs.start(now);
        }

        // The Setting Sun
        if self.config.tss && self.selected == "TSS" && !self.config.awakening_keys.is_disjoint(&pressed) {
            self.tss.start(now);
        }

        // Night Parade
        if self.config.np && self.selected == "NP" && !self.config.skill_keys.is_disjoint(&pressed) {
            self.np.start(now);
        }

        // Lithia gemstone thing?
        if self.config.lithia_awk == true && !self.config.awakening_keys.is_disjoint(&pressed) {
            self.lithia_awk.start(now);
        }

        // Grove
        if self.config.grove == true && !self.config.awakening_keys.is_disjoint(&pressed) {
            self.grove.start(now);
        }

        // Flow (consumables)
        if self.config.flow == true && !self.config.consumable_keys.is_disjoint(&pressed) { 
            self.flow.start(now);
        }

        drop(pressed);
         
        crate::ui::show_ui(self, ctx, now);

        ctx.request_repaint_after(Duration::from_millis(50));
    }
}

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_always_on_top()
        .with_transparent(true);
    
    eframe::run_native(
        "Title Tracker",
        options.clone(),
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let (tx, rx) = std::sync::mpsc::channel();

            Tracker::spawn_listener(tx);

            let config = Config::load();

            Ok(Box::new(Tracker::new(cc, rx, config)))
        }),
    )
}