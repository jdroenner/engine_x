use crate::{
    meta_raster_operator::{MetaRasterOperator, RasterCreates, RasterWants},
    operator_creation,
    primitives::Raster,
    source::{CreateBinaryOperator, Query, RasterSource, Source},
    CreateBoxedBinaryOperatorInplace,
};
use num_traits::One;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use typetag;

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
pub struct AddRasterOperator<S1, S2> {
    pub source: (S1, S2),
}

impl<T1, T2, S1, S2> Source for AddRasterOperator<S1, S2>
where
    S1: RasterSource<RasterType = T1>,
    S2: RasterSource<RasterType = T2>,
    T1: AddAssign + One + Copy + Clone + Sized,
    T2: AddAssign + One + Copy + Clone + Sized + Into<T1>,
{
    type Output = Raster<T1>;
    fn query(&self, query: Query) -> Self::Output {
        println!("AddRasterOperator query");
        let mut r1 = self.source.0.raster_query(query);
        let r2 = self.source.1.raster_query(query);
        r1.v.iter_mut()
            .zip(r2.v.iter())
            .for_each(|(p1, &p2)| p1.add_assign(p2.into()));
        r1
    }
}

impl<X1, X2> CreateBinaryOperator<X1, X2, String> for AddRasterOperator<X1, X2>
where
    X1: RasterSource,
    X2: RasterSource,
    X2::RasterType: Into<X1::RasterType>,
{
    fn create<T1, T2>(source_a: X1, source_b: X2, _params: String) -> Self {
        AddRasterOperator {
            source: (source_a, source_b),
        }
    }
}

impl CreateBoxedBinaryOperatorInplace<String> for MetaAddRasterOperator {
    fn create_binary_boxed<T1, T2>(
        source_a: Box<dyn RasterSource<RasterType = T1>>,
        source_b: Box<dyn RasterSource<RasterType = T2>>,
        _params: String,
    ) -> Box<dyn RasterSource<RasterType = T1>>
    where
        T1: Add + AddAssign + One + Copy + 'static,
        T2: Add + AddAssign + One + Into<T1> + Copy + 'static,
    {
        Box::new(AddRasterOperator {
            source: (source_a, source_b),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetaAddRasterOperator {
    pub sources: Vec<Box<dyn MetaRasterOperator>>,
}

impl MetaAddRasterOperator {
    pub const REQUIRES_TYPES: [RasterWants; 2] = [RasterWants::Any, RasterWants::Any];
}

#[typetag::serde]
impl MetaRasterOperator for MetaAddRasterOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type()
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaAddRasterOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        operator_creation::create_operator_binary_raster_u8_u8::<Self>(
            self.sources[0].create_raster_op(),
            self.sources[0].create_raster_op(),
        )
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        operator_creation::create_operator_binary_raster_u16_x_commutativ::<Self>(
            self.sources[0].create_raster_op(),
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
