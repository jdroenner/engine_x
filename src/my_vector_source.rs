use crate::{
    primitives::Point,
    source::{CreateSourceOperator, Query, Source},
};
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
