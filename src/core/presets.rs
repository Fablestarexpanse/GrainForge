use crate::core::film_stock::{
    FilmStock, FilmMeta, GrainParameters, ResponseCurve, ColorParameters,
    TextureParameters, CrystalType, ResponseMode, ClusteringType,
};
use crate::utils::validation::BoundedFloat;

/// Built-in presets for common film grain types
pub fn get_builtin_presets() -> Vec<FilmStock> {
    vec![
        fine_grain_preset(),
        medium_grain_preset(),
        coarse_grain_preset(),
    ]
}

/// Fine grain - like Kodak Ektar 100 or Velvia 50
fn fine_grain_preset() -> FilmStock {
    FilmStock {
        meta: FilmMeta {
            name: "Fine Grain".to_string(),
            description: Some("Clean, minimal grain like Ektar 100 or Velvia 50".to_string()),
            author: Some("GrainForge".to_string()),
            version: 1,
            tags: vec!["fine".to_string(), "clean".to_string(), "low-iso".to_string()],
            is_real_stock: false,
        },
        grain: GrainParameters {
            intensity: BoundedFloat::new(0.2, 0.0, 2.0),
            size: BoundedFloat::new(0.5, 0.1, 3.0),
            size_variation: BoundedFloat::new(0.3, 0.0, 2.0),
            crystal_type: CrystalType::Tabular,
            sharpness: BoundedFloat::new(0.7, 0.0, 1.0),
        },
        response: ResponseCurve {
            shadows: BoundedFloat::new(0.4, 0.0, 2.0),
            midtones: BoundedFloat::new(0.5, 0.0, 2.0),
            highlights: BoundedFloat::new(0.3, 0.0, 2.0),
            mode: ResponseMode::Negative,
        },
        color: ColorParameters {
            is_color: true,
            channel_intensity: [
                BoundedFloat::new(0.9, 0.0, 3.0),
                BoundedFloat::new(1.0, 0.0, 3.0),
                BoundedFloat::new(1.1, 0.0, 3.0),
            ],
            channel_size: [
                BoundedFloat::new(0.9, 0.5, 2.0),
                BoundedFloat::new(1.0, 0.5, 2.0),
                BoundedFloat::new(1.1, 0.5, 2.0),
            ],
            correlation: BoundedFloat::new(0.3, -1.0, 1.0),
            dye_softness: BoundedFloat::new(0.2, 0.0, 1.0),
        },
        texture: TextureParameters {
            clustering: ClusteringType::None,
            cluster_size: BoundedFloat::new(1.0, 1.0, 50.0),
            organic: BoundedFloat::new(1.0, 1.0, 2.0),
            detail: BoundedFloat::new(6.0, 1.0, 8.0),
            swirl: BoundedFloat::new(0.0, 0.0, 5.0),
        },
    }
}

/// Medium grain - like Portra 400 or Tri-X
fn medium_grain_preset() -> FilmStock {
    FilmStock {
        meta: FilmMeta {
            name: "Medium Grain".to_string(),
            description: Some("Classic film look like Portra 400 or Tri-X".to_string()),
            author: Some("GrainForge".to_string()),
            version: 1,
            tags: vec!["medium".to_string(), "classic".to_string(), "400-iso".to_string()],
            is_real_stock: false,
        },
        grain: GrainParameters {
            intensity: BoundedFloat::new(0.5, 0.0, 2.0),
            size: BoundedFloat::new(1.0, 0.1, 3.0),
            size_variation: BoundedFloat::new(0.5, 0.0, 2.0),
            crystal_type: CrystalType::Cubic,
            sharpness: BoundedFloat::new(0.5, 0.0, 1.0),
        },
        response: ResponseCurve {
            shadows: BoundedFloat::new(0.6, 0.0, 2.0),
            midtones: BoundedFloat::new(0.5, 0.0, 2.0),
            highlights: BoundedFloat::new(0.4, 0.0, 2.0),
            mode: ResponseMode::Negative,
        },
        color: ColorParameters {
            is_color: true,
            channel_intensity: [
                BoundedFloat::new(1.0, 0.0, 3.0),
                BoundedFloat::new(1.0, 0.0, 3.0),
                BoundedFloat::new(1.0, 0.0, 3.0),
            ],
            channel_size: [
                BoundedFloat::new(1.0, 0.5, 2.0),
                BoundedFloat::new(1.0, 0.5, 2.0),
                BoundedFloat::new(1.0, 0.5, 2.0),
            ],
            correlation: BoundedFloat::new(0.0, -1.0, 1.0),
            dye_softness: BoundedFloat::new(0.3, 0.0, 1.0),
        },
        texture: TextureParameters {
            clustering: ClusteringType::Poisson,
            cluster_size: BoundedFloat::new(5.0, 1.0, 50.0),
            organic: BoundedFloat::new(1.2, 1.0, 2.0),
            detail: BoundedFloat::new(4.0, 1.0, 8.0),
            swirl: BoundedFloat::new(0.5, 0.0, 5.0),
        },
    }
}

/// Coarse grain - like pushed Tri-X or Delta 3200
fn coarse_grain_preset() -> FilmStock {
    FilmStock {
        meta: FilmMeta {
            name: "Coarse Grain".to_string(),
            description: Some("Heavy grain like pushed Tri-X or Delta 3200".to_string()),
            author: Some("GrainForge".to_string()),
            version: 1,
            tags: vec!["coarse".to_string(), "gritty".to_string(), "high-iso".to_string()],
            is_real_stock: false,
        },
        grain: GrainParameters {
            intensity: BoundedFloat::new(1.0, 0.0, 2.0),
            size: BoundedFloat::new(2.0, 0.1, 3.0),
            size_variation: BoundedFloat::new(1.0, 0.0, 2.0),
            crystal_type: CrystalType::Cellular,
            sharpness: BoundedFloat::new(0.3, 0.0, 1.0),
        },
        response: ResponseCurve {
            shadows: BoundedFloat::new(0.8, 0.0, 2.0),
            midtones: BoundedFloat::new(0.6, 0.0, 2.0),
            highlights: BoundedFloat::new(0.5, 0.0, 2.0),
            mode: ResponseMode::Negative,
        },
        color: ColorParameters {
            is_color: false, // B&W
            channel_intensity: [
                BoundedFloat::new(1.0, 0.0, 3.0),
                BoundedFloat::new(1.0, 0.0, 3.0),
                BoundedFloat::new(1.0, 0.0, 3.0),
            ],
            channel_size: [
                BoundedFloat::new(1.0, 0.5, 2.0),
                BoundedFloat::new(1.0, 0.5, 2.0),
                BoundedFloat::new(1.0, 0.5, 2.0),
            ],
            correlation: BoundedFloat::new(0.8, -1.0, 1.0),
            dye_softness: BoundedFloat::new(0.0, 0.0, 1.0),
        },
        texture: TextureParameters {
            clustering: ClusteringType::Fractal,
            cluster_size: BoundedFloat::new(15.0, 1.0, 50.0),
            organic: BoundedFloat::new(1.5, 1.0, 2.0),
            detail: BoundedFloat::new(3.0, 1.0, 8.0),
            swirl: BoundedFloat::new(1.5, 0.0, 5.0),
        },
    }
}
