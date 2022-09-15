use std::path::{PathBuf};

use eframe::{AppCreator, egui::{self, Color32, Event, Key, Label, ScrollArea, TextStyle, TextureId}, HardwareAcceleration, Theme};
use eframe::egui::{ColorImage, TextureHandle, RichText};
use image::RgbaImage;

use crate::{drawer::Drawer, image::DrawCommand};
use crate::image::load_source;

struct GuiImage {
    texture_handle: TextureHandle,
    width: f32,
    height: f32,
}

pub struct EndoApp {
    images: Vec<GuiImage>,

    commands: Vec<DrawCommand>,
    current_command: usize,
    drawer: Drawer,
}

impl EndoApp {
    fn new(commands: Vec<DrawCommand>) -> Self {
        EndoApp {
            images: Vec::new(),
            commands,
            current_command: 0,
            drawer: Drawer::new()
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

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for event in &ctx.input().events {
            match event {
                Event::Key { key: Key::Space, pressed: true, .. } => {
                    self.images.push(load_image(ctx, &load_source().unwrap()));
                }
                Event::Key { key: Key::ArrowDown, pressed: true, modifiers } => {
                    self.current_command += 1;
                }
                Event::Key { key: Key::ArrowUp, pressed: true, modifiers } => {
                    self.current_command -= 1;
                }
                _ => {}
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.commands.len();
            ui.horizontal(|ui| {
                ui.expand_to_include_y(500f32);

                if let Some(image) = self.images.first() {
                    ui.image(&image.texture_handle, [image.width, image.height]);
                }

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show_rows(ui, row_height, num_rows, |ui, row_range| {
                        ui.vertical(|ui| {
                            for row in row_range {
                                let command_str = format!("{:?}", self.commands[row]);
                                if row == self.current_command {
                                    let responce = ui.label(RichText::new(command_str).color(Color32::RED));
                                    responce.scroll_to_me(Some(egui::Align::Center));
                                } else {
                                    ui.label(command_str);
                                }
                            }
                        })
                    });
            });
        });
    }
}

crate::entry_point!("gui", gui_main, _EP_GUI);
fn gui_main() {
    let commands = crate::utils::load(["endo", "commands.ron"].iter().collect::<PathBuf>());
    let app = EndoApp::new(commands);
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
    eframe::run_native("Endo", native_options, Box::new(|_cc| Box::new(app)))
}
