use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::{HashSet,HashMap};

use rdev::Key;
use crate::keys::MyKey;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "Skills")]
    pub skills: Vec<String>,

    #[serde(rename = "Consumables")]
    pub consumables: Vec<String>,
    
    #[serde(rename = "Reset_Key")]
    pub resetkey: String,

    #[serde(rename = "Awakening")]
    pub awakening: String,

    #[serde(rename = "Onion_Slot")]
    pub onion: String,

    #[serde(rename = "SelectTitle")]
    pub select: String,

    #[serde(rename = "FS")]
    pub fs: bool,

    #[serde(rename = "NP")]
    pub np: bool,

    #[serde(rename = "TSS")]
    pub tss: bool,

    #[serde(rename = "FS_Key")]
    pub fskey: String,

    #[serde(rename = "NP_Key")]
    pub npkey: String,

    #[serde(rename = "TSS_Key")]
    pub tsskey: String,

    #[serde(rename = "NonRelevantTitle_Key")]
    pub anotherkey: String,

    #[serde(rename = "Flow")]
    pub flow: bool,

    #[serde(rename = "Grove")]
    pub grove: bool,

    #[serde(rename = "Lithia_Awk")]
    pub lithia_awk: bool,

    #[serde(rename = "Horizontal")]
    pub horizontal: bool,

    #[serde(rename = "ImageSize")]
    pub imagesize: Vec<f32>,

    // Runtime HashSets for fast lookup
    #[serde(skip)]
    pub consumable_keys: HashSet<Key>,

    #[serde(skip)]
    pub skill_keys: HashSet<Key>,

    #[serde(skip)]
    pub awakening_keys: HashSet<Key>,

    #[serde(skip)]
    pub title_keys: HashMap<Key, String>,
}


impl Config {
    pub fn load() -> Self {
        let data = fs::read_to_string("config.json")
            .expect("Failed to read config.json");

        let mut cfg: Config = serde_json::from_str(&data)
            .expect("Failed to parse config.json");

        // Convert strings to Key
        cfg.consumable_keys = cfg.consumables.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        cfg.skill_keys = cfg.skills.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        let mut awakening_keys: Vec<String> = vec![cfg.awakening.clone()];

        if !cfg.onion.trim().is_empty() {
            awakening_keys.push(cfg.onion.clone());
        }

        cfg.awakening_keys = awakening_keys.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        let mut title_keys = HashMap::new();

        if let Some(k) = MyKey::parse_key(&cfg.fskey) {
            title_keys.insert(k.to_rdev(), "FS".to_string());
        }
        if let Some(k) = MyKey::parse_key(&cfg.tsskey) {
            title_keys.insert(k.to_rdev(), "TSS".to_string());
        }
        if let Some(k) = MyKey::parse_key(&cfg.npkey) {
            title_keys.insert(k.to_rdev(), "NP".to_string());
        }
        if let Some(k) = MyKey::parse_key(&cfg.anotherkey) {
            title_keys.insert(k.to_rdev(), "Another".to_string());
        }

        cfg.title_keys = title_keys;

        cfg
    }

    pub fn save(&self) {
        let to_set = serde_json::to_string_pretty(self).expect("Failed to serialize config");
        fs::write("config.json", to_set);
    }

    pub fn update(&mut self) {
        // Convert strings to Key
        self.consumable_keys = self.consumables.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        self.skill_keys = self.skills.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        // Awakening keys
        let mut awakening_keys: Vec<String> = vec![self.awakening.clone()];
        if !self.onion.trim().is_empty() {
            awakening_keys.push(format!("Num{}", self.onion));
        }

        self.awakening_keys = awakening_keys.iter()
            .filter_map(|s| MyKey::parse_key(s))
            .map(|k| k.to_rdev())
            .collect();

        let mut title_keys = HashMap::new();

        if let Some(k) = MyKey::parse_key(&self.fskey) {
            title_keys.insert(k.to_rdev(), "FS".to_string());
        }
        if let Some(k) = MyKey::parse_key(&self.tsskey) {
            title_keys.insert(k.to_rdev(), "TSS".to_string());
        }
        if let Some(k) = MyKey::parse_key(&self.npkey) {
            title_keys.insert(k.to_rdev(), "NP".to_string());
        }
        if let Some(k) = MyKey::parse_key(&self.anotherkey) {
            title_keys.insert(k.to_rdev(), "Another".to_string());
        }

        self.title_keys = title_keys;
    }
}