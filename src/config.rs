use std::env;

#[derive(Debug)]
pub struct ConfigMommy {
    pub pronouns: String,
    pub roles: String,
    pub little: String,
    pub emotes: String,
    pub color: String,
    pub style: String,
    pub color_rgb: Option<String>,
    pub aliases: Option<String>,
}

pub fn load_config() -> ConfigMommy {
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

    ConfigMommy {
        pronouns,
        roles,
        little,
        emotes,
        color,
        style,
        color_rgb,
        aliases,
    }
}