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

pub trait CreateBoxedUnaryOperator<P> {
    fn create_unary_boxed<T1>(
        source: Box<dyn RasterSource<RasterType = T1>>,
        params: P,
    ) -> Box<dyn RasterSource<RasterType = T1>>
    where
        T1: Add + AddAssign + One + Copy + 'static;
}

pub trait CreateBoxedBinaryOperatorInplace<P> {
    fn create_binary_boxed<T1, T2>(
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
    U32(Box<dyn RasterSource<RasterType = u32>>),
    U64(Box<dyn RasterSource<RasterType = u64>>),
    I16(Box<dyn RasterSource<RasterType = i16>>),
    I32(Box<dyn RasterSource<RasterType = i32>>),
    I64(Box<dyn RasterSource<RasterType = i64>>),
    F32(Box<dyn RasterSource<RasterType = f32>>),
    F64(Box<dyn RasterSource<RasterType = f64>>),
}

impl BoxedRasterOperatorInstance {
    pub fn get_u8(self) -> Option<Box<dyn RasterSource<RasterType = u8>>> {
        match self {
            BoxedRasterOperatorInstance::U8(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_u16(self) -> Option<Box<dyn RasterSource<RasterType = u16>>> {
        match self {
            BoxedRasterOperatorInstance::U16(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_u32(self) -> Option<Box<dyn RasterSource<RasterType = u32>>> {
        match self {
            BoxedRasterOperatorInstance::U32(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_u64(self) -> Option<Box<dyn RasterSource<RasterType = u64>>> {
        match self {
            BoxedRasterOperatorInstance::U64(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_i16(self) -> Option<Box<dyn RasterSource<RasterType = i16>>> {
        match self {
            BoxedRasterOperatorInstance::I16(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_i32(self) -> Option<Box<dyn RasterSource<RasterType = i32>>> {
        match self {
            BoxedRasterOperatorInstance::I32(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_i64(self) -> Option<Box<dyn RasterSource<RasterType = i64>>> {
        match self {
            BoxedRasterOperatorInstance::I64(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_f32(self) -> Option<Box<dyn RasterSource<RasterType = f32>>> {
        match self {
            BoxedRasterOperatorInstance::F32(r) => Some(r),
            _ => None,
        }
    }
    pub fn get_f64(self) -> Option<Box<dyn RasterSource<RasterType = f64>>> {
        match self {
            BoxedRasterOperatorInstance::F64(r) => Some(r),
            _ => None,
        }
    }
}
