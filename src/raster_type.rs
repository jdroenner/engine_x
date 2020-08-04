use serde::{Deserialize, Serialize};

/// An enum for the Raster types.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum RasterType {
    U8,
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    F32,
    F64,
}

/// A trait to get the RasterType from primitive types.
pub trait StaticRasterType: Copy + Default + 'static {
    const TYPE: RasterType;
}

impl StaticRasterType for u8 {
    const TYPE: RasterType = RasterType::U8;
}

impl StaticRasterType for u16 {
    const TYPE: RasterType = RasterType::U16;
}

impl StaticRasterType for u32 {
    const TYPE: RasterType = RasterType::U32;
}

impl StaticRasterType for u64 {
    const TYPE: RasterType = RasterType::U64;
}

impl StaticRasterType for i16 {
    const TYPE: RasterType = RasterType::I16;
}

impl StaticRasterType for i32 {
    const TYPE: RasterType = RasterType::I32;
}

impl StaticRasterType for i64 {
    const TYPE: RasterType = RasterType::I64;
}

impl StaticRasterType for f32 {
    const TYPE: RasterType = RasterType::F32;
}

impl StaticRasterType for f64 {
    const TYPE: RasterType = RasterType::F64;
}
