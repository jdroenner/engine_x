use crate::{
    raster_type::RasterType,
    source::{BoxedRasterOperatorInstance, RasterSource},
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
    ConceteType(RasterType),
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
    /// get the types the operator generates.
    fn requires_type(&self) -> &[RasterWants];

    /// get the sources of the Operator. TODO: extra trait?
    fn raster_sources(&self) -> &[Box<dyn MetaRasterOperator>];
    //fn raster_sources(&self) -> &[&dyn MetaRasterOperator];

    /// get the types the operator generates.
    fn requires_collection(&self) -> &[RasterWants] {
        &[]
    }

    /// get the sources of the Operator. TODO: extra trait?
    fn vector_sources(&self) -> &[Box<dyn MetaRasterOperator>] {
        &[]
    }
}

#[typetag::serde(tag = "type")]
pub trait MetaVectorOperator: MetaOperator {}

/// The MetaRasterOperator is a trait for MetaOperators creating RasterOperators for processing Raster data
#[typetag::serde(tag = "type")]
pub trait MetaRasterOperator: MetaOperator {
    /// The magic method to handle the mapping of the create type to a concrete implementation. More work required! TODO: macro?
    fn create_raster_op(&self) -> BoxedRasterOperatorInstance {
        println!("MetaRasterOperator: create_raster_op");
        match self.creates_type() {
            RasterCreates::ConceteType(ct) => match ct {
                RasterType::U8 => BoxedRasterOperatorInstance::U8(self.create_u8_raster_op()),
                RasterType::U16 => BoxedRasterOperatorInstance::U16(self.create_u16_raster_op()),
                RasterType::U32 => BoxedRasterOperatorInstance::U32(self.create_u32_raster_op()),
                RasterType::U64 => BoxedRasterOperatorInstance::U64(self.create_u64_raster_op()),
                RasterType::I16 => BoxedRasterOperatorInstance::I16(self.create_i16_raster_op()),
                RasterType::I32 => BoxedRasterOperatorInstance::I32(self.create_i32_raster_op()),
                RasterType::I64 => BoxedRasterOperatorInstance::I64(self.create_i64_raster_op()),
                RasterType::F32 => BoxedRasterOperatorInstance::F32(self.create_f32_raster_op()),
                RasterType::F64 => BoxedRasterOperatorInstance::F64(self.create_f64_raster_op()),
            },
            _ => panic!(),
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
    fn creates_type(&self) -> RasterCreates;
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
