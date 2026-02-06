use egui::Ui;
use crate::app::state::AppState;

pub fn show(ui: &mut Ui, _state: &mut AppState) {
    ui.heading("Presets");
    ui.separator();
    ui.collapsing("My Stocks", |ui| {
        ui.label("User preset 1");
    });
    ui.collapsing("Real Film", |ui| {
        ui.label("Tri-X 400");
        ui.label("Portra 400");
    });
}
