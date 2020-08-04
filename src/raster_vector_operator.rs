use crate::{
    primitives::{Raster, VectorData},
    source::{Query, Source},
};

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
