use std::cmp::max;
use std::convert::TryInto;

use image::{ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};

use crate::image::{Color, DrawCommand, Pixel, Alpha};
use std::ops::Deref;
use crate::utils::load;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_c(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_cc(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

trait Bitmap {
    fn set_pixel(&mut self, position: Position, pixel: Pixel);
    fn draw_line(&mut self, p0: Position, p1: Position, pixel: Pixel);
    fn fill(&mut self, p: Position, initial: Pixel, new: Pixel);
}

impl Bitmap for RgbaImage {
    fn set_pixel(&mut self, position: Position, pixel: Pixel) {
        self.put_pixel(position.x.try_into().unwrap(), position.y.try_into().unwrap(), pixel);
    }

    fn draw_line(&mut self, p0: Position, p1: Position, pixel: Pixel) {
        println!("Draw line: {:?} {:?} {:?}", p0, p1, pixel);
        let Position { x: x0, y: y0 } = p0;
        let Position { x: x1, y: y1 } = p1;
        let delta_x = x1 - x0;
        let delta_y = y1 - y0;
        let d = max(delta_x.abs(), delta_y.abs());
        let c = if delta_x * delta_y <= 0 { 1 } else { 0 };
        let mut x = x0 * d + ((d - c) as f32 / 2f32).floor() as i32;
        let mut y = y0 * d + ((d - c) as f32 / 2f32).floor() as i32;
        for _ in 0..d {
            let position = Position {
                x: (x as f32 / d as f32).floor() as i32,
                y: (y as f32 / d as f32).floor() as i32,
            };
            self.set_pixel(position, pixel);
            x += delta_x;
            y += delta_y;
        }
        self.set_pixel(p1, pixel);
    }

    fn fill(&mut self, p: Position, initial: Pixel, new: Pixel) {
        if initial == new {
            return;
        }
        let mut stack = Vec::new();
        if *self.get_pixel(p.x as u32, p.y as u32) == initial {
            stack.push(p);
        }
        loop {
            if let Some(current) = stack.pop() {
                self.set_pixel(current, new);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let x = current.x + dx;
                    let y = current.y + dy;
                    if x >= 0 && x < self.width() as i32 && y >= 0 && y < self.height() as i32 &&
                        *self.get_pixel(x as u32, y as u32) == initial {
                        stack.push(Position { x, y });
                    }
                }
            } else {
                break
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Drawer {
    bucket: Vec<Color>,
    position: Position,
    mark: Position,
    direction: Direction,
    // reversed stack
    bitmaps: Vec<RgbaImage>,
    max_x: i32,
    max_y: i32,
}

impl Drawer {
    pub fn new() -> Self {
        let max_x = 600;
        let max_y = 600;
        let transparent_bitmap = ImageBuffer::from_fn(max_x as u32, max_y as u32, |_x, _y| {
            Rgba([0, 0, 0, 255])
        });
        Drawer {
            bucket: Vec::new(),
            position: Position { x: 0, y: 0},
            mark: Position { x: 0, y: 0},
            direction: Direction::East,
            bitmaps: vec![transparent_bitmap],
            max_x,
            max_y,
        }
    }

    fn current_pixel(&self) -> Pixel {
        println!("Bucket: {:?}", self.bucket);
        fn average(values: &Vec<u8>, default: u64) -> u64 {
            if values.is_empty() {
                return default;
            } else {
                let s: u64 = values.iter().fold(0u64, |acc, v| acc + *v as u64);
                s / values.len() as u64
            }
        }
        let reds: Vec<_> = self.bucket.iter().filter_map(|color| {
            match color {
                Color::Rgb(rgb) => {
                    Some(rgb.encode()[0])
                }
                Color::Alpha(_) => None
            }
        }).collect();
        let grens: Vec<_> = self.bucket.iter().filter_map(|color| {
            match color {
                Color::Rgb(rgb) => {
                    Some(rgb.encode()[1])
                }
                _ => None
            }
        }).collect();
        let blues: Vec<_> = self.bucket.iter().filter_map(|color| {
            match color {
                Color::Rgb(rgb) => {
                    Some(rgb.encode()[2])
                }
                Color::Alpha(_) => None
            }
        }).collect();
        let transparents: Vec<_> = self.bucket.iter().filter_map(|color| {
            match color {
                Color::Alpha(t) => Some(t.encode()),
                _ => None
            }
        }).collect();
        let transparency = average(&transparents, 255);
        Rgba([(average(&reds, 0) * transparency / 255).try_into().unwrap(),
              (average(&blues, 0) * transparency / 255).try_into().unwrap(),
              (average(&grens, 0) * transparency / 255).try_into().unwrap(),
              transparency.try_into().unwrap()])
    }

    pub fn apply(&mut self, command: DrawCommand) {
        match command {
            DrawCommand::AddColor(color) => {
                self.bucket.push(color);
            }
            DrawCommand::ClearBucket => {
                self.bucket.clear()
            }
            DrawCommand::Move => {
                match self.direction {
                    Direction::North => {
                        self.position = Position {
                            x: self.position.x,
                            y: (self.position.y - 1).rem_euclid(self.max_y)
                        };
                    }
                    Direction::East => {
                        self.position = Position {
                            x: (self.position.x + 1).rem_euclid(self.max_x),
                            y: self.position.y
                        };
                    }
                    Direction::South => {
                        self.position = Position {
                            x: self.position.x,
                            y: (self.position.y + 1).rem_euclid(self.max_y),
                        };
                    }
                    Direction::West => {
                        self.position = Position {
                            x: (self.position.x - 1).rem_euclid(self.max_x),
                            y: self.position.y
                        };
                    }
                }
            }
            DrawCommand::TurnCC => {
                self.direction = self.direction.turn_cc();
            }
            DrawCommand::TurnC => {
                self.direction = self.direction.turn_c();
            }
            DrawCommand::Mark => {
                self.mark = self.position;
            }
            DrawCommand::Line => {
                let current_pixel = self.current_pixel();
                self.bitmaps.last_mut().unwrap().draw_line(self.position, self.mark,
                                                           current_pixel);
            }
            DrawCommand::TryFill => {

            }
            DrawCommand::AddBitmap => {}
            DrawCommand::Compose => {}
            DrawCommand::Clip => {}
        }
    }
}

crate::entry_point!("drawer", drawer_main);
fn drawer_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let commands: Vec<DrawCommand> = load([&folder, "commands.ron"].iter().collect::<PathBuf>());
    let mut drawer = Drawer::new();
    for command in commands {
        drawer.apply(command);
    }
    drawer.bitmaps.last().unwrap().save(["data", &folder, "result.png"].iter().collect::<PathBuf>()).unwrap();
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn with_image<N, F>(file_name: N, f: F)
        where
            F: FnOnce(&mut RgbaImage),
            N: AsRef<str>,
    {
        let mut image = ImageBuffer::from_fn(60, 60, |_x, _y| {
            Rgba([0, 0, 0, 255])
        });
        f(&mut image);
        let path: PathBuf = ["data", "tests", file_name.as_ref()].iter().collect();
        if path.exists() {
            let actual = crate::image::load_from_file(path).unwrap();
            if image != actual {
                let path: PathBuf = ["data", "tests", "unexpected", file_name.as_ref()].iter().collect();
                image.save(&path).unwrap();
                panic!("Unexpected image: {:?}", path);
            }
        } else {
            image.save(path).unwrap();
        }
    }

    #[test]
    fn line_test() {
        with_image("lines1.png", |bitmap| {
            bitmap.draw_line(Position { x: 10, y: 11}, Position { x: 30, y: 15 }, Rgba([255, 255, 255, 255]));
        });
        with_image("lines2.png", |bitmap| {
            bitmap.draw_line(Position { x: 0, y: 0}, Position { x: 59, y: 59 }, Rgba([255, 255, 255, 255]));
            bitmap.draw_line(Position { x: 0, y: 59}, Position { x: 59, y: 0 }, Rgba([255, 255, 255, 255]));
        });
    }

    #[test]
    fn fill_test() {
        with_image("fill1.png", |bitmap| {
            bitmap.fill(Position { x: 10, y: 10}, Rgba([0, 0, 0, 255]), Rgba([0, 255, 0, 255]));
        });
        with_image("fill2.png", |bitmap| {
            bitmap.draw_line(Position { x: 0, y: 0}, Position { x: 59, y: 59 }, Rgba([255, 255, 255, 255]));
            bitmap.draw_line(Position { x: 0, y: 59}, Position { x: 59, y: 0 }, Rgba([255, 255, 255, 255]));
            bitmap.draw_line(Position { x: 0, y: 29}, Position { x: 59, y: 29 }, Rgba([255, 255, 255, 255]));
            bitmap.fill(Position { x: 29, y: 29}, Rgba([255, 255, 255, 255]), Rgba([0, 255, 0, 255]));
        });
    }
}