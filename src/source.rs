use crate::{primitives::Raster, Point};
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

impl<V> Source for Box<dyn VectorSource<VectorType = V>>
where
    V: 'static,
{
    type Output = V;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().vector_query(query)
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

pub enum BoxedVectorOperatorInstance {
    Points(Box<dyn VectorSource<VectorType = Point>>),
}

#[cfg(test)]
mod tests {
    use crate::{
        GdalSource, MyVectorSource, Point, Query, RasterOperatorExt, Source, VectorOperatorExt,
        VectorSource,
    };
    use std::marker::PhantomData;

    #[test]
    fn complex() {
        // a gdal source
        let gdal_source: GdalSource<u16> = GdalSource {
            dataset: "meh".to_owned(),
            data: PhantomData,
        };

        // concrete raster!
        let r = gdal_source.query(Query);
        println!("{:?}", r);

        let raster_plus_one = gdal_source.plus_one();
        let r = raster_plus_one.query(Query);
        println!("{:?}", r);

        let other_gdal_source: GdalSource<u8> = GdalSource {
            dataset: "meh".to_owned(),
            data: PhantomData,
        };

        let raster_plusone_plus_other = raster_plus_one.plus_raster(other_gdal_source);
        let r = raster_plusone_plus_other.query(Query);
        println!("{:?}", r);

        // a vector source
        let vector_source: MyVectorSource<Point> = MyVectorSource {
            dataset: "vec".to_owned(),
            data: PhantomData,
        };

        // concrete vector!
        let v = vector_source.query(Query);
        println!("{:?}", v);

        // take the vector_source, add a noop, combine the result with the raster_source wrapped in a noop
        let vector_noop_raster_noop_combine = vector_source
            .noop()
            .add_raster_values(RasterOperatorExt::noop(raster_plusone_plus_other));
        // add more noops
        let vector_noop_raster_noop_combine_noop_noop =
            vector_noop_raster_noop_combine.noop().noop();
        // will produce the concrete vector type! (all known at compile time)
        println!(
            "{:?}",
            vector_noop_raster_noop_combine_noop_noop.vector_query(Query)
        );
    }
}
