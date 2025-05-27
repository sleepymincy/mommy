use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Affirmations {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

// TODO: Implement moods functionality and other stuffs present in cargo mommy.
pub fn load_affirmations() -> Option<Affirmations> {
    let json_str = include_str!("../assets/affirmations.json");
    serde_json::from_str::<Affirmations>(json_str).ok()
}

pub fn load_custom_affirmations<P: AsRef<Path>>(path: P) -> Option<Affirmations> {
    let json_str = fs::read_to_string(&path).ok()?;
    serde_json::from_str::<Affirmations>(&json_str).ok()
}