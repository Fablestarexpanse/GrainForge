use egui::{Context, SidePanel, TopBottomPanel, CentralPanel};
use crate::app::state::AppState;

pub fn show(ctx: &Context, state: &mut AppState) {
    // Global Keyboard Shortcuts
    if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Z)) {
        state.history.undo(&mut state.parameters);
    }
    if ctx.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, egui::Key::Y)) {
        state.history.redo(&mut state.parameters);
    }

    // Toolbar (Top)
    TopBottomPanel::top("toolbar").show(ctx, |ui| {
        crate::ui::toolbar::show(ui, state);
    });

    // SB (Bottom)
    TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Ready");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("GPU: Active");
            });
        });
    });

    // Sidebar (Left - Resizable)
    SidePanel::left("sidebar")
        .default_width(240.0)
        .resizable(true)
        .show(ctx, |ui| {
            crate::ui::sidebar::show(ui, state);
        });

    // Inspector (Right - Resizable)
    SidePanel::right("inspector")
        .default_width(320.0)
        .resizable(true)
        .show(ctx, |ui| {
            crate::ui::inspector::show(ui, state);
        });

    // Central Preview
    CentralPanel::default().show(ctx, |ui| {
        crate::ui::preview::show(ui, state);
    });
}
