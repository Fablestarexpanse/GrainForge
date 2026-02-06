use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrainError {
    #[error("GPU initialization failed: {0}")]
    GpuInit(String), // wgpu::Error isn't easily clonable/displayable sometimes, treating as string for now implies generic
    
    #[error("Invalid parameter '{name}': {reason}")]
    InvalidParameter { name: String, reason: String },
    
    #[error("Export failed: {0}")]
    Export(#[from] ExportError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("Invalid path")]
    InvalidPath,
    
    #[error("Path not allowed (must be inside allowed directories)")]
    PathNotAllowed,
    
    #[error("Invalid extension (allowed: png, tif, exr)")]
    InvalidExtension,

    #[error("Write failed: {0}")]
    WriteFailed(String),
}
