use crate::{
    meta_raster_operator::{MetaRasterOperator, RasterCreates, RasterWants},
    operator_creation,
    primitives::Raster,
    source::{CreateUnaryOperator, Query, RasterSource, Source},
    CreateBoxedUnaryOperator,
};
use num_traits::One;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use typetag;

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
pub struct PlusOneOperator<S> {
    pub source: S,
}

/// It works for anything
impl<T, S> Source for PlusOneOperator<S>
where
    S: Source<Output = Raster<T>>,
    T: AddAssign + Add<T> + One + Copy + Clone + Sized,
{
    type Output = Raster<T>;
    fn query(&self, query: Query) -> Self::Output {
        println!("PlusOneOperator query");
        let mut r = self.source.query(query);
        r.v.iter_mut().for_each(|p| p.add_assign(T::one()));
        r
    }
}

impl<S> CreateUnaryOperator<S, String> for PlusOneOperator<S> {
    fn create<T1>(source: S, _params: String) -> Self {
        PlusOneOperator { source }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetaPlusOneOperator {
    pub sources: Vec<Box<dyn MetaRasterOperator>>,
}

impl MetaPlusOneOperator {
    pub const REQUIRES_TYPES: [RasterWants; 1] = [RasterWants::Any];
}

impl CreateBoxedUnaryOperator<String> for MetaPlusOneOperator {
    fn create_unary_boxed<T1>(
        source: Box<dyn RasterSource<RasterType = T1>>,
        params: String,
    ) -> Box<dyn RasterSource<RasterType = T1>>
    where
        T1: Add + AddAssign + One + Copy + 'static,
    {
        Box::new(PlusOneOperator::create::<T1>(source, params))
    }
}

#[typetag::serde]
impl MetaRasterOperator for MetaPlusOneOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type()
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaPlusOneOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        operator_creation::create_operator_unary_raster_u8::<Self>(
            self.sources[0].create_raster_op(),
        )
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        operator_creation::create_operator_unary_raster_u16::<Self>(
            self.sources[0].create_raster_op(),
        )
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
