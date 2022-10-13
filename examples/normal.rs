use egui::CentralPanel;
use egui_tabs::tabs::Tabs;

#[derive(Default)]
struct NormalTabs {
    index: usize,
}

impl eframe::App for NormalTabs {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Tabs::new(ui, self.index)
                .add_header_section(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Tab 1").clicked() {
                            self.index = 0;
                        }
                        if ui.button("Tab 2").clicked() {
                            self.index = 1;
                        }
                        if ui.button("Tab 3").clicked() {
                            self.index = 2;
                        }
                    });
                })
                .add_body(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Tab 1");
                    });
                })
                .add_body(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Tab 2");
                    });
                })
                .add_body(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label("Tab 3");
                    });
                });
        });
    }
}

impl NormalTabs {
    fn new() -> Self {
        Self { index: 0 }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(NormalTabs::default())),
    );
}
