use std::cmp::max;
use std::convert::TryInto;

use image::{ImageBuffer, Rgba, RgbaImage};
use serde::{Deserialize, Serialize};

use crate::image::{Color, DrawCommand, Pixel};
use crate::utils::load;
use std::path::PathBuf;
use std::rc::Rc;

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
    fn fill(&mut self, p: Position, new: Pixel);
    fn compose(&mut self, other: &RgbaImage);
    fn clip(&mut self, other: &RgbaImage);
}

impl Bitmap for RgbaImage {
    fn set_pixel(&mut self, position: Position, pixel: Pixel) {
        self.put_pixel(position.x.try_into().unwrap(), position.y.try_into().unwrap(), pixel);
    }

    fn draw_line(&mut self, p0: Position, p1: Position, pixel: Pixel) {
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

    fn fill(&mut self, p: Position, new: Pixel) {
        let initial = *self.get_pixel(p.x as u32, p.y as u32);
        if initial == new {
            return;
        }
        let mut stack = Vec::new();
        stack.push(p);
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

    fn compose(&mut self, other: &RgbaImage) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let [r0, g0, b0, a0] = other.get_pixel(x, y).0;
                let [r1, g1, b1, a1] = self.get_pixel(x, y).0;
                self.put_pixel(x, y, Rgba([
                    r0 + ((r1 as u32) * ((255 - a0) as u32) / 255) as u8,
                    g0 + ((g1 as u32) * ((255 - a0) as u32) / 255) as u8,
                    b0 + ((b1 as u32) * ((255 - a0) as u32) / 255) as u8,
                    a0 + ((a1 as u32) * ((255 - a0) as u32) / 255) as u8
                ]));
            }
        }
    }

    #[allow(unused_variables)]
    fn clip(&mut self, other: &RgbaImage) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let [r0, g0, b0, a0] = other.get_pixel(x, y).0;
                let [r1, g1, b1, a1] = self.get_pixel(x, y).0;
                self.put_pixel(x, y, Rgba([
                    (r1 as u32 * a0 as u32 / 255) as u8,
                    (g1 as u32 * a0 as u32 / 255) as u8,
                    (b1 as u32 * a0 as u32 / 255) as u8,
                    (a1 as u32 * a0 as u32 / 255) as u8
                ]));
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
    pub bitmaps: Rc<Vec<RgbaImage>>,
    max_x: i32,
    max_y: i32,
}

fn empty_bitmap(max_x: i32, max_y: i32) -> RgbaImage {
    ImageBuffer::from_fn(max_x as u32, max_y as u32, |_x, _y| {
        Rgba([0, 0, 0, 255])
    })
}

fn current_pixel(bucket: &Vec<Color>) -> Pixel {
    fn average(values: &Vec<u8>, default: u64) -> u64 {
        if values.is_empty() {
            return default;
        } else {
            let s: u64 = values.iter().fold(0u64, |acc, v| acc + *v as u64);
            s / values.len() as u64
        }
    }
    let reds: Vec<_> = bucket.iter().filter_map(|color| {
        match color {
            Color::Rgb(rgb) => {
                Some(rgb.encode()[0])
            }
            Color::Alpha(_) => None
        }
    }).collect();
    let grens: Vec<_> = bucket.iter().filter_map(|color| {
        match color {
            Color::Rgb(rgb) => {
                Some(rgb.encode()[1])
            }
            Color::Alpha(_) => None
        }
    }).collect();
    let blues: Vec<_> = bucket.iter().filter_map(|color| {
        match color {
            Color::Rgb(rgb) => {
                Some(rgb.encode()[2])
            }
            Color::Alpha(_) => None
        }
    }).collect();
    let transparents: Vec<_> = bucket.iter().filter_map(|color| {
        match color {
            Color::Alpha(t) => Some(t.encode()),
            _ => None
        }
    }).collect();
    let transparency = average(&transparents, 255);
    Rgba([(average(&reds, 0) * transparency / 255).try_into().unwrap(),
          (average(&grens, 0) * transparency / 255).try_into().unwrap(),
          (average(&blues, 0) * transparency / 255).try_into().unwrap(),
          transparency.try_into().unwrap()])
}

impl Drawer {
    pub fn new() -> Self {
        let max_x = 600;
        let max_y = 600;
        Drawer {
            bucket: Vec::new(),
            position: Position { x: 0, y: 0},
            mark: Position { x: 0, y: 0},
            direction: Direction::East,
            bitmaps: Rc::new(vec![empty_bitmap(max_x, max_y)]),
            max_x,
            max_y,
        }
    }

    fn current_pixel(&self) -> Pixel {
        current_pixel(&self.bucket)
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
                Rc::make_mut(&mut self.bitmaps)
                    .last_mut().unwrap()
                    .draw_line(self.position, self.mark, current_pixel);
            }
            DrawCommand::TryFill => {
                let current_pixel = self.current_pixel();
                Rc::make_mut(&mut self.bitmaps)
                    .last_mut().unwrap()
                    .fill(self.position, current_pixel);
            }
            DrawCommand::AddBitmap => {
                if self.bitmaps.len() < 10 {
                    Rc::make_mut(&mut self.bitmaps).push(empty_bitmap(self.max_x, self.max_y));
                }
            }
            DrawCommand::Compose => {
                if self.bitmaps.len() >= 2 {
                    let bitmaps = Rc::make_mut(&mut self.bitmaps);
                    let top = bitmaps.pop().unwrap();
                    bitmaps.last_mut().unwrap().compose(&top);
                }
            }
            DrawCommand::Clip => {
                if self.bitmaps.len() >= 2 {
                    let bitmaps = Rc::make_mut(&mut self.bitmaps);
                    let top = bitmaps.pop().unwrap();
                    bitmaps.last_mut().unwrap().clip(&top);
                }
            }
        }
    }

    pub fn apply_all(&mut self, commands: &[DrawCommand]) {
        for command in commands {
            self.apply(*command)
        }
    }
}

crate::entry_point!("drawer", drawer_main);
fn drawer_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let commands: Vec<DrawCommand> = load([&folder, "commands.ron"].iter().collect::<PathBuf>());
    let mut drawer = Drawer::new();
    let images_dir = ["data", &folder, "images"].iter().collect::<PathBuf>();
    if images_dir.exists() {
        std::fs::remove_dir_all(&images_dir).unwrap();
    }
    std::fs::create_dir_all(&images_dir).unwrap();
    for (idx, command) in commands.into_iter().enumerate() {
        drawer.apply(command);
        if idx % 100 == 0 {
            drawer.bitmaps.last().unwrap().save(images_dir.join(format!("image_{}.png", idx))).unwrap();
        }
    }
    // drawer.bitmaps.last_mut().unwrap().fill(Position {x : 0, y: 0},  Rgba([0, 0, 0, 255]));
    drawer.bitmaps.last().unwrap().save(["data", &folder, "result.png"].iter().collect::<PathBuf>()).unwrap();
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::image::{Rgb, Alpha};

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
            bitmap.fill(Position { x: 10, y: 10}, Rgba([0, 255, 0, 255]));
        });
        with_image("fill2.png", |bitmap| {
            bitmap.draw_line(Position { x: 0, y: 0}, Position { x: 59, y: 59 }, Rgba([255, 255, 255, 255]));
            bitmap.draw_line(Position { x: 0, y: 59}, Position { x: 59, y: 0 }, Rgba([255, 255, 255, 255]));
            bitmap.draw_line(Position { x: 0, y: 29}, Position { x: 59, y: 29 }, Rgba([255, 255, 255, 255]));
            bitmap.fill(Position { x: 29, y: 29}, Rgba([0, 255, 0, 255]));
        });
    }

    #[test]
    fn current_pixel_test() {
        assert_eq!(current_pixel(&vec![]), Rgba([0, 0, 0, 255]));
        let b = Color::Rgb(Rgb::Black);
        let r = Color::Rgb(Rgb::Red);
        let m = Color::Rgb(Rgb::Magenta);
        let w = Color::Rgb(Rgb::White);
        let y = Color::Rgb(Rgb::Yellow);
        let c = Color::Rgb(Rgb::Cyan);
        let t = Color::Alpha(Alpha::Transparent);
        let o = Color::Alpha(Alpha::Opaque);
        assert_eq!(current_pixel(&vec![t, o, o]), Rgba([0, 0, 0, 170]));
        assert_eq!(current_pixel(&vec![b, y, c]), Rgba([85, 170, 85, 255]));
        assert_eq!(current_pixel(&vec![y, t, o]), Rgba([127, 127, 0, 127]));
        let bucket: Vec<_> =
            std::iter::repeat(b).take(18).chain(
                std::iter::repeat(r).take(7).chain(
                    std::iter::repeat(m).take(39).chain(
                        std::iter::repeat(w).take(10).chain(
                            std::iter::repeat(o).take(3).chain(
                                std::iter::repeat(t).take(1)))))).collect();
        assert_eq!(current_pixel(&bucket), Rgba([143, 25, 125, 191]));
    }
}