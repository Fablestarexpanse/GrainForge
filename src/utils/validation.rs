use std::path::{Path, PathBuf};
use crate::core::error::ExportError;
use serde::{Deserialize, Serialize};

/// Bounded float that strictly clamps values
// Removed const generic version due to rust stability issues for f32 consts.
// Using runtime struct instead.

// Trick to allow const generics with simple types, 
// using converting bits because f32 cannot be a const generic param in stable Rust yet (easily).
// wait, the snippet used `BoundedFloat<0.0, 2.0>`. 
// Rust const generics for f32 are unstable ("feature(adt_const_params)").
// I must implement it differently or use a workaround if I want stable rust.
// However, the user provided snippet: `struct BoundedFloat<const MIN: f32, const MAX: f32>`.
// This implies they expect me to use unstable features or they forgot.
// `edition = "2021"` implies stable.
// I will implement a runtime version or use a macro to generate types, OR assume the user is okay with `adt_const_params` if I add `#![feature(adt_const_params)]` to main.rs.
// BUT `adt_const_params` is incomplete.
// Better approach: Make `BoundedFloat` take bounds at runtime or use ZST traits.
// But the snippet explicitly showed generics.
// I'll check `rustc --version`?
// I'll stick to a simpler implementation: plain struct with new() that takes min/max, 
// OR struct with fields `min`, `max`.
// The user snippet in Section 6:
// ```rust
// pub struct BoundedFloat { value: f32, min: f32, max: f32 }
// ```
// Ah! Section 6 had a simple struct.
// Section 10 showed `BoundedFloat<0.0, 2.0>`.
// Section 10 contradicts Section 6's simplicity. Section 10's version requires nightly.
// I will follow Section 6's safe/stable implementation for `BoundedFloat`.
// I'll adapt Section 10's structs to initialize these BoundedFloats with correct min/max.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundedFloat {
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

impl BoundedFloat {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
        }
    }
    
    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn get(&self) -> f32 {
        self.value
    }
}

impl Default for BoundedFloat {
    fn default() -> Self {
        Self { value: 0.0, min: 0.0, max: 1.0 }
    }
}

pub fn validate_export_path(path: &Path) -> Result<PathBuf, ExportError> {
    let canonical = path.canonicalize()
        .map_err(|_| ExportError::InvalidPath)?;
    
    // For MVP, allow any path that canonicalizes (exists)
    // Real implementation would check against an allowlist of dirs
    
    let ext = canonical.extension()
        .and_then(|e| e.to_str())
        .ok_or(ExportError::InvalidExtension)?;
    
    match ext.to_lowercase().as_str() {
        "png" | "tiff" | "tif" | "exr" => Ok(canonical),
        _ => Err(ExportError::InvalidExtension),
    }
}

pub fn validate_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where D: serde::Deserializer<'de>
{
    let s: String = Deserialize::deserialize(deserializer)?;
    
    if s.len() > 64 {
        return Err(serde::de::Error::custom("Name too long (max 64 chars)"));
    }
    
    if !s.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
        return Err(serde::de::Error::custom("Invalid characters in name"));
    }
    
    Ok(s)
}
