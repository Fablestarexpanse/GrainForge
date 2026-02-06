use std::path::Path;
use image::{ImageBuffer, Rgba};
use crate::core::error::{GrainError, ExportError};
use crate::utils::validation::validate_export_path;

/// Export a grain texture to PNG format
pub fn export_png(
    data: &[u8],
    width: u32,
    height: u32,
    output_path: &Path,
) -> Result<(), GrainError> {
    // Validate path
    let validated_path = validate_export_path(output_path)?;
    
    // Ensure we have the right amount of data (RGBA)
    let expected_size = (width * height * 4) as usize;
    if data.len() != expected_size {
        return Err(GrainError::Export(ExportError::WriteFailed(
            format!("Data size mismatch: expected {}, got {}", expected_size, data.len())
        )));
    }
    
    // Create image buffer
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, data.to_vec())
        .ok_or_else(|| GrainError::Export(ExportError::WriteFailed(
            "Failed to create image buffer".to_string()
        )))?;
    
    // Save to PNG
    img.save(&validated_path)
        .map_err(|e| GrainError::Export(ExportError::WriteFailed(e.to_string())))?;
    
    Ok(())
}

/// Export formats supported
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Png,
    Tiff,
    #[cfg(feature = "exr")]
    Exr,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Tiff => "tiff",
            #[cfg(feature = "exr")]
            Self::Exr => "exr",
        }
    }
    
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "tiff" | "tif" => Some(Self::Tiff),
            #[cfg(feature = "exr")]
            "exr" => Some(Self::Exr),
            _ => None,
        }
    }
}
