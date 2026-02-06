use serde::{Deserialize, Serialize};
use crate::utils::validation::{BoundedFloat, validate_name};

/// Represents a complete film stock definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FilmStock {
    pub meta: FilmMeta,
    pub grain: GrainParameters,
    pub response: ResponseCurve,
    pub color: ColorParameters,
    pub texture: TextureParameters,
}

impl Default for FilmStock {
    fn default() -> Self {
        Self {
            meta: FilmMeta::default(),
            grain: GrainParameters::default(),
            response: ResponseCurve::default(),
            color: ColorParameters::default(),
            texture: TextureParameters::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FilmMeta {
    #[serde(deserialize_with = "validate_name")]
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub version: u32,
    #[serde(default)]
    pub tags: Vec<String>,
    pub is_real_stock: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GrainParameters {
    #[serde(default = "default_intensity")]
    pub intensity: BoundedFloat, // 0.0 - 2.0
    
    #[serde(default = "default_size")]
    pub size: BoundedFloat, // 0.1 - 3.0
    
    #[serde(default = "default_variation")]
    pub size_variation: BoundedFloat, // 0.0 - 2.0
    
    #[serde(default)]
    pub crystal_type: CrystalType,
    
    #[serde(default = "default_sharpness")]
    pub sharpness: BoundedFloat, // 0.0 - 1.0
}

impl Default for GrainParameters {
    fn default() -> Self {
        Self {
            intensity: default_intensity(),
            size: default_size(),
            size_variation: default_variation(),
            crystal_type: CrystalType::default(),
            sharpness: default_sharpness(),
        }
    }
}

fn default_intensity() -> BoundedFloat { BoundedFloat::new(0.5, 0.0, 2.0) }
fn default_size() -> BoundedFloat { BoundedFloat::new(1.0, 0.1, 3.0) }
fn default_variation() -> BoundedFloat { BoundedFloat::new(0.5, 0.0, 2.0) }
fn default_sharpness() -> BoundedFloat { BoundedFloat::new(0.5, 0.0, 1.0) }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum CrystalType {
    #[default]
    Cubic,
    Tabular,
    CoreShell,
    Cellular,
    Needle,
    Custom { sides: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCurve {
    #[serde(default = "default_response_param")]
    pub shadows: BoundedFloat,
    #[serde(default = "default_response_param")]
    pub midtones: BoundedFloat,
    #[serde(default = "default_response_param")]
    pub highlights: BoundedFloat,
    #[serde(default)]
    pub mode: ResponseMode,
}

fn default_response_param() -> BoundedFloat { BoundedFloat::new(0.5, 0.0, 2.0) }

impl Default for ResponseCurve {
    fn default() -> Self {
        Self {
            shadows: default_response_param(),
            midtones: default_response_param(),
            highlights: default_response_param(),
            mode: ResponseMode::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResponseMode {
    #[default]
    Negative,
    Print,
    Reversal,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorParameters {
    pub is_color: bool,
    #[serde(default = "default_channel_intensity")]
    pub channel_intensity: [BoundedFloat; 3], // 0.0 - 3.0
    #[serde(default = "default_channel_size")]
    pub channel_size: [BoundedFloat; 3], // 0.5 - 2.0
    #[serde(default = "default_correlation")]
    pub correlation: BoundedFloat, // -1.0 - 1.0
    #[serde(default = "default_dye_softness")]
    pub dye_softness: BoundedFloat, // 0.0 - 1.0
}

fn default_channel_intensity() -> [BoundedFloat; 3] {
    [
        BoundedFloat::new(1.0, 0.0, 3.0),
        BoundedFloat::new(1.0, 0.0, 3.0),
        BoundedFloat::new(1.0, 0.0, 3.0),
    ]
}

fn default_channel_size() -> [BoundedFloat; 3] {
    [
        BoundedFloat::new(1.0, 0.5, 2.0),
        BoundedFloat::new(1.0, 0.5, 2.0),
        BoundedFloat::new(1.0, 0.5, 2.0),
    ]
}
fn default_correlation() -> BoundedFloat { BoundedFloat::new(0.0, -1.0, 1.0) }
fn default_dye_softness() -> BoundedFloat { BoundedFloat::new(0.0, 0.0, 1.0) }

impl Default for ColorParameters {
    fn default() -> Self {
        Self {
            is_color: true,
            channel_intensity: default_channel_intensity(),
            channel_size: default_channel_size(),
            correlation: default_correlation(),
            dye_softness: default_dye_softness(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextureParameters {
    #[serde(default)]
    pub clustering: ClusteringType,
    #[serde(default = "default_cluster_size")]
    pub cluster_size: BoundedFloat, // 1.0 - 50.0
    #[serde(default = "default_organic")]
    pub organic: BoundedFloat, // 1.0 - 2.0
    #[serde(default = "default_detail")]
    pub detail: BoundedFloat, // 1.0 - 8.0
    #[serde(default = "default_swirl")]
    pub swirl: BoundedFloat, // 0.0 - 5.0
}

fn default_cluster_size() -> BoundedFloat { BoundedFloat::new(1.0, 1.0, 50.0) }
fn default_organic() -> BoundedFloat { BoundedFloat::new(1.0, 1.0, 2.0) }
fn default_detail() -> BoundedFloat { BoundedFloat::new(4.0, 1.0, 8.0) }
fn default_swirl() -> BoundedFloat { BoundedFloat::new(0.0, 0.0, 5.0) }

impl Default for TextureParameters {
    fn default() -> Self {
        Self {
            clustering: ClusteringType::default(),
            cluster_size: default_cluster_size(),
            organic: default_organic(),
            detail: default_detail(),
            swirl: default_swirl(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClusteringType {
    #[default]
    None,
    Poisson,
    Fractal,
    Voronoi,
    Hybrid,
}
