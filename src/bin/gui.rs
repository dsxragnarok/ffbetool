use std::path::PathBuf;

use eframe::egui;
use egui::{Response, Ui};
use rfd::FileDialog;

#[derive(Default)]
struct App {
    atlas_file: Option<PathBuf>,
    cgs_file: Option<PathBuf>,
    cgg_files: Option<Vec<PathBuf>>,
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
            ui.label("Left Pane");
            if ui.button("Load Atlas File").clicked() {
                self.atlas_file = FileDialog::new().pick_file();
            }
            if let Some(atlas_file) = &self.atlas_file {
                render_file_label(ui, &atlas_file, None);
            }
            if ui.button("Load CGS File").clicked() {
                self.cgs_file = FileDialog::new().pick_file();
            }
            if let Some(cgs_file) = &self.cgs_file {
                render_file_label(ui, &cgs_file, None);
            }
            if ui.button("Load Anim Files").clicked() {
                self.cgg_files = FileDialog::new().pick_files();
            }
            if let Some(paths) = &self.cgg_files {
                for cgg_file in paths {
                    render_file_label(ui, &cgg_file, None);
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::TopBottomPanel::bottom("bottom_pane")
                .resizable(true)
                .show(ctx, |ui| {
                    ui.label("Bottom Pane");
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Image Pane");
            });
        });
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
