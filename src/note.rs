use chrono::prelude::*;
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: u32,
    pub content: String,
    pub date: String,
    pub time: String,
    pub tag: String,
}
impl Note {
    pub fn new(content: String, tag: String) -> Self {
        let date = Local::now().format("%Y-%m-%d");
        let time = Local::now().format("%H:%M");
        let id = rand::thread_rng().gen_range(1000..9999);
        Note {
            id: id,
            content: content,
            date: date.to_string(),
            time: time.to_string(),
            tag: tag,
        }
    }

    pub fn load_from_json() -> Option<Vec<Note>> {
        let data = fs::read_to_string("test.json").ok()?;
        let notes: Vec<Note> = serde_json::from_str(&data).ok()?;
        Some(notes)
    }
    pub fn save_to_json(notes: &Vec<Self>) -> bool   {
        let json = match serde_json::to_string_pretty(notes) {
            Ok(j) => j,
            Err(_) => return false,
        };

        fs::write("test.json", json).is_ok()
    }
}
