use egui::Ui;
use crate::app::state::AppState;

pub fn show(ui: &mut Ui, _state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("🎬 GrainForge");
        ui.separator();
        if ui.button("New").clicked() {}
        if ui.button("Open").clicked() {}
        if ui.button("Save").clicked() {}
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⚙").clicked() {}
            if ui.button("Export ▼").clicked() {}
            ui.separator();
            
            // Mode Switcher
            ui.selectable_value(&mut _state.active_mode, crate::app::state::EditMode::Simple, "Simple");
            ui.selectable_value(&mut _state.active_mode, crate::app::state::EditMode::Advanced, "Advanced");
        });
    });
}
