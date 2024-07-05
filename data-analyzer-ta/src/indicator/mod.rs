// src/indicator/mod.rs
pub mod exponentialMovingAverage;

// Re-export the ExponentialMovingAverage so it can be accessed directly from `indicator`
pub use exponentialMovingAverage::ExponentialMovingAverage;
