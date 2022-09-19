use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::interpreter::dna::Base;

pub type Pixel = image::Rgba<u8>;

pub fn load_from_file<P>(path: P) -> Result<image::RgbaImage, String> where P: AsRef<Path> {
    let image = image::io::Reader::open(path)
                    .map_err(|_err| "No source image".to_string())?
                    .decode().map_err(|_err| "Failed to decode source image".to_string())?;
    Ok(image.to_rgba8())
}

pub fn load_source() -> Result<image::RgbaImage, String> {
    load_from_file("data/source.png")
}

pub fn load_target() -> Result<image::RgbaImage, String> {
    load_from_file("data/target.png")
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Rgb {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Rgb {
    pub fn encode(&self) -> [u8; 3] {
        match self {
            Rgb::Black => [0, 0, 0],
            Rgb::Red => [255, 0, 0],
            Rgb::Green => [0, 255, 0],
            Rgb::Yellow => [255, 255, 0],
            Rgb::Blue => [0, 0, 255],
            Rgb::Magenta => [255, 0, 255],
            Rgb::Cyan => [0, 255, 255],
            Rgb::White => [255, 255, 255],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Alpha {
    Transparent,
    Opaque,
}

impl Alpha {
    pub fn encode(&self) -> u8 {
        match self {
            Alpha::Transparent => 0,
            Alpha::Opaque => 255,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Rgb(Rgb),
    Alpha(Alpha)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum DrawCommand {
    AddColor(Color),
    ClearBucket,
    Move,
    TurnCC,
    TurnC,
    Mark,
    Line,
    TryFill,
    AddBitmap,
    Compose,
    Clip
}

impl DrawCommand {
    pub fn decode(d: &[Base]) -> Option<Self> {
        use crate::interpreter::dna::Base::*;
        match d {
            [P, I, P, I, I, I, C] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Black))),
            [P, I, P, I, I, I, P] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Red))),
            [P, I, P, I, I, C, C] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Green))),
            [P, I, P, I, I, C, F] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Yellow))),
            [P, I, P, I, I, C, P] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Blue))),
            [P, I, P, I, I, F, C] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Magenta))),
            [P, I, P, I, I, F, F] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::Cyan))),
            [P, I, P, I, I, P, C] => Some(DrawCommand::AddColor(Color::Rgb(Rgb::White))),
            [P, I, P, I, I, P, F] => Some(DrawCommand::AddColor(Color::Alpha(Alpha::Transparent))),
            [P, I, P, I, I, P, P] => Some(DrawCommand::AddColor(Color::Alpha(Alpha::Opaque))),

            [P, I, I, P, I, C, P] => Some(DrawCommand::ClearBucket),

            [P, I, I, I, I, I, P] => Some(DrawCommand::Move),
            [P, C, C, C, C, C, P] => Some(DrawCommand::TurnCC),
            [P, F, F, F, F, F, P] => Some(DrawCommand::TurnC),

            [P, C, C, I, F, F, P] => Some(DrawCommand::Mark),
            [P, F, F, I, C, C, P] => Some(DrawCommand::Line),

            [P, I, I, P, I, I, P] => Some(DrawCommand::TryFill),
            [P, C, C, P, F, F, P] => Some(DrawCommand::AddBitmap),
            [P, F, F, P, C, C, P] => Some(DrawCommand::Compose),
            [P, F, F, I, C, C, F] => Some(DrawCommand::Clip),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn load_test() {
        let image = load_source().unwrap();
        let pixel = image.get_pixel(0, 0);
        assert_eq!(pixel, &Rgba([62, 39, 76, 255]));
    }
}