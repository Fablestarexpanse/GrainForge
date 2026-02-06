use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    Selection(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterCurve {
    // Placeholder for curve data (e.g., control points)
    pub points: Vec<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterRange {
    Float { min: f32, max: f32 },
    Int { min: i32, max: i32 },
    Selection(Vec<String>),
    Curve(ParameterCurve),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub value: ParameterValue,
    pub default_value: ParameterValue,
    pub range: ParameterRange,
}

impl Parameter {
    pub fn new_float(id: &str, name: &str, desc: &str, val: f32, min: f32, max: f32) -> Self {
        Self {
            id: id.to_string(),
            display_name: name.to_string(),
            description: desc.to_string(),
            value: ParameterValue::Float(val),
            default_value: ParameterValue::Float(val),
            range: ParameterRange::Float { min, max },
        }
    }
    
    // Helper to reset to default
    pub fn reset(&mut self) {
        self.value = self.default_value.clone();
    }
}
