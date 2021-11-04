use std::path::{Path, PathBuf};

use eframe::{egui::{self, Color32, ScrollArea, TextStyle, TextureId}, epi};

use crate::image::DrawCommand;

struct GuiImage {
    texture_id: TextureId,
    width: f32,
    height: f32,
}

pub struct EndoApp {
    images: Vec<GuiImage>,
    commands: Vec<DrawCommand>,
}

impl Default for EndoApp {
    fn default() -> Self {
        Self {
            images: Vec::new(),
            commands: Vec::new(),
        }
    }
}

fn load_image<P>(frame: &mut epi::Frame<'_>, path: P) -> GuiImage
where
    P: AsRef<Path>,
{
    let image = crate::image::load_from_file(path).unwrap();
    let pixels: Vec<Color32> = image
        .pixels()
        .map(|p| Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3]))
        .collect();
    let texture_id = frame
        .tex_allocator()
        .alloc_srgba_premultiplied((image.width() as usize, image.height() as usize), &pixels);
    GuiImage {
        texture_id,
        width: image.width() as f32,
        height: image.height() as f32,
    }
}

impl epi::App for EndoApp {
    fn name(&self) -> &str {
        "Endo"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        self.images.push(load_image(frame, "data/target.png"));
        self.commands = crate::utils::load(["endo", "commands.ron"].iter().collect::<PathBuf>());
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { images, commands } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    images.iter().for_each(|image| {
                        ui.image(image.texture_id, (image.width, image.height));
                    });
                });
                ui.vertical(|ui| {
                    let text_style = TextStyle::Body;
                    let row_height = ui.fonts()[text_style].row_height();
                    let num_rows = commands.len();
                    ScrollArea::auto_sized().show_rows(
                        ui,
                        row_height,
                        num_rows,
                        |ui, row_range| {
                            for row in row_range {
                                ui.label(format!("{:?}", commands[row]));
                            }
                        },
                    );
                });
            });
        });
    }
}

crate::entry_point!("gui", gui_main);
fn gui_main() {
    let app = EndoApp::default();
    let mut native_options = eframe::NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_size: None,
        resizable: true,
        transparent: false,
    };
    eframe::run_native(Box::new(app), native_options)
}
