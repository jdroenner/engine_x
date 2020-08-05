use crate::{
    raster_type::RasterType,
    source::{BoxedRasterOperatorInstance, RasterSource},
    BoxedVectorOperatorInstance, Point, VectorSource,
};

/// An Enum to indicate what a RasterOperator produces. TODO: find out what kind of combinations we need!
#[derive(Debug, Clone)]
pub enum RasterCreates {
    /// Upgrades a RasterType of a selected source: U8 -> U16
    UpgradesInput(usize),
    /// Downpgrades a RasterType of a selected source: U16 -> U8
    DowngradesInput(usize),
    // The same as a specified input
    SameAsInput(usize),
    // A concrete Type
    SecificType(RasterType),
}

/// An Enum to indicate what a RasterOperator requires at an input.
#[derive(Debug, Clone)]
pub enum RasterWants {
    /// accepts any input
    Any,
    /// requries a specific Input
    ConceteType(RasterType),
    /// no input
    None,
}

pub trait MetaOperator {
    /// get the sources of the Operator. TODO: extra trait?
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>];
    //fn raster_sources(&self) -> &[&dyn MetaRasterOperator];

    /// get the sources of the Operator. TODO: extra trait?
    fn vector_sources(&self) -> &[Box<dyn MetaVectorOperator>] {
        &[]
    }
}

#[typetag::serde(tag = "type")]
pub trait MetaVectorOperator: MetaOperator {
    fn creates_collection_type(&self) -> () {
        ()
    }

    fn create_vector_op(&self) -> BoxedVectorOperatorInstance {
        println!("MetaVectorOperator: create_vector_op");
        match self.creates_collection_type() {
            () => BoxedVectorOperatorInstance::Points(self.create_point_op()),
        }
    }

    fn create_point_op(&self) -> Box<dyn VectorSource<VectorType = Point>>;
}

/// The MetaRasterOperator is a trait for MetaOperators creating RasterOperators for processing Raster data
#[typetag::serde(tag = "type")]
pub trait MetaRasterOperator: MetaOperator {
    /// The magic method to handle the mapping of the create type to a concrete implementation. More work required! TODO: macro?
    fn create_raster_op(&self) -> BoxedRasterOperatorInstance {
        println!("MetaRasterOperator: create_raster_op");
        match self.creates_type() {
            RasterType::U8 => BoxedRasterOperatorInstance::U8(self.create_u8_raster_op()),
            RasterType::U16 => BoxedRasterOperatorInstance::U16(self.create_u16_raster_op()),
            RasterType::U32 => BoxedRasterOperatorInstance::U32(self.create_u32_raster_op()),
            RasterType::U64 => BoxedRasterOperatorInstance::U64(self.create_u64_raster_op()),
            RasterType::I16 => BoxedRasterOperatorInstance::I16(self.create_i16_raster_op()),
            RasterType::I32 => BoxedRasterOperatorInstance::I32(self.create_i32_raster_op()),
            RasterType::I64 => BoxedRasterOperatorInstance::I64(self.create_i64_raster_op()),
            RasterType::F32 => BoxedRasterOperatorInstance::F32(self.create_f32_raster_op()),
            RasterType::F64 => BoxedRasterOperatorInstance::F64(self.create_f64_raster_op()),
        }
    }

    // there is no way to use generics for the MetaRasterOperators in combination with serialisation -_-. We need to implement the create operator methods. TODO: Macro?
    fn create_u8_raster_op(&self) -> Box<dyn RasterSource<RasterType = u8>>;
    fn create_u16_raster_op(&self) -> Box<dyn RasterSource<RasterType = u16>>;
    fn create_u32_raster_op(&self) -> Box<dyn RasterSource<RasterType = u32>>;
    fn create_u64_raster_op(&self) -> Box<dyn RasterSource<RasterType = u64>>;
    fn create_i16_raster_op(&self) -> Box<dyn RasterSource<RasterType = i16>>;
    fn create_i32_raster_op(&self) -> Box<dyn RasterSource<RasterType = i32>>;
    fn create_i64_raster_op(&self) -> Box<dyn RasterSource<RasterType = i64>>;
    fn create_f32_raster_op(&self) -> Box<dyn RasterSource<RasterType = f32>>;
    fn create_f64_raster_op(&self) -> Box<dyn RasterSource<RasterType = f64>>;

    /// get the type the Operator creates.
    fn creates_type(&self) -> RasterType;
}

pub mod operator_creation {

    use crate::{
        BoxedRasterOperatorInstance, CreateBoxedBinaryOperatorInplace, CreateBoxedUnaryOperator,
        RasterSource,
    };

    pub fn create_operator_unary_raster_u8<O>(
        source: BoxedRasterOperatorInstance,
    ) -> Box<dyn RasterSource<RasterType = u8> + 'static>
    where
        O: CreateBoxedUnaryOperator<String>,
    {
        println!("create_operator_unary_raster_u8");
        let s = source.get_u8().expect("not u8");

        O::create_unary_boxed(s, "params".to_string())
    }

    pub fn create_operator_unary_raster_u16<O>(
        source: BoxedRasterOperatorInstance,
    ) -> Box<dyn RasterSource<RasterType = u16> + 'static>
    where
        O: CreateBoxedUnaryOperator<String>,
    {
        println!("create_operator_unary_raster_u16");
        let s = source.get_u16().expect("not u16");

        O::create_unary_boxed(s, "params".to_string())
    }

    pub fn create_operator_binary_raster_u8_u8<O>(
        source_a: BoxedRasterOperatorInstance,
        source_b: BoxedRasterOperatorInstance,
    ) -> Box<dyn RasterSource<RasterType = u8> + 'static>
    where
        O: CreateBoxedBinaryOperatorInplace<String> + 'static,
    {
        println!("create_operator_binary_raster_u8_u8");
        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U8(a), BoxedRasterOperatorInstance::U8(b)) => {
                O::create_binary_boxed(a, b, "params".to_string())
            }
            _ => panic!(),
        }
    }

    pub fn create_operator_binary_raster_u16_x_commutativ<O>(
        source_a: BoxedRasterOperatorInstance,
        source_b: BoxedRasterOperatorInstance,
    ) -> Box<dyn RasterSource<RasterType = u16> + 'static>
    where
        O: CreateBoxedBinaryOperatorInplace<String> + 'static,
    {
        println!("create_operator_binary_raster_u16_x_commutativ");

        match (source_a, source_b) {
            (BoxedRasterOperatorInstance::U16(a), BoxedRasterOperatorInstance::U8(b)) => {
                O::create_binary_boxed(a, b, "params".to_string())
            }
            (BoxedRasterOperatorInstance::U8(a), BoxedRasterOperatorInstance::U16(b)) => {
                O::create_binary_boxed(b, a, "params".to_string())
            }
            (BoxedRasterOperatorInstance::U16(a), BoxedRasterOperatorInstance::U16(b)) => {
                O::create_binary_boxed(a, b, "params".to_string())
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MetaAddRasterOperator, MetaGdalSource, MetaMyVectorSourceOperator, MetaNoopOperator,
        MetaPlusOneOperator, MetaRasterVectorOperator, Query,
    };

    #[test]
    fn mixed_graph() {
        // create a MetaGdalSource
        let meta_gdal_source = MetaGdalSource {
            raster_type: RasterType::U16,
        };

        let meta_vector_source = MetaMyVectorSourceOperator {};

        let meta_combining_operator = MetaRasterVectorOperator {
            raster_sources: vec![Box::new(meta_gdal_source)],
            vector_sources: vec![Box::new(meta_vector_source)],
        };

        let boxed_meta_combining_operator =
            Box::new(meta_combining_operator) as Box<dyn MetaVectorOperator>;

        // serialice the dynamic operator graph.
        let dynamic_serial = serde_json::to_string(&boxed_meta_combining_operator).unwrap();
        println!("{:?}", dynamic_serial);

        // deserialize the json opgraph!
        let deserial: Box<dyn MetaVectorOperator> = serde_json::from_str(&dynamic_serial).unwrap();

        // create the processing oeprator
        let d_op = deserial.create_vector_op();
        match d_op {
            BoxedVectorOperatorInstance::Points(p) => {
                let res = p.vector_query(Query);
                dbg!(res);
            }
        }
    }

    #[test]
    fn raster_graph() {
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
            sources: Vec::from([
                Box::new(meta_gdal_source_noop_noop) as Box<dyn MetaRasterOperator>
            ]),
        };

        let meta_gdal_source_noop_noop_noop_noop_plusone = MetaPlusOneOperator {
            sources: Vec::from([
                Box::new(meta_gdal_source_noop_noop_noop) as Box<dyn MetaRasterOperator>
            ]),
        };

        let meta_gdal_source_noop_noop_noop_noop_plusone_plusother = MetaAddRasterOperator {
            sources: vec![
                Box::new(meta_gdal_source_noop_noop_noop_noop_plusone)
                    as Box<dyn MetaRasterOperator>,
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
}
