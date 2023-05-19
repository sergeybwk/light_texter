mod files;
mod pdf_converter;

use std::path::PathBuf;

use egui::ScrollArea;
use rfd;

struct App {
    text: String,
    picked_path: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            text: files::read_from_file(None),
            picked_path: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("New File").clicked() {
                    files::write_to_file(&self.text, self.picked_path.clone());
                    self.text = "".to_owned();
                    self.picked_path = None;
                }
                if ui.button("Choose File").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.clone());
                        self.text = files::read_from_file(Some(path.clone()));
                    }
                }
                ui.add_space(700.0);
                if ui.button("Save").clicked() {
                    files::write_to_file(&self.text, self.picked_path.clone());
                }
                if ui.button("Save as").clicked() {
                    let path = rfd::FileDialog::new().save_file();
                    files::write_to_file(&self.text, path.clone());
                    self.picked_path = path.clone();
                }
                if ui.button("Export as pdf").clicked() {
                    files::write_to_file(&self.text, self.picked_path.clone());
                    pdf_converter::create_pdf(self.picked_path.clone());
                    files::open_file();
                }
            });
            ui.add_space(10.0);
            ui.centered_and_justified(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .frame(true),
                    );
                });
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        files::write_to_file(&self.text, self.picked_path.clone());
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "light_texter",
        options,
        Box::new(|_cc| (Box::new(App::default()))),
    )
    .expect("Something is wrong with eframe run");

    Ok(())
}
