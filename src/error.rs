use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyperbolicError {
    #[error("curvature must be positive, got {0}")]
    InvalidCurvature(f64),
    #[error("point outside the ball: norm {norm} >= radius {radius}")]
    OutsideBall { norm: f64, radius: f64 },
    #[error("dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },
    #[error("numerical error: {0}")]
    Numerical(String),
}
