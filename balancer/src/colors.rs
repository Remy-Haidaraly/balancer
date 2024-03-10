const RESET_COLOR: &str = "\x1b[0m";
const GREEN_COLOR: &str = "\x1b[32m";
const RED_COLOR: &str = "\x1b[31m";

pub enum TextColor {
    Green,
    Red,
}
/// Function to colorize text with specified color
pub fn colorize(text: &str, color: TextColor) -> String {
    match color {
        TextColor::Green => format!("{}{}{}", GREEN_COLOR, text, RESET_COLOR),
        TextColor::Red => format!("{}{}{}", RED_COLOR, text, RESET_COLOR),
    }
}
