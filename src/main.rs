use num_traits::One;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    marker::PhantomData,
    ops::{Add, AddAssign},
};
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
    T: Default + Copy,
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

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
struct PlusOneOperator<S> {
    source: S,
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

// impl Subgraph for NoOpOperator. TODO: find out if this is needed.
impl<S> Subgraph for PlusOneOperator<S>
where
    S: Source,
{
    type Sources = S;
    fn sources(&self) -> &Self::Sources {
        &self.source
    }
}

/// The NoOp Operator does nothing. It wraps any Operator.
#[derive(Debug, Clone)]
struct AddRasterOperator<S1, S2> {
    source: (S1, S2),
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
            .for_each(|(p1, &p2)| p1.add_assign(p2.try_into().unwrap()));
        r1
    }
}

// impl Subgraph for NoOpOperator. TODO: find out if this is needed.
impl<S1, S2> Subgraph for AddRasterOperator<S1, S2>
where
    S1: Source,
    S2: Source,
{
    type Sources = (S1, S2);
    fn sources(&self) -> &Self::Sources {
        &self.source
    }
}

/// A nice litte trait to add operations enable chaining all operators.
trait VectorOperatorExt {
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
trait RasterOperatorExt {
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

impl<T> Source for Box<dyn Source<Output = T>> {
    type Output = T;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().query(query)
    }
}

trait CreateSourceOperator<P> {
    fn create(params: P) -> Self;
}

impl<T> CreateSourceOperator<String> for GdalSource<T> {
    fn create(params: String) -> Self {
        GdalSource {
            data: PhantomData,
            dataset: params,
        }
    }
}

impl<T> CreateSourceOperator<String> for MyVectorSource<T> {
    fn create(params: String) -> Self {
        MyVectorSource {
            data: PhantomData,
            dataset: params,
        }
    }
}

trait CreateUnaryOperator<S> {
    fn create(source: S) -> Self;
}

impl<S> CreateUnaryOperator<S> for NoOpOperator<S> {
    fn create(source: S) -> Self {
        NoOpOperator { source }
    }
}

impl<S> CreateUnaryOperator<S> for PlusOneOperator<S> {
    fn create(source: S) -> Self {
        PlusOneOperator { source }
    }
}

trait CreateBinaryOperator<S1, S2> {
    fn create(source_a: S1, source_b: S2) -> Self;
}

impl<S1, S2> CreateBinaryOperator<S1, S2> for RasterVectorOperator<S1, S2>
where
    S1: VectorSource,
    S2: RasterSource,
{
    fn create(source_a: S1, source_b: S2) -> Self {
        RasterVectorOperator {
            sources: (source_a, source_b),
        }
    }
}

impl<S1, S2> CreateBinaryOperator<S1, S2> for AddRasterOperator<S1, S2>
where
    S1: RasterSource,
    S2: RasterSource,
{
    fn create(source_a: S1, source_b: S2) -> Self {
        AddRasterOperator {
            source: (source_a, source_b),
        }
    }
}

/**
// We need trait objects so allow RasterSource objects be a Source.
impl<T> Source for Box<dyn RasterSource<RasterType = T>>
where
    T: Copy + 'static,
{
    type Output = Raster<T>;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().raster_query(query)
    }
}
*/

// We need trait objects so allow VectorSource objects be a Source.
/*impl<V> Source for Box<dyn VectorSource<VectorType = V>>
where
    V: VectorData + 'static,
{
    type Output = V;
    fn query(&self, query: Query) -> Self::Output {
        self.as_ref().vector_query(query)
    }
}
*/

/// An enum for the Raster types.
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
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
    fn create_raster_op(&self) -> BoxedRasterOperatorInstance {
        println!("MetaRasterOperator: create_raster_op");
        match self.creates_type() {
            RasterCreates::ConceteType(ct) => match ct {
                RasterType::U8 => BoxedRasterOperatorInstance::U8(self.create_u8_raster_op()),
                RasterType::U16 => BoxedRasterOperatorInstance::U16(self.create_u16_raster_op()),
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
struct MetaGdalSource {
    raster_type: RasterType,
}

#[typetag::serde]
impl MetaRasterOperator for MetaGdalSource {
    fn creates_type(&self) -> RasterCreates {
        RasterCreates::ConceteType(self.raster_type) // TODO: need to look this up!
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
    sources: Vec<Box<dyn MetaRasterOperator>>,
}

// cant use constants in the crate bcause of typetag... -_-
impl MetaNoopOperator {
    pub const REQUIRES_TYPES: [RasterWants; 1] = [RasterWants::Any];
}

// impl MetaNoopOperator for MetaRasterOperator
#[typetag::serde]
impl MetaRasterOperator for MetaNoopOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type() // this sould be same as input 1. need to handle this somewhere.
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaNoopOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaNoopOperator: create_u8_raster_op");
        Box::new(RasterOperatorExt::noop(
            self.raster_sources()[0]
                .create_raster_op()
                .as_u8()
                .expect("not u8"),
        ))
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaNoopOperator: create_u16_raster_op");
        Box::new(RasterOperatorExt::noop(
            self.raster_sources()[0]
                .create_raster_op()
                .as_u16()
                .expect("not u8"),
        ))
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
}

#[derive(Serialize, Deserialize)]
struct MetaPlusOneOperator {
    sources: Vec<Box<dyn MetaRasterOperator>>,
}

#[typetag::serde]
impl MetaRasterOperator for MetaPlusOneOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type()
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaNoopOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaPlusOneOperator: create_u8_raster_op");
        let s = self.raster_sources()[0]
            .create_raster_op()
            .as_u8()
            .expect("not u8");
        s.plus_one().boxed_raster()
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaPlusOneOperator: create_u16_raster_op");
        let s = self.raster_sources()[0]
            .create_raster_op()
            .as_u16()
            .expect("not u16");

        s.plus_one().boxed_raster()
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
}

#[derive(Serialize, Deserialize)]
struct MetaAddRasterOperator {
    sources: Vec<Box<dyn MetaRasterOperator>>,
}

/*
trait BinaryFunc {
    fn create<'a, T1, T2>(
        s1: Box<dyn RasterSource<RasterType = T1>>,
        s2: Box<dyn RasterSource<RasterType = T2>>,
    ) -> Box<dyn 'a + RasterSource<RasterType = T1>>
    where
        T1: 'a + Add + AddAssign + Copy + Clone + One,
        T2: 'a + Add + AddAssign + Copy + Clone + One + Into<T1>;
}

impl BinaryFunc for MetaAddRasterOperator {
    fn create<'a, T1, T2>(
        s1: Box<dyn RasterSource<RasterType = T1>>,
        s2: Box<dyn RasterSource<RasterType = T2>>,
    ) -> Box<dyn 'a + RasterSource<RasterType = T1>>
    where
        T1: 'a + Add + AddAssign + Copy + Clone + One,
        T2: 'a + Add + AddAssign + Copy + Clone + One + Into<T1>,
    {
        Box::new(s1.plus_raster(s2))
    }
}

fn binary_func<R, F: BinaryFunc>(
    a: BoxedRasterOperatorInstance,
    b: BoxedRasterOperatorInstance,
) -> Box<dyn RasterSource<RasterType = R>>
where
{
    match (a, b) {
        (BoxedRasterOperatorInstance::U8(s1), BoxedRasterOperatorInstance::U8(s2)) => {
            let c = F::create::<u8, u8>(s1, s2);
            c
            //Box::new(::create(s1, s2))
        }
        (BoxedRasterOperatorInstance::U8(s1), BoxedRasterOperatorInstance::U16(s2)) => {
            unimplemented!()
        }
        (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U8(s2)) => {
            unimplemented!()
        }
        (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U16(s2)) => {
            unimplemented!()
        }
        _ => panic!(),
    }
}
*/

#[typetag::serde]
impl MetaRasterOperator for MetaAddRasterOperator {
    fn creates_type(&self) -> RasterCreates {
        self.sources[0].creates_type()
    }
    fn requires_type(&self) -> &[RasterWants] {
        &MetaNoopOperator::REQUIRES_TYPES
    }

    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>> {
        println!("MetaAddRasterOperator: create_u8_raster_op");
        let source_a = self.sources[0].create_raster_op();
        let source_b = self.sources[0].create_raster_op();
        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U8(s1), BoxedRasterOperatorInstance::U8(s2)) => {
                AddRasterOperator::create(s1, s2).boxed_raster()
            }
            _ => panic!(),
        }
    }
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>> {
        println!("MetaAddRasterOperator: create_u16_raster_op");
        let source_a = self.sources[0].create_raster_op();
        let source_b = self.sources[0].create_raster_op();

        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U8(s2)) => {
                AddRasterOperator::create(s1, s2).boxed_raster()
            }
            (BoxedRasterOperatorInstance::U16(s1), BoxedRasterOperatorInstance::U16(s2)) => {
                AddRasterOperator::create(s1, s2).boxed_raster()
            }
            _ => panic!(),
        }
    }
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        self.sources.as_slice()
    }
}

/// Enum required to hand the operator instances out from MetaRasterOperators create_raster_op "magic"
enum BoxedRasterOperatorInstance {
    U8(Box<dyn RasterSource<RasterType = u8>>),
    U16(Box<dyn RasterSource<RasterType = u16>>),
}

impl BoxedRasterOperatorInstance {
    fn as_u8(self) -> Option<Box<dyn RasterSource<RasterType = u8>>> {
        match self {
            BoxedRasterOperatorInstance::U8(r) => Some(r),
            _ => None,
        }
    }
    fn as_u16(self) -> Option<Box<dyn RasterSource<RasterType = u16>>> {
        match self {
            BoxedRasterOperatorInstance::U16(r) => Some(r),
            _ => None,
        }
    }
}

trait AutoGenerateDynUnaryCombos: MetaRasterOperator {
    fn auto_create_u8_raster_op<'a, O>(&self) -> Box<dyn RasterSource<RasterType = u8> + 'a>
    where
        O: 'a
            + RasterSource<RasterType = u8>
            + CreateUnaryOperator<Box<dyn RasterSource<RasterType = u8>>>,
    {
        println!("MetaPlusOneOperator: create_u8_raster_op");
        let s = self.raster_sources()[0]
            .create_raster_op()
            .as_u8()
            .expect("not u8");
        O::create(s).boxed_raster()
    }

    fn auto_create_u16_raster_op<'a, O>(&self) -> Box<dyn RasterSource<RasterType = u16> + 'a>
    where
        O: 'a
            + RasterSource<RasterType = u16>
            + CreateUnaryOperator<Box<dyn RasterSource<RasterType = u16>>>,
    {
        println!("MetaPlusOneOperator: create_u16_raster_op");
        let s = self.raster_sources()[0]
            .create_raster_op()
            .as_u16()
            .expect("not u16");

        O::create(s).boxed_raster()
    }
}

impl AutoGenerateDynUnaryCombos for MetaPlusOneOperator {}

fn main() {
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
    let vector_noop_raster_noop_combine_noop_noop = vector_noop_raster_noop_combine.noop().noop();
    // will produce the concrete vector type! (all known at compile time)
    println!(
        "{:?}",
        vector_noop_raster_noop_combine_noop_noop.vector_query(Query)
    );

    // this is the magic dynamic stuff
    // create a MetaGdalSource
    let meta_gdal_source = MetaGdalSource {
        raster_type: RasterType::U16,
    };
    // put it in a box
    let meta_gdal_sourcein_a_box = Box::new(meta_gdal_source) as Box<dyn MetaRasterOperator>;

    let other_meta_gdal_source = Box::new(MetaGdalSource {
        raster_type: RasterType::U8,
    }) as Box<dyn MetaRasterOperator>;

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

    let meta_gdal_source_noop_noop_noop_noop_plusone = MetaPlusOneOperator {
        sources: Vec::from([
            Box::new(meta_gdal_source_noop_noop_noop) as Box<dyn MetaRasterOperator>
        ]),
    };

    let meta_gdal_source_noop_noop_noop_noop_plusone_plusother = MetaAddRasterOperator {
        sources: vec![
            Box::new(meta_gdal_source_noop_noop_noop_noop_plusone) as Box<dyn MetaRasterOperator>,
            other_meta_gdal_source as Box<dyn MetaRasterOperator>,
        ],
    };

    // somehow it is required to be in a box to use it...
    let meta_gdal_source_noop_noop_noop_box =
        Box::new(meta_gdal_source_noop_noop_noop_noop_plusone_plusother)
            as Box<dyn MetaRasterOperator>;
    // create a BoxedRasterOperatorInstance.
    let operator_instance = meta_gdal_source_noop_noop_noop_box.create_raster_op();
    println!("meh");

    // BoxedRasterOperatorInstance is an enum. Unpack it for access to the concrete type.
    if let BoxedRasterOperatorInstance::U8(r) = operator_instance {
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
    if let BoxedRasterOperatorInstance::U16(r) = d_op {
        let meh = r.raster_query(Query);
        println!("{:?}", meh);
    }
}
