use std::env;
use std::error::Error;
use std::process::{Command, exit};
use crate::config::load_config;
use crate::affirmations::load_affirmations;
use crate::utils::{fill_template, graceful_print};
use crate::color::random_style_pick;

pub fn mommy() -> Result<(), Box<dyn Error>> {
    let config = load_config();
    let affirmations = load_affirmations();
    let default_positive: Vec<String> = vec![ "{roles} loves {pronouns} {little}~ {emotes}".to_string() ];
    let default_negative: Vec<String> = vec![ "{roles} truly believes you can do it, {little}~ {emotes}".to_string() ];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("Usage: {} {}", args[0], if config.needy { "<exit_code>" } else { "<command> [args ...]" }); exit(1); }

    let exit_code: i32 = if config.needy {
        args[1].parse()?
    } else {
        let raw_command = args[1..].join(" ");
        let run_command = if let Some(ref aliases_path) = config.aliases {
            format!("shopt -s expand_aliases; source \"{}\"; eval {}", aliases_path, raw_command)
        } else {
            raw_command
        };

        let status = Command::new("bash")
            .arg("-c")
            .arg(&run_command)
            .spawn()
            .and_then(|mut child| child.wait())?;

        status.code().unwrap_or(1)
    };

    let (template, _affirmation_type) = if exit_code == 0 || exit_code == 130 {
        let templates = if let Some(ref aff) = affirmations { &aff.positive } else { &default_positive };
        let idx = fastrand::usize(..templates.len());
        (templates[idx].as_str(), "positive")
    } else {
        let templates = if let Some(ref aff) = affirmations { &aff.negative } else { &default_negative };
        let idx = fastrand::usize(..templates.len());
        (templates[idx].as_str(), "negative")
    };

    let output = fill_template(template, &config);
    let styled_output = random_style_pick(&config).paint(output);
    graceful_print(styled_output);

    Ok(())
}
