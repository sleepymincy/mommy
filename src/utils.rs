use std::io::{self, Write};
use crate::config::ConfigMommy;

pub fn parse_string(s: &str) -> Vec<String> {
    s.split('/').map(|token| token.trim().to_lowercase()).filter(|token| !token.is_empty()).collect()
}

pub fn random_string_pick(input: &str) -> Option<String> {
    let parts = parse_string(input);

    if parts.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..parts.len());
        Some(parts[idx].to_string())
    }
}

pub fn fill_template(template: &str, config: &ConfigMommy) -> String {
    let role = random_string_pick(&config.roles).unwrap_or_else(|| config.roles.clone());
    let pronoun = random_string_pick(&config.pronouns).unwrap_or_else(|| config.pronouns.clone());
    let little = random_string_pick(&config.little).unwrap_or_else(|| config.little.clone());
    let emote = random_string_pick(&config.emotes).unwrap_or_else(|| config.emotes.clone());

    template
        .replace("{roles}", &role)
        .replace("{pronouns}", &pronoun)
        .replace("{little}", &little)
        .replace("{emotes}", &emote)
}

pub fn graceful_print<T: std::fmt::Display>(s: T) {
    if writeln!(io::stderr(), "{}", s).is_err() {
        std::process::exit(0);
    }
}