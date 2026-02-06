use crate::core::parameter::Parameter;
use crate::core::history::HistoryManager;

pub struct AppState {
    pub parameters: Vec<Parameter>,
    pub history: HistoryManager,
    pub active_mode: EditMode,
    // Grain preview parameters (bound to sliders)
    pub grain_amount: f32,
    pub grain_size: f32,
    pub preview_seed: f32,
}

#[derive(Default, PartialEq)]
pub enum EditMode {
    #[default]
    Simple,
    Advanced,
}

impl Default for AppState {
    fn default() -> Self {
        let mut state = Self {
            parameters: Vec::new(),
            history: HistoryManager::default(),
            active_mode: EditMode::Simple,
            grain_amount: 0.5,
            grain_size: 1.0,
            preview_seed: 0.0,
        };
        state.init_default_parameters();
        state
    }
}

impl AppState {
    fn init_default_parameters(&mut self) {
        // Section 4 Example Mappings
        self.parameters.push(Parameter::new_float(
            "grain_amount",
            "Grain Amount",
            "Controls the visual intensity of the grain structure (RMS).",
            0.5, 0.0, 1.0
        ));
        
        self.parameters.push(Parameter::new_float(
            "grain_size", 
            "Grain Size",
            "Average diameter of silver halide crystals.",
            1.0, 0.1, 5.0
        ));
    }
}

