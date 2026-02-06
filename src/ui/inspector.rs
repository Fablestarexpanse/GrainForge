use egui::Ui;
use crate::app::state::{AppState, EditMode};
use crate::core::parameter::{ParameterValue, ParameterRange};
use crate::core::history::Command;

pub fn show(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Inspector");
    ui.separator();

    match state.active_mode {
        EditMode::Simple => {
            ui.collapsing("Parameters (Simple)", |ui| {
                let mut command_to_push = None;

                for param in &mut state.parameters {
                    let mut current_val = param.value.clone();
                    let mut changed = false;

                    ui.horizontal(|ui| {
                        ui.label(&param.display_name)
                            .on_hover_text(&param.description);

                        match &param.range {
                            ParameterRange::Float { min, max } => {
                                if let ParameterValue::Float(val) = &mut current_val {
                                    if ui.add(egui::Slider::new(val, *min..=*max)).changed() {
                                        changed = true;
                                    }
                                }
                            }
                            _ => { ui.label("TODO"); }
                        }
                    });

                    if changed {
                        command_to_push = Some(Command::SetParameter {
                            param_id: param.id.clone(),
                            old_value: param.value.clone(),
                            new_value: current_val.clone(),
                        });
                        param.value = current_val; 
                    }
                }

                if let Some(cmd) = command_to_push {
                    state.history.push(cmd);
                }
            });
        }
        EditMode::Advanced => {
            ui.label("Node Properties (Advanced Mode)");
            ui.label("Select a node in the graph to view properties.");
        }
    }
}
