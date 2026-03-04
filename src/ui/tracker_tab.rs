use crate::types::loader::*;
use eframe::egui;

pub struct TrackerUI {
    pub fs_on: egui::TextureHandle,
    pub fs_off: egui::TextureHandle,
    pub tss_on: egui::TextureHandle,
    pub tss_off: egui::TextureHandle,
    pub np_on: egui::TextureHandle,
    pub np_off: egui::TextureHandle,
    pub flow_on: egui::TextureHandle,
    pub flow_off: egui::TextureHandle,
    pub grove_on: egui::TextureHandle,
    pub grove_off: egui::TextureHandle,
    pub lithia_on: egui::TextureHandle,
    pub lithia_off: egui::TextureHandle,
    pub icon_size: egui::Vec2,
}

impl TrackerUI {
    pub fn new(ctx: &egui::Context, imagesize: &Vec<f32>) -> Self {
        use eframe::egui::{ColorImage, TextureOptions};
        use std::io::Cursor;
        use image::io::Reader as ImageReader;

        let icon_size = egui::vec2(imagesize[0], imagesize[1]);

        fn load_png(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
            let img = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8();

            let size = [img.width() as usize, img.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &img);

            ctx.load_texture(name, color_image, TextureOptions::LINEAR)
        }

        Self {
            fs_on: load_png(ctx, "fs_on", include_bytes!("../assets/icons/FS.png")),
            fs_off: load_png(ctx, "fs_off", include_bytes!("../assets/icons/FS_off.png")),
            tss_on: load_png(ctx, "tss_on", include_bytes!("../assets/icons/TSS.png")),
            tss_off: load_png(ctx, "tss_off", include_bytes!("../assets/icons/TSS_off.png")),
            np_on: load_png(ctx, "np_on", include_bytes!("../assets/icons/NP.png")),
            np_off: load_png(ctx, "np_off", include_bytes!("../assets/icons/NP_off.png")),
            flow_on: load_png(ctx, "flow_on", include_bytes!("../assets/icons/Flow.png")),
            flow_off: load_png(ctx, "flow_off", include_bytes!("../assets/icons/Flow_off.png")),
            grove_on: load_png(ctx, "grove_on", include_bytes!("../assets/icons/Grove.png")),
            grove_off: load_png(ctx, "grove_off", include_bytes!("../assets/icons/Grove_off.png")),
            lithia_on: load_png(ctx, "lithia_on", include_bytes!("../assets/icons/Lithia.png")),
            lithia_off: load_png(ctx, "lithia_off", include_bytes!("../assets/icons/Lithia_off.png")),
            icon_size,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui, tracker: &Tracker, now: f64) {
        let icon_size = egui::vec2(tracker.config.imagesize[0], tracker.config.imagesize[1]);
        ui.visuals_mut().override_text_color =
            Some(egui::Color32::from_rgb(200, 220, 255));

        ui.label(format!("Selected: {}", tracker.selected));

        let mut cds = vec![
                (tracker.config.fs, &tracker.fs, &self.fs_on, &self.fs_off, "FS"),
                (tracker.config.tss, &tracker.tss, &self.tss_on, &self.tss_off, "TSS"),
                (tracker.config.np, &tracker.np, &self.np_on, &self.np_off, "NP"),
                (tracker.config.flow, &tracker.flow, &self.flow_on, &self.flow_off, "Flow"),
                (tracker.config.grove, &tracker.grove, &self.grove_on, &self.grove_off, "Grove"),
                (tracker.config.lithia_awk, &tracker.lithia_awk, &self.lithia_on, &self.lithia_off, "Lithia")
        ];

        if tracker.config.horizontal {
            ui.horizontal(|ui| {
                for (toggled, cd, icon_on, icon_off, name) in cds {
                    if toggled == true {
                        ui.vertical(|ui| {
                            let icon =
                                if cd.start_time.is_none() { icon_on } else { icon_off };

                            ui.add(
                                egui::Image::new(icon)
                                    .fit_to_exact_size(icon_size),
                            );

                            if let Some(t) = cd.start_time {
                                ui.label(format!(
                                    "{}: {:.1}",
                                    name,
                                    (t + cd.duration - now).max(0.0)
                                ));
                            } else {
                                ui.label(format!("{}: Ready", name));
                            }
                        });
                    }
                }
            });
        }
        else {
            for (toggled, cd, icon_on, icon_off, name) in cds {
                if toggled == true {
                    ui.horizontal(|ui| {
                        let icon =
                            if cd.start_time.is_none() { icon_on } else { icon_off };

                        ui.add(
                            egui::Image::new(icon)
                                .fit_to_exact_size(icon_size),
                        );

                        if let Some(t) = cd.start_time {
                            ui.label(format!(
                                "{}: {:.1}",
                                name,
                                (t + cd.duration - now).max(0.0)
                            ));
                        } else {
                            ui.label(format!("{}: Ready", name));
                        }
                    });
                }
            }
        }
    }
}