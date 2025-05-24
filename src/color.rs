use ansi_term::{Color, Style};
use crate::config::ConfigMommy;
use crate::utils::parse_string;

pub fn color_from_name(name: &str) -> Option<Color> {
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

pub fn color_from_rgb(rgb_str: &str) -> Option<Color> {
    let parts: Vec<&str> = rgb_str.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    Some(Color::RGB(r, g, b))
}

pub fn random_style_pick(config: &ConfigMommy) -> Style {
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