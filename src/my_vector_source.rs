use crate::{
    primitives::Point,
    source::{CreateSourceOperator, Query, Source},
    MetaOperator, MetaVectorOperator,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// MyVectorSource is a mock VectorSource
#[derive(Debug, Clone)]
pub struct MyVectorSource<V> {
    pub dataset: String,
    pub data: PhantomData<V>,
}

/// It is a Source producing Vector data -> its a VectorSource
impl Source for MyVectorSource<Point> {
    type Output = Point;
    fn query(&self, _: Query) -> Self::Output {
        println!("MyVectorSource query");
        Point { a: 12.0, b: 13.0 }
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

#[derive(Serialize, Deserialize)]
pub struct MetaMyVectorSourceOperator {}

impl MetaOperator for MetaMyVectorSourceOperator {
    fn raster_sources(&self) -> &[Box<dyn crate::MetaRasterOperator>] {
        &[]
    }
    fn vector_sources(&self) -> &[Box<dyn MetaVectorOperator>] {
        &[]
    }
}

#[typetag::serde]
impl MetaVectorOperator for MetaMyVectorSourceOperator {
    fn creates_collection_type(&self) -> () {
        ()
    }
    fn create_point_op(&self) -> Box<dyn crate::VectorSource<VectorType = crate::Point>> {
        Box::new(MyVectorSource {
            dataset: "dataset".to_string(),
            data: PhantomData,
        })
    }
}
