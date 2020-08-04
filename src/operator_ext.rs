use crate::{
    add_raster_operator::AddRasterOperator,
    noop_operator::NoOpOperator,
    plus_one_operator::PlusOneOperator,
    primitives::{Raster, VectorData},
    raster_vector_operator::RasterVectorOperator,
    source::{RasterSource, Source, VectorSource},
};

/// A nice litte trait to add operations enable chaining all operators.
pub trait VectorOperatorExt {
    /// wraps any operator inside a NoOpOperator
    fn noop(self) -> NoOpOperator<Self>
    where
        Self: Sized,
    {
        NoOpOperator { source: self }
    }

    /// wraps any vector operator and adds raster data from any Operator producing Raster<T>
    fn add_raster_values<R, V, T>(self, raster: R) -> RasterVectorOperator<R, Self>
    where
        Self: Sized,
        Self: Source<Output = V>,
        V: VectorData,
        R: Source<Output = Raster<T>>,
        T: Copy + Default,
    {
        RasterVectorOperator {
            sources: (raster, self),
        }
    }

    fn boxed_vector(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

// implement the methods in OperatorExt for all Sources
impl<S> VectorOperatorExt for S where S: VectorSource {}

/// A nice litte trait to add operations enable chaining all operators.
pub trait RasterOperatorExt {
    /// wraps any operator inside a NoOpOperator
    fn noop(self) -> NoOpOperator<Self>
    where
        Self: Sized,
    {
        NoOpOperator { source: self }
    }

    fn plus_one(self) -> PlusOneOperator<Self>
    where
        Self: RasterSource + Sized,
    {
        PlusOneOperator { source: self }
    }

    fn plus_raster<R2, T2>(self, other: R2) -> AddRasterOperator<Self, R2>
    where
        R2: RasterSource<RasterType = T2>,
        Self: RasterSource + Sized,
    {
        AddRasterOperator {
            source: (self, other),
        }
    }

    fn boxed_raster(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<S> RasterOperatorExt for S where S: RasterSource {}
