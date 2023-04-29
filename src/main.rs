mod files;
mod pdf_converter;

struct App {
    text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            text: match files::read_from_file() {
                Ok(v) => v,
                _ => "".to_owned(),
            },
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.text).font(egui::TextStyle::Monospace));
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        files::write_to_file(&self.text);
        pdf_converter::create_pdf();
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
