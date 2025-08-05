use std::env;
use std::process::{Command, exit};
use crate::config::load_config;
use crate::utils::{fill_template, graceful_print};
use crate::affirmations::{load_affirmations, load_custom_affirmations, Affirmations};
use crate::color::random_style_pick;

// https://en.wikipedia.org/wiki/Don't_repeat_yourself
fn choose_template<'a>( json_template: Option<&'a Vec<String>>, default_template: &'a Vec<String> ) -> &'a str {
    let templates = json_template.unwrap_or(default_template);
    let idx = fastrand::usize(..templates.len());
    templates[idx].as_str()
}

pub fn mommy() -> Result<i32, Box<dyn std::error::Error>> {
    let config = load_config();
    let affirmations: Option<Affirmations> = if let Some(ref path) = config.affirmations { load_custom_affirmations(path) } else { load_affirmations() };
    let affirmations_error: Vec<String> = vec![ "{roles} failed to load any affirmations, {little}~ {emotes}".to_string() ];

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
            .status()?;

        status.code().unwrap_or(1)
    };

    let (template, _affirmation_type) = match (exit_code == 0, config.only_negative) {
        (true, false) => ( choose_template(affirmations.as_ref().map(|aff| &aff.positive), &affirmations_error), "positive" ),
        (false, true) => ( choose_template(affirmations.as_ref().map(|aff| &aff.negative), &affirmations_error), "negative" ),
        (false, false) => ( choose_template(affirmations.as_ref().map(|aff| &aff.negative), &affirmations_error), "negative" ),
        _ => return Ok(exit_code),
    };

    let output = fill_template(template, &config);
    let styled_output = random_style_pick(&config).paint(output);
    graceful_print(styled_output);

    Ok(exit_code)
}
