use crate::{
    meta_raster_operator::{MetaRasterOperator, RasterCreates, RasterWants},
    operator_ext::RasterOperatorExt,
    primitives::Raster,
    source::{BoxedRasterOperatorInstance, CreateBinaryOperator, Query, RasterSource, Source},
};
use num_traits::One;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;
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
        println!("MetaAddRasterOperator: create_u8_raster_op");
        let source_a = self.sources[0].create_raster_op();
        let source_b = self.sources[0].create_raster_op();
        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U8(s1), BoxedRasterOperatorInstance::U8(s2)) => {
                AddRasterOperator::create::<u8, u8>(s1, s2, "add u8 u8".to_string()).boxed_raster()
            }
            _ => panic!(), // This case can not happen since the operator always produces an input type as output type
        }
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaAddRasterOperator: create_u16_raster_op");
        let source_a = self.sources[0].create_raster_op();
        let source_b = self.sources[0].create_raster_op();

        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U8(s1), BoxedRasterOperatorInstance::U16(s2)) => {
                AddRasterOperator::create::<u16, u8>(s2, s1, "add u8 u16".to_string())
                    .boxed_raster()
            }
            (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U8(s2)) => {
                AddRasterOperator::create::<u16, u8>(s1, s2, "add u16 u8".to_string())
                    .boxed_raster()
            }
            (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U16(s2)) => {
                AddRasterOperator::create::<u16, u16>(s1, s2, "add u16 u16".to_string())
                    .boxed_raster()
            }
            _ => panic!(), // This case can not happen since the operator always produces an input type as output type
        }
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
}
