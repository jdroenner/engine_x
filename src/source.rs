use crate::primitives::Raster;
use num_traits::One;
use std::ops::{Add, AddAssign};

/// The Query...
#[derive(Debug, Clone, Copy)]
pub struct Query;

/// a the most generic Source
pub trait Source {
    type Output;
    fn query(&self, query: Query) -> Self::Output;
}

/// a RasterSource is similar to a Source but it returns Raster<T>
pub trait RasterSource {
    type RasterType;
    fn raster_query(&self, query: Query) -> Raster<Self::RasterType>;
}

/// A Source is a RasterSource if it returns Rasters...
impl<S, T> RasterSource for S
where
    S: Source<Output = Raster<T>>,
{
    type RasterType = T;
    fn raster_query(&self, query: Query) -> Raster<Self::RasterType> {
        self.query(query)
    }
}

/// A VectorSource Returns some kind of Vector data
pub trait VectorSource {
    type VectorType;
    fn vector_query(&self, query: Query) -> Self::VectorType;
}

/// A Source is a VectorSource if it returns Vector data...
impl<S, VD> VectorSource for S
where
    S: Source<Output = VD>,
{
    type VectorType = VD;

    fn vector_query(&self, query: Query) -> Self::VectorType {
        self.query(query)
    }
}

impl<T> Source for Box<dyn Source<Output = T>> {
    type Output = T;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().query(query)
    }
}

// We need trait objects so allow RasterSource objects be a Source.
impl<T> Source for Box<dyn RasterSource<RasterType = T>>
where
    T: 'static,
{
    type Output = Raster<T>;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().raster_query(query)
    }
}

pub trait CreateSourceOperator<P> {
    fn create(params: P) -> Self;
}

pub trait CreateUnaryOperator<S, P> {
    fn create<T1>(source: S, params: P) -> Self;
}

pub trait CreateBinaryOperator<S1, S2, P> {
    fn create<T1, T2>(source_a: S1, source_b: S2, params: P) -> Self;
}

pub trait CreateBinaryOperator2<P> {
    fn create_from_boxes<T1, T2>(
        source_a: Box<dyn RasterSource<RasterType = T1>>,
        source_b: Box<dyn RasterSource<RasterType = T2>>,
        params: P,
    ) -> Box<dyn RasterSource<RasterType = T1>>
    where
        T1: Add + AddAssign + One + Copy + 'static,
        T2: Add + AddAssign + One + Into<T1> + Copy + 'static;
}

pub enum BoxedRasterOperatorInstance {
    U8(Box<dyn RasterSource<RasterType = u8>>),
    U16(Box<dyn RasterSource<RasterType = u16>>),
}

impl BoxedRasterOperatorInstance {
    pub fn as_u8(self) -> Option<Box<dyn RasterSource<RasterType = u8>>> {
        match self {
            BoxedRasterOperatorInstance::U8(r) => Some(r),
            _ => None,
        }
    }
    pub fn as_u16(self) -> Option<Box<dyn RasterSource<RasterType = u16>>> {
        match self {
            BoxedRasterOperatorInstance::U16(r) => Some(r),
            _ => None,
        }
    }
}
