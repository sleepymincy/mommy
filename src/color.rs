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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;
    
    #[test]
    fn test_color_names() {
        // Make sure all colors are correctly evaluated:
        assert_eq!(color_from_name("black"), Some(Color::Black));
        assert_eq!(color_from_name("red"), Some(Color::Red));
        assert_eq!(color_from_name("green"), Some(Color::Green));
        assert_eq!(color_from_name("yellow"), Some(Color::Yellow));
        assert_eq!(color_from_name("blue"), Some(Color::Blue));
        assert_eq!(color_from_name("purple"), Some(Color::Purple));
        assert_eq!(color_from_name("magenta"), Some(Color::Purple));
        assert_eq!(color_from_name("cyan"), Some(Color::Cyan));
        assert_eq!(color_from_name("white"), Some(Color::White));
    }
    
    #[test]
    fn test_invalid_color() {
        // Not valid color name:
        assert_eq!(color_from_name("not a color"), None);
        assert_eq!(color_from_name(""), None);
    }
    
    #[test]
    fn test_rgb_color_ok() {
        // Well‐formatted:
        let c = color_from_rgb("10,20,30");
        assert_eq!(c, Some(Color::RGB(10, 20, 30)));
        
        // With random spaces:
        let c2 = color_from_rgb("  0 ,255, 128  ");
        assert_eq!(c2, Some(Color::RGB(0, 255, 128)));
    }
    
    #[test]
    fn test_rgb_color_err() {
        // Wrong amount:
        assert_eq!(color_from_rgb("10,20"), None);
        assert_eq!(color_from_rgb("1,2,3,4"), None);
        
        // Non‐numeric:
        assert_eq!(color_from_rgb("a,b,c"), None);
        
        // Out of range:
        assert_eq!(color_from_rgb("256,0,0"), None);
    }
    
    #[test]
    fn test_color_style() {
        // Not RGB and bold:
        let mut config = load_config();
        config.color = "red".to_string();
        config.style = "bold".to_string();
        config.color_rgb = None;

        let styled = random_style_pick(&config);
        let output = styled.paint("Test").to_string();
        
        // 1 = bold, 31 = red
        // Expect: \x1b[1;31mTest\x1b[0m
        assert!(output.starts_with("\x1b[1;31m"), "expected string start: '\x1b[1;31m'");
        assert!(output.ends_with("Test\x1b[0m"), "expected string end: 'Test\x1b[0m'");
    }
    
    #[test]
    fn test_rgb_with_two_styles() {
        // RGB and two styles:
        let mut config = load_config();
        config.style = "underline, italic".to_string();
        config.color_rgb = Some("128,0,255".to_string());
        
        let styled = random_style_pick(&config);
        let output = styled.paint("Test").to_string();
        
        // italic = 3, underline = 4, RGB= 38;2;R;G;B
        // Expect: \x1b[3;4;38;2;128;0;255m
        assert!(output.starts_with("\x1b[3;4;38;2;128;0;255m"), "expected string start: '\x1b[3;4;38;2;128;0;255m'");
        assert!(output.ends_with("Test\x1b[0m"), "expected string end: 'Test\x1b[0m'");
    }
}