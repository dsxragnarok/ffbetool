use eframe::egui;

#[derive(Default)]
struct App {
    // texture: Option<egui::TextureHandle>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_pane").show(ctx, |ui| {
            ui.label("Left Pane");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
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
