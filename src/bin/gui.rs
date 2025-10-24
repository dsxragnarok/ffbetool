use std::path::PathBuf;

use eframe::egui;
use egui::{Response, Ui};
use rfd::FileDialog;

#[derive(Default)]
struct App {
    atlas_file: Option<PathBuf>,
    cgg_file: Option<PathBuf>,
    cgs_files: Option<Vec<PathBuf>>,
    output_dir: Option<PathBuf>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    columns: usize,
    save_gif: bool,
    save_apng: bool,
    save_json: bool,
    include_empty: bool,
}

fn render_file_label(ui: &mut Ui, file_path: &PathBuf, default: Option<&str>) -> Response {
    let label = match file_path.file_name() {
        Some(filename) => filename
            .to_str()
            .unwrap_or_else(|| default.unwrap_or_default()),
        None => "",
    };
    ui.label(label)
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_pane").show(ctx, |ui| {
            if ui.button("Load Atlas File").clicked() {
                self.atlas_file = FileDialog::new().pick_file();
            }
            if let Some(atlas_file) = &self.atlas_file {
                render_file_label(ui, &atlas_file, None);
                self.load_image_from_path(ctx, &atlas_file.clone());
            }
            if ui.button("Load CGG File").clicked() {
                self.cgg_file = FileDialog::new().pick_file();
            }
            if let Some(cgs_file) = &self.cgg_file {
                render_file_label(ui, &cgs_file, None);
            }
            if ui.button("Select Output Directory").clicked() {
                self.output_dir = FileDialog::new().pick_folder();
            }
            if let Some(output_dir) = &self.output_dir {
                render_file_label(ui, &output_dir, None);
            }

            if ui.button("Load Anim Files").clicked() {
                self.cgs_files = FileDialog::new().pick_files();
            }
            ui.label("Animation Files");
            if let Some(paths) = &self.cgs_files {
                for cgg_file in paths {
                    render_file_label(ui, &cgg_file, None);
                }
            }
        });
        egui::SidePanel::right("right_pane").show(ctx, |ui| {
            ui.checkbox(&mut self.save_gif, "gif");
            ui.checkbox(&mut self.save_apng, "apng");
            ui.checkbox(&mut self.save_json, "Output JSON");
            ui.checkbox(&mut self.include_empty, "Render Empty Frames");

            egui::ComboBox::from_label("columns")
                .selected_text(format!("{}", self.columns))
                .show_ui(ui, |ui| {
                    for n in 0..5 {
                        ui.selectable_value(&mut self.columns, n, n.to_string());
                    }
                });

            if ui.button("Start").clicked() {
                // Begin processing
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                ui.add(egui::Image::new(texture).shrink_to_fit());
            } else if let Some(err) = &self.error {
                ui.label(err);
            }
        });
    }
}

impl App {
    fn load_image_from_path(&mut self, ctx: &egui::Context, path: &PathBuf) {
        match image::open(&path) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &rgba);

                self.texture = Some(ctx.load_texture("image", color_image, Default::default()));
            }
            Err(err) => {
                self.texture = None;
                self.error = Some(err.to_string())
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "FFBETool",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
