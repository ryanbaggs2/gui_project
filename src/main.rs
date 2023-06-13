use eframe::{egui, Frame};
use egui::{Align, Align2, Context, Direction, vec2};

fn main() -> Result<(), eframe::Error>{
    let options = eframe::NativeOptions {
        icon_data: None,
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Confirm Exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::containers::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Try to close the window");
        });

        if self.show_confirmation_dialog {
            // Show the confirmation dialog.
            egui::containers::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    egui::Grid::new("some id").show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                self.allowed_to_close = true;
                                frame.close();
                            }
                            if ui.button("Cancel").clicked() {
                                self.show_confirmation_dialog = false;
                            }
                        });
                        ui.end_row();
                    });
                });

        }
    }

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }
}
