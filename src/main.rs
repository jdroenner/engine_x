use std::marker::PhantomData;

fn main() {
    use engine_x::*;

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
