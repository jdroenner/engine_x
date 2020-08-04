/// A Subgraph is something with sources. TODO: find out if required
pub trait Subgraph {
    type Sources;
    fn sources(&self) -> &Self::Sources;
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
