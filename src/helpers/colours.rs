use colors_transform::{Color, Rgb};
use hex_color::HexColor;
pub static TRANSPARENT: &str = "#00000000";

/// Every function in colours.rs returns a String, which is a HEX.
/// This means that every function can be chained into each other

fn parse(hex_colour: String) -> HexColor {
    HexColor::parse(&hex_colour).expect(format!("Invalid hex colour: {}", hex_colour).as_str())
}

pub fn luma(hex_colour: String) -> f32 {
    let rgb = parse(hex_colour);

    // SMPTE C, Rec. 709 weightings
    (0.2126 * rgb.r as f32) + (0.7152 * rgb.g as f32) + (0.0722 * rgb.b as f32)
}

pub fn bw_on_bg(background_hex: String) -> String {
    if luma(background_hex.clone()) > 165.0 {
        "#000000".to_string()
    } else {
        "#ffffff".to_string()
    }
}

pub fn opacity(hex_color: String, opacity: f32) -> String {
    let opacity_hex_str = format!("{:02x}", (opacity * 255.0) as u8);
    if hex_color.len() == 7 {
        format!("{}{}", &hex_color, opacity_hex_str)
    } else {
        format!("{}{}", &hex_color[..7], opacity_hex_str)
    }
}

pub fn darken(hex_colour: String, amount: f32) -> String {
    let rgb = parse(hex_colour);
    let hsl = Rgb::from(rgb.r as f32, rgb.g as f32, rgb.b as f32).to_hsl();
    hsl.lighten(-(amount * 50.0)).to_rgb().to_css_hex_string()
}
