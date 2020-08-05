use crate::{
    primitives::{Raster, VectorData},
    source::{Query, Source},
    MetaOperator, MetaRasterOperator, MetaVectorOperator,
};

use serde::{Deserialize, Serialize};

/// An Operator consuming a Raster and a Vector!
#[derive(Debug, Clone)]
pub struct RasterVectorOperator<R, V> {
    pub sources: (R, V),
}

// It is a Source producing Vector data. So it is a VectorSource
impl<RD, VD, R, V> Source for RasterVectorOperator<R, V>
where
    R: Source<Output = Raster<RD>>,
    V: Source<Output = VD>,
    VD: VectorData,
{
    type Output = VD;
    fn query(&self, query: Query) -> Self::Output {
        println!("RasterVectorOperator query");
        self.sources.0.query(query.clone());
        self.sources.1.query(query)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MetaRasterVectorOperator {
    pub raster_sources: Vec<Box<dyn MetaRasterOperator>>,
    pub vector_sources: Vec<Box<dyn MetaVectorOperator>>,
}

impl MetaOperator for MetaRasterVectorOperator {
    fn raster_sources(&self) -> &[Box<dyn crate::MetaRasterOperator>] {
        &self.raster_sources
    }
    fn vector_sources(&self) -> &[Box<dyn MetaVectorOperator>] {
        &self.vector_sources
    }
}

#[typetag::serde]
impl MetaVectorOperator for MetaRasterVectorOperator {
    fn creates_collection_type(&self) -> () {
        ()
    }
    fn create_point_op(&self) -> Box<dyn crate::VectorSource<VectorType = crate::Point>> {
        let raster_source = self.raster_sources()[0].create_raster_op();
        let vector_source = self.vector_sources()[0].create_vector_op();
        match (raster_source, vector_source) {
            (
                crate::BoxedRasterOperatorInstance::U8(r),
                crate::BoxedVectorOperatorInstance::Points(p),
            ) => Box::new(RasterVectorOperator { sources: (r, p) }),
            (
                crate::BoxedRasterOperatorInstance::U16(r),
                crate::BoxedVectorOperatorInstance::Points(p),
            ) => Box::new(RasterVectorOperator { sources: (r, p) }),
            _ => panic!(),
        }
    }
}
