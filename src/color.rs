#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

//배경색은 ColorCode를 통해 표현됩니다.
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "White" | "white" | "0xf" => Color::White,
            "Blue" | "blue" | "0x1" => Color::Blue,
            "Green" | "green" | "0x2" => Color::Green,
            "Cyan" | "cyan" | "0x3" => Color::Cyan,
            "Red" | "red" | "0x4" => Color::Red,
            "Magenta" | "magenta" | "0x5" => Color::Magenta,
            "Brown" | "brown" | "0x6" => Color::Brown,
            "LightGray" | "lightgray" | "0x7" => Color::LightGray,
            "DarkGray" | "darkgray" | "0x8" => Color::DarkGray,
            "LightBlue" | "lightblue" | "0x9" => Color::LightBlue,
            "LightGreen" | "lightgreen" | "0xa" => Color::LightGreen,
            "LightCyan" | "lightcyan" | "0xb" => Color::LightCyan,
            "LightRed" | "lightred" | "0xc" => Color::LightRed,
            "LightMagenta" | "lightmagenta" | "0xd" => Color::LightMagenta,
            "Yellow" | "yellow" | "0xe" => Color::Yellow,
            "Black" | "black" | "0x0"  => Color::Black,
            _ => panic!()
        }
    }
}
