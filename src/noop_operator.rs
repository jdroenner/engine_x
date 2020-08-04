use crate::{
    meta_raster_operator::{MetaRasterOperator, RasterCreates, RasterWants},
    source::{CreateUnaryOperator, Query, RasterSource, Source},
};
use serde::{Deserialize, Serialize};
use typetag;

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
pub struct NoOpOperator<S> {
    pub source: S,
}

/// It works for anything
impl<D, S> Source for NoOpOperator<S>
where
    S: Source<Output = D>,
{
    type Output = D;
    fn query(&self, query: Query) -> Self::Output {
        println!("NoOpOperator query");
        self.source.query(query)
    }
}

impl<S> CreateUnaryOperator<S, String> for NoOpOperator<S> {
    fn create<T1>(source: S, _params: String) -> Self {
        NoOpOperator { source }
    }
}

/// The MetaNoopOperator
#[derive(Serialize, Deserialize)]
pub struct MetaNoopOperator {
    pub sources: Vec<Box<dyn MetaRasterOperator>>,
}

// cant use constants in the crate bcause of typetag... -_-
impl MetaNoopOperator {
    pub const REQUIRES_TYPES: [RasterWants; 1] = [RasterWants::Any];
}

// impl MetaNoopOperator for MetaRasterOperator
#[typetag::serde]
impl MetaRasterOperator for MetaNoopOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type() // this sould be same as input 1. need to handle this somewhere.
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaNoopOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaNoopOperator: create_u8_raster_op");
        let source = self.raster_sources()[0]
            .create_raster_op()
            .get_u8()
            .expect("not u8");
        Box::new(NoOpOperator::create::<u8>(source, "noop".to_string()))
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaNoopOperator: create_u16_raster_op");
        let source = self.raster_sources()[0]
            .create_raster_op()
            .get_u16()
            .expect("not u8");
        Box::new(NoOpOperator::create::<u16>(source, "noop".to_string()))
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
    fn create_u32_raster_op(&self) -> Box<dyn RasterSource<RasterType = u32>> {
        todo!()
    }
    fn create_u64_raster_op(&self) -> Box<dyn RasterSource<RasterType = u64>> {
        todo!()
    }
    fn create_i16_raster_op(&self) -> Box<dyn RasterSource<RasterType = i16>> {
        todo!()
    }
    fn create_i32_raster_op(&self) -> Box<dyn RasterSource<RasterType = i32>> {
        todo!()
    }
    fn create_i64_raster_op(&self) -> Box<dyn RasterSource<RasterType = i64>> {
        todo!()
    }
    fn create_f32_raster_op(&self) -> Box<dyn RasterSource<RasterType = f32>> {
        todo!()
    }
    fn create_f64_raster_op(&self) -> Box<dyn RasterSource<RasterType = f64>> {
        todo!()
    }
}
