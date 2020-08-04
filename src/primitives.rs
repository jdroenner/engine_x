/// Simple mock implementation of a generic Raster
#[derive(Debug, Clone)]
pub struct Raster<T> {
    pub v: Vec<T>,
}

/// Simple mock implementation of a Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub a: f32,
    pub b: f32,
}

/// A trait for Vector Data
pub trait VectorData {}

/// a Point is a VectorData format
impl VectorData for Point {}
