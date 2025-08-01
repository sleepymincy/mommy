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
    pub affirmations: Option<String>,
    pub needy: bool,
    pub only_negative: bool,
}

pub fn load_config() -> ConfigMommy {
    let pronouns        = env::var("SHELL_MOMMYS_PRONOUNS").unwrap_or_else(|_| "her".to_string());
    let roles           = env::var("SHELL_MOMMYS_ROLES").unwrap_or_else(|_| "mommy".to_string());
    let little          = env::var("SHELL_MOMMYS_LITTLE").unwrap_or_else(|_| "girl".to_string());
    let emotes          = env::var("SHELL_MOMMYS_EMOTES").unwrap_or_else(|_| "💖/💗/💓/💞".to_string());
    let color           = env::var("SHELL_MOMMYS_COLOR").unwrap_or_else(|_| "white".to_string());
    let style           = env::var("SHELL_MOMMYS_STYLE").unwrap_or_else(|_| "bold".to_string());
    let color_rgb       = env::var("SHELL_MOMMYS_COLOR_RGB").ok();
    let aliases         = env::var("SHELL_MOMMYS_ALIASES").ok();
    let affirmations    = env::var("SHELL_MOMMYS_AFFIRMATIONS").ok();
    let needy           = env::var("SHELL_MOMMYS_NEEDY").map_or(false, |v| v == "1");
    let only_negative   = env::var("SHELL_MOMMY_ONLY_NEGATIVE").map_or(false, |v| v == "1");

    ConfigMommy {
        pronouns,
        roles,
        little,
        emotes,
        color,
        style,
        color_rgb,
        aliases,
        affirmations,
        needy,
        only_negative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use serial_test::serial;

    // Helper to clear all config‐related env vars.
    fn clear_all() {
        let keys = [
            "SHELL_MOMMYS_PRONOUNS",
            "SHELL_MOMMYS_ROLES",
            "SHELL_MOMMYS_LITTLE",
            "SHELL_MOMMYS_EMOTES",
            "SHELL_MOMMYS_COLOR",
            "SHELL_MOMMYS_STYLE",
            "SHELL_MOMMYS_COLOR_RGB",
            "SHELL_MOMMYS_ALIASES",
            "SHELL_MOMMYS_AFFIRMATIONS",
            "SHELL_MOMMYS_NEEDY",
            "SHELL_MOMMY_ONLY_NEGATIVE",
        ];
        for k in &keys { unsafe {
            env::remove_var(k);
        } }
    }

    #[test]
    #[serial]
    fn test_default_vars() {
        clear_all();
        let config = load_config();

        // Expect: all defaults
        assert_eq!(config.pronouns, "her");
        assert_eq!(config.roles, "mommy");
        assert_eq!(config.little, "girl");
        assert_eq!(config.emotes, "💖/💗/💓/💞");
        assert_eq!(config.color, "white");
        assert_eq!(config.style, "bold");
        assert_eq!(config.color_rgb, None);
        assert_eq!(config.aliases, None);
        assert_eq!(config.affirmations, None);
        assert!(!config.needy);
        assert!(!config.only_negative);
    }

    #[test]
    #[serial]
    fn test_custom_vars() {
        clear_all();
        unsafe {
            env::set_var("SHELL_MOMMYS_PRONOUNS", "his");
            env::set_var("SHELL_MOMMYS_ROLES", "daddy");
            env::set_var("SHELL_MOMMYS_COLOR_RGB", "255,255,255");
            env::set_var("SHELL_MOMMYS_NEEDY", "1");
            env::set_var("SHELL_MOMMY_ONLY_NEGATIVE", "1");
        }
        let config = load_config();

        // Expect: pronouns: his; role: daddy; color_rgb: 255,255,255; needy: 1; only_negative: 1
        assert_eq!(config.pronouns, "his");
        assert_eq!(config.roles, "daddy");
        assert_eq!(config.color_rgb, Some("255,255,255".to_string()));
        assert!(config.needy, "expected 1, got {:#?}", config.needy);
        assert!(config.only_negative, "expected 1, got {:#?}", config.only_negative);
    }
}