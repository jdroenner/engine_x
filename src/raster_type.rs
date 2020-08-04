use serde::{Deserialize, Serialize};

/// An enum for the Raster types.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum RasterType {
    U8,
    U16,
}

/// A trait to get the RasterType from primitive types.
pub trait StaticRasterType: Copy + Default + 'static {
    const TYPE: RasterType;
}

// u8 is alwasy RasterType U8
impl StaticRasterType for u8 {
    const TYPE: RasterType = RasterType::U8;
}

// u16 is alwasy RasterType U16
impl StaticRasterType for u16 {
    const TYPE: RasterType = RasterType::U16;
}
