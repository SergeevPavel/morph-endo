use std::cmp::{max, min};
use std::path::{PathBuf};

use eframe::{egui::{self, Color32, Event, Key, ColorImage, TextureHandle, RichText}, HardwareAcceleration, Theme};
use eframe::egui::{Slider};
use image::RgbaImage;

use crate::{drawer::Drawer, image::DrawCommand};

struct GuiImage {
    texture_handle: TextureHandle,
    width: f32,
    height: f32,
}

pub struct EndoApp {
    images: Vec<GuiImage>,
    current_image: Option<usize>,

    commands: Vec<DrawCommand>,
    current_command: usize,
    drawer_states: Vec<Drawer>,
}

impl EndoApp {
    fn new(commands: Vec<DrawCommand>) -> Self {
        EndoApp {
            images: Vec::new(),
            commands,
            current_image: None,
            current_command: 0,
            drawer_states: vec![Drawer::new()]
        }
    }

    fn reload_bitmaps(&mut self, ctx: &egui::Context) {
        let drawer_state_index = self.current_command + 1;
        while self.drawer_states.len() <= drawer_state_index {
            let i = self.drawer_states.len() - 1;
            let mut drawer_state = self.drawer_states[i].clone();
            drawer_state.apply(self.commands[i]);
            self.drawer_states.push(drawer_state);
        }
        let drawer = &self.drawer_states[drawer_state_index];
        self.images.clear();
        for bitmap in drawer.bitmaps.iter() {
            self.images.push(load_image(ctx, &bitmap));
        }
        self.select_image(|id| id);
    }

    fn select_image<F: FnOnce(usize) -> usize>(&mut self, f: F) {
        if self.images.len() == 0 {
            self.current_image = None;
        } else {
            let idx = self.current_image.map(f).unwrap_or(0);
            self.current_image = Some(idx.max(0).min(self.images.len() - 1))
        }
    }
}

fn load_image(ctx: &egui::Context, image: &RgbaImage) -> GuiImage {
    let pixels: Vec<Color32> = image
        .pixels()
        .map(|p| Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3]))
        .collect();
    let color_image = ColorImage {
        size: [image.width() as usize, image.height() as usize],
        pixels
    };
    let texture_handle = ctx.load_texture("Some image", color_image, egui::TextureFilter::Nearest);
    GuiImage {
        texture_handle,
        width: image.width() as f32,
        height: image.height() as f32,
    }
}

impl eframe::App for EndoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for event in &ctx.input().events {
            match event {
                Event::Key { key: Key::Space, pressed: true, .. } => {
                    self.reload_bitmaps(ctx);
                }
                Event::Key { key: Key::ArrowDown, pressed: true, modifiers } => {
                    let step = if modifiers.shift {
                        100
                    } else {
                        1
                    };
                    if self.current_command < self.commands.len() - step {
                        self.current_command += step;
                    }
                    self.reload_bitmaps(ctx);
                }
                Event::Key { key: Key::ArrowUp, pressed: true, modifiers } => {
                    let step = if modifiers.shift {
                        100
                    } else {
                        1
                    };
                    if self.current_command >= step {
                        self.current_command -= step;
                    }
                    self.reload_bitmaps(ctx);
                }
                Event::Key { key: Key::ArrowLeft, pressed: true, modifiers: _ } => {
                    self.select_image(|id| id.saturating_sub(1));
                }
                Event::Key { key: Key::ArrowRight, pressed: true, modifiers: _ } => {
                    self.select_image(|id| id.saturating_add(1));
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.expand_to_include_y(500f32);

                    for (id, image) in self.images.iter().enumerate() {
                        let down_scale = if self.current_image == Some(id) {
                            1f32
                        } else {
                            4f32
                        };
                        ui.image(&image.texture_handle, [image.width / down_scale, image.height / down_scale]);
                    }
                    ui.vertical(|ui| {
                        let r = max(0, self.current_command.saturating_sub(10))..min(self.commands.len(), self.current_command + 10);
                        for row in r {
                            let command_str = format!("{:?}", self.commands[row]);
                            if row == self.current_command {
                                ui.label(RichText::new(command_str).color(Color32::RED));
                            } else {
                                ui.label(command_str);
                            }
                        }
                    });
                });

                if ui.add(Slider::new(&mut self.current_command, 0..=(self.commands.len() - 1))).dragged() {
                    self.reload_bitmaps(ctx);
                }
            })
        });
    }
}

crate::entry_point!("gui", gui_main, _EP_GUI);
fn gui_main() {
    let folder = std::env::args().nth(2).expect("Not enough arguments");
    let commands = crate::utils::load(["cache", &folder, "commands.ron"].iter().collect::<PathBuf>());
    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: false,
        vsync: false,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: false,
        default_theme: Theme::Dark,
        run_and_return: false
    };
    eframe::run_native("Endo", native_options, Box::new(|cc| {
        let mut app = EndoApp::new(commands);
        app.reload_bitmaps(&cc.egui_ctx);
        Box::new(app)
    }))
}
