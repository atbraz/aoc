/// Represents different colors and text styles for terminal output
#[derive(Debug, Clone, Copy)]
pub enum Color {
    // Regular colors (foreground)
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Bright colors (foreground)
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // Background colors
    OnBlack,
    OnRed,
    OnGreen,
    OnYellow,
    OnBlue,
    OnMagenta,
    OnCyan,
    OnWhite,
    OnBrightBlack,
    OnBrightRed,
    OnBrightGreen,
    OnBrightYellow,
    OnBrightBlue,
    OnBrightMagenta,
    OnBrightCyan,
    OnBrightWhite,
    // Styles
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    None,
}

impl Color {
    /// Returns the ANSI escape code for this color/style
    #[must_use]
    pub fn wrap_code(&self) -> Option<&'static str> {
        let code = match self {
            // Regular colors (foreground)
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            // Bright colors (foreground)
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
            // Background colors
            Color::OnBlack => "40",
            Color::OnRed => "41",
            Color::OnGreen => "42",
            Color::OnYellow => "43",
            Color::OnBlue => "44",
            Color::OnMagenta => "45",
            Color::OnCyan => "46",
            Color::OnWhite => "47",
            Color::OnBrightBlack => "100",
            Color::OnBrightRed => "101",
            Color::OnBrightGreen => "102",
            Color::OnBrightYellow => "103",
            Color::OnBrightBlue => "104",
            Color::OnBrightMagenta => "105",
            Color::OnBrightCyan => "106",
            Color::OnBrightWhite => "107",
            // Styles
            Color::Bold => "1",
            Color::Dim => "2",
            Color::Italic => "3",
            Color::Underline => "4",
            Color::Blink => "5",
            Color::Reverse => "7",
            Color::None => return None,
        };
        Some(code)
    }

    /// Wraps text with the ANSI escape codes for this color/style
    #[must_use]
    pub fn wrap(&self, text: &str) -> String {
        if let Some(code) = self.wrap_code() {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }

    /// Combines multiple colors/styles and wraps text with the combined ANSI escape codes
    pub fn combine(colors: &[Color], text: &str) -> String {
        let codes: Vec<&str> = colors.iter().filter_map(Color::wrap_code).collect();

        if codes.is_empty() {
            return text.to_string();
        }

        format!("\x1b[{}m{text}\x1b[0m", codes.join(";"))
    }
}
