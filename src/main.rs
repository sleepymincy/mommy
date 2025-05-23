use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};
use std::error::Error;
use ansi_term::{Color, Style};
use serde::Deserialize;

#[derive(Debug)]
struct ConfigMommy {
    pronouns: String,
    roles: String,
    little: String,
    emotes: String,
    color: String,
    style: String,
    color_rgb: Option<String>,
    aliases: Option<String>,
}

#[derive(Deserialize)]
struct Affirmations {
    positive: Vec<String>,
    negative: Vec<String>,
}

fn load_config() -> ConfigMommy {
    let pronouns    = env::var("SHELL_MOMMYS_PRONOUNS").unwrap_or_else(|_| "her".to_string());
    let roles       = env::var("SHELL_MOMMYS_ROLES").unwrap_or_else(|_| "mommy".to_string());
    let little      = env::var("SHELL_MOMMYS_LITTLE").unwrap_or_else(|_| "girl".to_string());
    let emotes      = env::var("SHELL_MOMMYS_EMOTES").unwrap_or_else(|_| "💖/💗/💓/💞".to_string());
    let color       = env::var("SHELL_MOMMYS_COLOR").unwrap_or_else(|_| "white".to_string());
    let style       = env::var("SHELL_MOMMYS_STYLE").unwrap_or_else(|_| "bold".to_string());
    let color_rgb   = env::var("SHELL_MOMMYS_COLOR_RGB").ok();
    let aliases     = env::var("SHELL_MOMMYS_ALIASES").ok();
    // TODO: Add blocklist functionality for those who want to run mommy at all times.
    //       Mainly because commands like cd are not executable with it, and others, 
    //       like clear and exit, might not be desired to be executed with mommy.

    ConfigMommy { pronouns, roles, little, emotes, color, style, color_rgb, aliases }
}

fn load_affirmations() -> Option<Affirmations> {
    let json_str = include_str!("affirmations.json");
    serde_json::from_str::<Affirmations>(json_str).ok()
}

fn parse_string(s: &str) -> Vec<String> {
    s.split('/').map(|token| token.trim().to_lowercase()).filter(|token| !token.is_empty()).collect()
}

fn color_from_name(name: &str) -> Option<Color> {
    match name {
        "black"  => Some(Color::Black),
        "red"    => Some(Color::Red),
        "green"  => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue"   => Some(Color::Blue),
        "purple" | "magenta" => Some(Color::Purple),
        "cyan"   => Some(Color::Cyan),
        "white"  => Some(Color::White),
        _ => None,
    }
}

fn color_from_rgb(rgb_str: &str) -> Option<Color> {
    let parts: Vec<&str> = rgb_str.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Color::RGB(r, g, b))
}

fn random_string_pick(input: &str) -> Option<String> {
    let parts = parse_string(input);

    if parts.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..parts.len());
        Some(parts[idx].to_string())
    }
}

fn random_style_pick(config: &ConfigMommy) -> Style {
    let mut style = Style::new();

    if let Some(ref rgb_env) = config.color_rgb {
        let candidates = parse_string(rgb_env);
        if !candidates.is_empty() {
            let idx = fastrand::usize(..candidates.len());
            if let Some(col) = color_from_rgb(&candidates[idx]) {
                style = style.fg(col);
            }
        }
    } else {
        let candidates = parse_string(&config.color);
        if !candidates.is_empty() {
            let idx = fastrand::usize(..candidates.len());
            if let Some(col) = color_from_name(&candidates[idx]) {
                style = style.fg(col);
            }
        }
    }
    
    let style_combos: Vec<&str> = config.style.split('/')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    if !style_combos.is_empty() {
        let chosen_combo = style_combos[fastrand::usize(..style_combos.len())];
        let styles_in_combo: Vec<String> = chosen_combo.split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();
        
        for candidate in styles_in_combo {
            match &candidate[..] {
                "bold"      => style = style.bold(),
                "italic"    => style = style.italic(),
                "dimmed"    => style = style.dimmed(),
                "underline" => style = style.underline(),
                "blink"     => style = style.blink(),
                "reverse"   => style = style.reverse(),
                "hidden"    => style = style.hidden(),
                _           => {}
            }
        }
    }
    
    style
}

fn fill_template(template: &str, config: &ConfigMommy) -> String {
    let role = random_string_pick(&config.roles).unwrap_or_else(|| config.roles.clone());
    let pronoun = random_string_pick(&config.pronouns).unwrap_or_else(|| config.pronouns.clone());
    let little = random_string_pick(&config.little).unwrap_or_else(|| config.little.clone());
    let emote = random_string_pick(&config.emotes).unwrap_or_else(|| config.emotes.clone());
    
    template.replace("{roles}", &role)
            .replace("{pronouns}", &pronoun)
            .replace("{little}", &little)
            .replace("{emotes}", &emote)
}

fn graceful_print<T: std::fmt::Display>(s: T) {
    if writeln!(io::stderr(), "{}", s).is_err() {
        std::process::exit(0);
    }
}

fn mommy() -> Result<(), Box<dyn Error>> {
    let config = load_config();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args ...]", args[0]);
        exit(1);
    }

    let raw_command = args[1..].join(" ");
    let run_command = if let Some(ref aliases_path) = config.aliases {
        format!("shopt -s expand_aliases; source \"{}\"; eval {}", aliases_path, raw_command) // That is a lil disgusting, however I have no clue how to make it better at the moment. QwQ
    } else {
        raw_command
    };

    let status = Command::new("bash")
        .arg("-c")
        .arg(&run_command)
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
    let styled_output = random_style_pick(&config).paint(output);
    graceful_print(styled_output);
    Ok(())
}

fn main() {
    let exit_code = mommy().map(|_| 0).unwrap_or_else(|e| {
        eprintln!("Error: {e:?}");
        1
    });
    std::process::exit(exit_code)
}