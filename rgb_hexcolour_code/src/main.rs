use std::fmt::Display;
use std::str::FromStr;

struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

#[derive(Debug)]
enum ErrHandler {
    Something,
    SomethingElse,
}

impl FromStr for Rgb {
    type Err = ErrHandler;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('#') {
            return Err(ErrHandler::Something);
        }

        let red = u8::from_str_radix(&s[1..=2], 16).unwrap();
        let green = u8::from_str_radix(&s[3..=4], 16).unwrap();
        let blue = u8::from_str_radix(&s[5..=6], 16).unwrap();

        println!("red: {}, green: {}, blue: {}", red, green, blue);

        Ok(Self {
            red: red,
            green: green,
            blue: blue,
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    let s = "#f3ad35";
    let red = u32::from_str_radix(&s[1..=2], 16).unwrap();
    let green = u32::from_str_radix(&s[3..=4], 16).unwrap();
    let blue = u32::from_str_radix(&s[5..=6], 16).unwrap();

    println!("red: {}, green: {}, blue: {}", red, green, blue);
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
