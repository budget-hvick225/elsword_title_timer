use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use eframe::CreationContext;
use crate::types::cooldown::Cooldown;
use crate::ui::tracker_tab::TrackerUI;
use crate::types::config::Config;
use crate::types::tab::Tab;
pub struct Tracker {
    pub select: Cooldown,
    pub fs: Cooldown,
    pub np: Cooldown,
    pub tss: Cooldown,
    pub flow: Cooldown,
    pub grove: Cooldown,
    pub lithia_awk: Cooldown,
    pub selected: String,

    pub pressed_keys: Arc<Mutex<HashSet<Key>>>,
    pub rx: mpsc::Receiver<EventType>,

    pub active_tab: Tab,
    pub ui: TrackerUI,
    pub config: Config,
}

impl Tracker {
    pub fn new(
        cc: &CreationContext<'_>,
        rx: std::sync::mpsc::Receiver<rdev::EventType>,
        config: Config,
    ) -> Self {
        Self {
            select: Cooldown::new(3.0),
            fs: Cooldown::new(60.0),
            np: Cooldown::new(25.0),
            tss: Cooldown::new(30.0),
            flow: Cooldown::new(15.0),
            grove: Cooldown::new(10.0),
            lithia_awk: Cooldown::new(15.0),
            selected: String::new(),
            pressed_keys: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashSet::new())),
            rx,
            ui: TrackerUI::new(&cc.egui_ctx, &config.imagesize),
            active_tab: Tab::Tracker,

            config,
        }
    }

    // -------------------- Global Listener --------------------

    pub fn spawn_listener(tx: mpsc::Sender<EventType>) {
        thread::spawn(move || {
            let callback = move |event: Event| {
                tx.send(event.event_type).ok();
            };

            if let Err(e) = listen(callback) {
                println!("Error: {:?}", e);
            }
        });
    }
}