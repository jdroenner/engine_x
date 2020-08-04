use crate::{
    meta_raster_operator::{MetaRasterOperator, RasterCreates, RasterWants},
    primitives::Raster,
    raster_type::RasterType,
    source::{CreateSourceOperator, Query, RasterSource, Source},
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use typetag;

/// A GdalSource produces typed Raster<T> data
#[derive(Debug, Clone)]
pub struct GdalSource<T> {
    pub dataset: String,
    pub data: PhantomData<T>,
}

// It is a Source producing Raster<T> -> its a RasterSource
impl<T> Source for GdalSource<T>
where
    T: Default + Copy,
{
    type Output = Raster<T>;
    fn query(&self, _: Query) -> Self::Output {
        println!("GdalSource query");
        Raster {
            v: vec![T::default(); 4],
        }
    }
}

impl<T> CreateSourceOperator<String> for GdalSource<T> {
    fn create(params: String) -> Self {
        GdalSource {
            data: PhantomData,
            dataset: params,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaGdalSource {
    pub raster_type: RasterType,
}

#[typetag::serde]
impl MetaRasterOperator for MetaGdalSource {
    fn creates_type(&self) -> RasterCreates {
        RasterCreates::ConceteType(self.raster_type) // TODO: need to look this up!
    }
    fn requires_type(&self) -> &[RasterWants] {
        // NO inputs so no requirements
        &[]
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaGdalSource: create_u16_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        &[] // no sources!
    }
    fn create_u32_raster_op(&self) -> Box<dyn RasterSource<RasterType = u32>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_u64_raster_op(&self) -> Box<dyn RasterSource<RasterType = u64>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_i16_raster_op(&self) -> Box<dyn RasterSource<RasterType = i16>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_i32_raster_op(&self) -> Box<dyn RasterSource<RasterType = i32>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_i64_raster_op(&self) -> Box<dyn RasterSource<RasterType = i64>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_f32_raster_op(&self) -> Box<dyn RasterSource<RasterType = f32>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }

    fn create_f64_raster_op(&self) -> Box<dyn RasterSource<RasterType = f64>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
}
