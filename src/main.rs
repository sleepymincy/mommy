use std::env;
use ansi_term::Style;
use std::process::{Command, exit};
use serde::Deserialize;

#[derive(Debug)]
struct ConfigMommy {
    pronouns: String,
    roles: String,
    little: String,
    emotes: String,
}

#[derive(Deserialize)]
struct Affirmations {
    positive: Vec<String>,
    negative: Vec<String>,
}

fn load_config() -> ConfigMommy {
    let pronouns = env::var("SHELL_MOMMYS_PRONOUNS").unwrap_or_else(|_err| String::from("her"));
    let roles    = env::var("SHELL_MOMMYS_ROLES").unwrap_or_else(|_err| String::from("mommy"));
    let little   = env::var("SHELL_MOMMYS_LITTLE").unwrap_or_else(|_err| String::from("girl"));
    let emotes   = env::var("SHELL_MOMMYS_EMOTES").unwrap_or_else(|_err| String::from("💖/💗/💓/💞"));

    ConfigMommy { pronouns, roles, little, emotes }
}

fn load_affirmations() -> Option<Affirmations> {
    let json_str = include_str!("affirmations.json");
    serde_json::from_str::<Affirmations>(json_str).ok()
}

fn random_part(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split('/')
                                .filter(|s| !s.trim().is_empty())
                                .collect();

    if parts.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..parts.len());
        Some(parts[idx].to_string())
    }
}

fn fill_template(template: &str, config: &ConfigMommy) -> String {
    let role = random_part(&config.roles).unwrap_or_else(|| config.roles.clone());
    let pronoun = random_part(&config.pronouns).unwrap_or_else(|| config.pronouns.clone());
    let little = random_part(&config.little).unwrap_or_else(|| config.little.clone());
    let emote = random_part(&config.emotes).unwrap_or_else(|| config.emotes.clone());
    
    template.replace("{roles}", &role)
        .replace("{pronouns}", &pronoun)
        .replace("{little}", &little)
        .replace("{emotes}", &emote)
}

fn mommy() {
    let config = load_config();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args ...]", args[0]);
        exit(1);
    }

    let command = &args[1];
    let command_args = &args[2..];

    let status = Command::new(command)
        .args(command_args)
        .spawn()
        .and_then(|mut child| child.wait());

    let affirmations = load_affirmations();

    let default_positive: Vec<String> = vec![
        "{roles} loves {pronouns} {little}~ {emotes}".to_string()
    ];

    let _default_negative: Vec<String> = vec![
        "{roles} truly believes you can do it, {little}~ {emotes}".to_string()
    ];

    let (template, _affirmation_type) = match status {
        Ok(exit_status) if exit_status.success() || exit_status.code() == Some(130) => {
            let templates = if let Some(ref aff) = affirmations {
                &aff.positive
            } else {
                &default_positive
            };
            let idx = fastrand::usize(..templates.len());
            (templates[idx].as_str(), "positive")
        },
        _ => {
            let templates = if let Some(ref aff) = affirmations {
                &aff.negative
            } else {
                &_default_negative
            };
            let idx = fastrand::usize(..templates.len());
            (templates[idx].as_str(), "negative")
        },
    };

    let output = fill_template(template, &config);
    println!("{}", Style::new().bold().paint(output));
}

fn main() {
    mommy();
}