use egui::Ui;
use crate::app::state::AppState;

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Preview Canvas");
    
    // Grain parameter sliders
    ui.horizontal(|ui| {
        ui.label("Amount:");
        ui.add(egui::Slider::new(&mut state.grain_amount, 0.0..=1.0));
    });
    ui.horizontal(|ui| {
        ui.label("Size:");
        ui.add(egui::Slider::new(&mut state.grain_size, 0.1..=5.0));
    });
    ui.horizontal(|ui| {
        ui.label("Seed:");
        ui.add(egui::Slider::new(&mut state.preview_seed, 0.0..=100.0));
    });
    
    ui.separator();
    
    // Placeholder for rendered texture
    // In the future, this will display the egui::TextureHandle from GrainRenderer
    let available = ui.available_size();
    let (rect, _response) = ui.allocate_exact_size(available, egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, egui::Color32::from_gray(40));
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("Grain: {:.2} | Size: {:.2}", state.grain_amount, state.grain_size),
        egui::FontId::default(),
        egui::Color32::WHITE,
    );
}

