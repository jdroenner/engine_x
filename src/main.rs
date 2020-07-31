use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use typetag;

/// Simple mock implementation of a generic Raster
#[derive(Debug, Clone)]
pub struct Raster<T> {
    v: Vec<T>,
}

/// Simple mock implementation of a Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    a: f32,
    b: f32,
}

/// A trait for Vector Data
pub trait VectorData {}

/// a Point is a VectorData format
impl VectorData for Point {}

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
    type VektorType;
    fn vector_query(&self, query: Query) -> Self::VektorType;
}

/// A Source is a VectorSource if it returns Vector data...
impl<S, VD> VectorSource for S
where
    S: Source<Output = VD>,
{
    type VektorType = VD;

    fn vector_query(&self, query: Query) -> Self::VektorType {
        self.query(query)
    }
}

/// A Subgraph is something with sources. TODO: find out if required
pub trait Subgraph {
    type Sources;
    fn sources(&self) -> &Self::Sources;
}

/// A GdalSource produces typed Raster<T> data
#[derive(Debug, Clone)]
struct GdalSource<T> {
    dataset: String,
    data: PhantomData<T>,
}

// It is a Source producing Raster<T> -> its a RasterSource
impl<T> Source for GdalSource<T>
where
    T: Default + Copy + std::fmt::Debug,
{
    type Output = Raster<T>;
    fn query(&self, _: Query) -> Self::Output {
        println!("GdalSource query");
        Raster {
            v: vec![T::default(); 4],
        }
    }
}

/// MyVectorSource is a mock VectorSource
#[derive(Debug, Clone)]
struct MyVectorSource<V> {
    dataset: String,
    data: PhantomData<V>,
}

/// It is a Source producing Vector data -> its a VectorSource
impl Source for MyVectorSource<Point> {
    type Output = Point;
    fn query(&self, _: Query) -> Self::Output {
        println!("MyVectorSource query");
        Point { a: 12.0, b: 13.0 }
    }
}

/// An Operator consuming a Raster and a Vector!
#[derive(Debug, Clone)]
struct RasterVectorOperator<R, V> {
    sources: (R, V),
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
// impl Subgraph for RasterVectorOperator. TODO: find out if this is needed.
impl<R, V> Subgraph for RasterVectorOperator<R, V>
where
    R: Source,
    V: Source,
{
    type Sources = (R, V);
    fn sources(&self) -> &Self::Sources {
        &self.sources
    }
}

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
struct NoOpOperator<S> {
    source: S,
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

// impl Subgraph for NoOpOperator. TODO: find out if this is needed.
impl<S> Subgraph for NoOpOperator<S>
where
    S: Source,
{
    type Sources = S;
    fn sources(&self) -> &Self::Sources {
        &self.source
    }
}

/// A nice litte trait to add operations enable chaining all operators.
trait OperatorExt: Source {
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
}

// implement the methods in OperatorExt for all Sources
impl<S> OperatorExt for S where S: Source {}

// We need trait objects so allow RasterSource objects be a Source.
impl<T> Source for Box<dyn RasterSource<RasterType = T>>
where
    T: StaticRasterType + Copy + std::fmt::Debug + 'static,
{
    type Output = Raster<T>;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().raster_query(query)
    }
}

// We need trait objects so allow VectorSource objects be a Source.
impl<V> Source for Box<dyn VectorSource<VektorType = V>>
where
    V: VectorData + std::fmt::Debug + 'static,
{
    type Output = V;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().vector_query(query)
    }
}

/// An enum for the Raster types.
#[derive(Debug, Clone)]
pub enum RasterType {
    U8,
    U16,
}

/// A trait to get the RasterType from primitive types.
pub trait StaticRasterType: Copy + Default + 'static {
    const TYPE: RasterType;
}

// u8 is alwasy RasterType U8
impl StaticRasterType for u8 {
    const TYPE: RasterType = RasterType::U8;
}

// u16 is alwasy RasterType U16
impl StaticRasterType for u16 {
    const TYPE: RasterType = RasterType::U16;
}

/// An Enum to indicate what a RasterOperator produces. TODO: find out what kind of combinations we need!
#[derive(Debug, Clone)]
enum RasterCreates {
    /// Upgrades a RasterType of a selected source: U8 -> U16
    UpgradesInput(usize),
    /// Downpgrades a RasterType of a selected source: U16 -> U8
    DowngradesInput(usize),
    // The same as a specified input
    SameAsInput(usize),
    // A concrete Type
    ConceteType(RasterType),
}

/// An Enum to indicate what a RasterOperator requires at an input.
#[derive(Debug, Clone)]
enum RasterWants {
    /// accepts any input
    Any,
    /// requries a specific Input
    ConceteType(RasterType),
    /// no input
    None,
}

/// The MetaRasterOperator is a trait for MetaOperators creating RasterOperators for processing Raster data
#[typetag::serde(tag = "type")]
trait MetaRasterOperator {
    /// The magic method to handle the mapping of the create type to a concrete implementation. More work required! TODO: macro?
    fn create_raster_op(&self) -> RasterOperatorInstance {
        println!("MetaRasterOperator: create_raster_op");
        match self.creates_type() {
            RasterCreates::ConceteType(ct) => match ct {
                RasterType::U8 => RasterOperatorInstance::U8(self.create_u8_raster_op()),
                RasterType::U16 => RasterOperatorInstance::U16(self.create_u16_raster_op()),
            },
            _ => panic!(),
        }
    }

    /// there is no way to use generics for the MetaRasterOperators in combination with serialisation -_-. We need to implement the create operator methods. TODO: Macro?
    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>>;
    /// there is no way to use generics for the MetaRasterOperators in combination with serialisation -_-. We need to implement the create operator methods. TODO: Macro?
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>>;

    /// get the type the Operator creates.
    fn creates_type(&self) -> RasterCreates;
    /// get the types the operator generates.
    fn requires_type(&self) -> &[RasterWants];

    /// get the sources of the Operator. TODO: extra trait?
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>];
    //fn raster_sources(&self) -> &[&dyn MetaRasterOperator];
}

/// The MetaGdalSource
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaGdalSource {}

#[typetag::serde]
impl MetaRasterOperator for MetaGdalSource {
    fn creates_type(&self) -> RasterCreates {
        RasterCreates::ConceteType(RasterType::U8) // TODO: need to look this up!
    }
    fn requires_type(&self) -> &[RasterWants] {
        // NO inputs so no requirements
        &[]
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaGdalSource: create_u8_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaGdalSource: create_u16_raster_op");
        Box::new(GdalSource {
            dataset: "meh".to_string(),
            data: PhantomData,
        })
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        &[] // no sources!
    }
}

/// The MetaNoopOperator
#[derive(Serialize, Deserialize)]
struct MetaNoopOperator {
    sources: Vec<Box<dyn MetaRasterOperator + 'static>>,
}

// cant use constants in the crate bcause of typetag... -_-
impl MetaNoopOperator {
    pub const REQUIRES_TYPES: [RasterWants; 1] = [RasterWants::Any];
}

// impl MetaNoopOperator for MetaRasterOperator
#[typetag::serde]
impl MetaRasterOperator for MetaNoopOperator {
    fn creates_type(&self) -> RasterCreates {
        RasterCreates::ConceteType(RasterType::U8) // this sould be same as input 1. need to handle this somewhere.
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaNoopOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaNoopOperator: create_u8_raster_op");
        Box::new(self.raster_sources()[0].create_u8_raster_op().noop())
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaNoopOperator: create_u16_raster_op");
        Box::new(self.raster_sources()[0].create_u16_raster_op().noop())
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
}

/// Enum required to hand the operator instances out from MetaRasterOperators create_raster_op "magic"
enum RasterOperatorInstance {
    U8(Box<dyn RasterSource<RasterType = u8>>),
    U16(Box<dyn RasterSource<RasterType = u16>>),
}

fn main() {
    // a gdal source
    let gdal_source: GdalSource<u8> = GdalSource {
        dataset: "meh".to_owned(),
        data: PhantomData,
    };

    // concrete raster!
    let r = gdal_source.query(Query);
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
    let vector_noop_raster_noop_combine =
        vector_source.noop().add_raster_values(gdal_source.noop());
    // add more noops
    let vector_noop_raster_noop_combine_noop_noop = vector_noop_raster_noop_combine.noop().noop();
    // will produce the concrete vector type! (all known at compile time)
    println!(
        "{:?}",
        vector_noop_raster_noop_combine_noop_noop.vector_query(Query)
    );

    // this is the magic dynamic stuff
    // create a MetaGdalSource
    let meta_gdal_source = MetaGdalSource {};
    // put it in a box
    let meta_gdal_sourcein_a_box = Box::new(meta_gdal_source) as Box<dyn MetaRasterOperator>;

    // wrap it with a noop operator
    let meta_gdal_source_noop = MetaNoopOperator {
        sources: Vec::from([meta_gdal_sourcein_a_box]),
    };

    // wrap it with a noop operator
    let meta_gdal_source_noop_noop = MetaNoopOperator {
        sources: Vec::from([Box::new(meta_gdal_source_noop) as Box<dyn MetaRasterOperator>]),
    };
    let meta_gdal_source_noop_noop_noop = MetaNoopOperator {
        sources: Vec::from([Box::new(meta_gdal_source_noop_noop) as Box<dyn MetaRasterOperator>]),
    };

    // somehow it is required to be in a box to use it...
    let meta_gdal_source_noop_noop_noop_box =
        Box::new(meta_gdal_source_noop_noop_noop) as Box<dyn MetaRasterOperator>;
    // create a RasterOperatorInstance.
    let operator_instance = meta_gdal_source_noop_noop_noop_box.create_raster_op();
    println!("meh");

    // RasterOperatorInstance is an enum. Unpack it for access to the concrete type.
    if let RasterOperatorInstance::U8(r) = operator_instance {
        // The query will produce a concrete type!
        let meh = r.raster_query(Query);
        println!("{:?}", meh);
    }

    // serialice the dynamic operator graph.
    let dynamic_serial = serde_json::to_string(&meta_gdal_source_noop_noop_noop_box).unwrap();
    println!("{:?}", dynamic_serial);

    // deserialize the json opgraph!
    let deserial: Box<dyn MetaRasterOperator> = serde_json::from_str(&dynamic_serial).unwrap();

    // create the processing oeprator
    let d_op = deserial.create_raster_op();
    // ....
    if let RasterOperatorInstance::U8(r) = d_op {
        let meh = r.raster_query(Query);
        println!("{:?}", meh);
    }
}
