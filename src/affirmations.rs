use std::fs;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_embedded_affirmations_some() {
        let affirmations = load_affirmations();
        
        // Expect: loaded affirmations to be Some(_)
        assert!(affirmations.is_some(), "expected embedded affirmations to be Some(_)");
    }

    #[test]
    fn test_embedded_affirmations_content() {
        let affirmations = load_affirmations().expect("embedded JSON didn't parse");

        // Expect: at least one positive and one negative
        assert!(!affirmations.positive.is_empty(), "expected at least one positive affirmation");
        assert!(!affirmations.negative.is_empty(), "expected at least one negative affirmation");

        // Expect: one specific affirmation from the ../assets/affirmations.json
        let positives: HashSet<_> = affirmations.positive.iter().collect();
        assert!(positives.contains(&"*boops your nose* {emotes}".to_string()));
    }

    #[test]
    fn load_custom_affirmations_ok() {
        let aff = load_affirmations().unwrap();

        // Expect: one valid positive and negative affirmations
        assert!(aff.positive.contains(&"you're such a smart cookie~ {emotes}".to_string()));
        assert!(aff.negative.contains(&"{roles} believes in you~ {emotes}".to_string()));
    }

    #[test]
    fn load_custom_affirmations_missing_file() {
        let path = "/nonexistent/path/to/file";
        let aff = load_custom_affirmations(&path);

        // Expect: None for nonexistent path
        assert!(aff.is_none(), "expected None for bad path, got {:#?}", aff);
    }
}
