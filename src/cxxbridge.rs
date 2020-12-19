#[allow(unused_imports)]
use cxx::{CxxString, UniquePtr};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::data::Identifier;
use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::graph::create_graph;
use crate::graph::GraphImpl;
use crate::ops::create_operation;
use crate::ops::OperationImpl;
use crate::stream::StreamDataImpl;

#[rustfmt::skip]
#[cxx::bridge(namespace = "opencompgraph")]
pub mod ffi {
    #[namespace = "opencompgraph::shared"]
    struct SharedThing {
        z: i32,
        y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    #[derive(Debug)]
    #[namespace = "opencompgraph::internal"]
    pub(crate) struct GraphImplShared {
        inner: Box<GraphImpl>,
    }

    #[derive(Debug)]
    #[namespace = "opencompgraph::internal"]
    pub(crate) struct OperationImplShared {
        inner: Box<OperationImpl>,
    }

    #[derive(Debug)]
    #[namespace = "opencompgraph::internal"]
    pub(crate) struct StreamDataImplShared {
        inner: Box<StreamDataImpl>,
    }
    impl Vec<StreamDataImplShared> {}

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "opencompgraph"]
    pub(crate) enum ExecuteStatus {
        Error = 0,
        Success = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "opencompgraph"]
    pub(crate) enum OperationType {
        // Creation / Input / Output
        Null = 0,
        ReadImage = 1,
        WriteImage = 2,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "opencompgraph"]
    pub(crate) enum OperationStatus {
        Error = 0,
        Warning = 1,
        Valid = 2,
        Uninitialized = 3,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "opencompgraph"]
    pub(crate) enum AttrState {
        Missing = 0,
        Exists = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    #[namespace = "opencompgraph"]
    pub(crate) enum StreamDataState {
        Invalid = 0,
        Valid = 1,
    }

    // ThingC
    #[namespace = "opencompgraph::cpp"]
    unsafe extern "C++" {
        include!("rust/cxx.h");
        include!("opencompgraph/cpp.h");

        type ThingC;
        fn make_thingc(appname: &str) -> UniquePtr<ThingC>;
        fn get_name(thing: &ThingC) -> &CxxString;
        fn run_sharedthing(state: SharedThing);
    }

    // ThingR
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type ThingR;
        fn print_r(r: &ThingR);
    }

    // PixelBlock
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type PixelBlock;
    }

    // BoundingBox2D
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type BoundingBox2D;
    }

    // Matrix4
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type Matrix4;
    }

    // StreamDataImpl
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type StreamDataImpl;
        fn get_hash(&self) -> usize;
        fn get_pixel_block(&self) -> &Box<PixelBlock>;
        fn get_bounding_box(&self) -> &Box<BoundingBox2D>;
        fn get_color_matrix(&self) -> &Box<Matrix4>;
        fn get_transform_matrix(&self) -> &Box<Matrix4>;
    }

    // OperationImpl
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type OperationImpl;
        fn get_id(&self) -> u64;
        fn get_op_type(&self) -> OperationType;
        fn get_op_type_id(&self) -> u8;
        fn get_status(&self) -> OperationStatus;
        fn get_status_id(&self) -> u8;

        // Compute
        fn hash(&mut self, inputs: &Vec<StreamDataImplShared>) -> usize;
        fn compute(&mut self, inputs: &Vec<StreamDataImplShared>, output: &mut StreamDataImplShared) -> OperationStatus;


        // AttrBlock
        fn attr_exists(&self, name: &str) -> AttrState;
        unsafe fn get_attr_string<'a, 'b>(&'b self, name: &'a str) -> &'b str;
        #[cxx_name = "set_attr"]
        fn set_attr_string(&mut self, name: &str, value: &str);
    }

    // Graph
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type GraphImpl;
        fn add_op(&mut self, op_box: Box<OperationImpl>) -> usize;
        fn connect(&mut self, src_index: usize, dst_index: usize, input_num: u8);
        fn execute(&mut self, start_index: usize) -> ExecuteStatus;


    }

    // Struct Creation
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        #[cxx_name = "create_operation_box"]
        fn create_operation_box(op_type: OperationType) -> Box<OperationImpl>;
        #[cxx_name = "create_operation_box"]
        fn create_operation_box_with_name(op_type: OperationType, name: &str) -> Box<OperationImpl>;
        #[cxx_name = "create_operation_box"]
        fn create_operation_box_with_id(op_type: OperationType, id: u64) -> Box<OperationImpl>;

        #[cxx_name = "create_operation_shared"]
        fn create_operation_shared(op_type: OperationType) -> OperationImplShared;
        #[cxx_name = "create_operation_shared"]
        fn create_operation_shared_with_name(op_type: OperationType, name: &str) -> OperationImplShared;
        #[cxx_name = "create_operation_shared"]
        fn create_operation_shared_with_id(op_type: OperationType, id: u64) -> OperationImplShared;

        fn create_graph_box() -> Box<GraphImpl>;
        fn create_graph_shared() -> GraphImplShared;

        fn create_stream_data_box() -> Box<StreamDataImpl>;
        fn create_stream_data_shared() -> StreamDataImplShared;
    }
}

pub struct ThingR(usize);

fn print_r(r: &ThingR) {
    println!("called back with r={}", r.0);
}

#[allow(dead_code)]
fn my_test() {
    let x = ffi::make_thingc("demo of cxx::bridge");
    println!("this is a \"{}\"", ffi::get_name(x.as_ref().unwrap()));
    ffi::run_sharedthing(ffi::SharedThing {
        z: 222,
        y: Box::new(ThingR(333)),
        x,
    });
}

pub fn create_operation_box(op_type: ffi::OperationType) -> Box<OperationImpl> {
    let id = generate_random_id();
    create_operation_box_with_id(op_type, id)
}

pub fn create_operation_box_with_name(
    op_type: ffi::OperationType,
    name: &str,
) -> Box<OperationImpl> {
    let id = generate_id_from_name(name);
    create_operation_box_with_id(op_type, id)
}

pub fn create_operation_box_with_id(
    op_type: ffi::OperationType,
    id: Identifier,
) -> Box<OperationImpl> {
    println!("create_operation_box(op_type={:?}, id={:?})", op_type, id);
    Box::new(create_operation(op_type, id))
}

fn create_graph_shared() -> ffi::GraphImplShared {
    println!("create_graph_shared()");
    ffi::GraphImplShared {
        inner: create_graph_box(),
    }
}

fn create_operation_shared(op_type: ffi::OperationType) -> ffi::OperationImplShared {
    let id = generate_random_id();
    create_operation_shared_with_id(op_type, id)
}

fn create_operation_shared_with_name(
    op_type: ffi::OperationType,
    name: &str,
) -> ffi::OperationImplShared {
    let id = generate_id_from_name(name);
    create_operation_shared_with_id(op_type, id)
}

fn create_operation_shared_with_id(
    op_type: ffi::OperationType,
    id: Identifier,
) -> ffi::OperationImplShared {
    println!(
        "create_operation_shared(op_type={:?}, id={:?})",
        op_type, id,
    );
    ffi::OperationImplShared {
        inner: create_operation_box_with_id(op_type, id),
    }
}

fn create_graph_box() -> Box<GraphImpl> {
    println!("create_graph_box()");
    Box::new(create_graph())
}

pub fn create_stream_data_shared() -> ffi::StreamDataImplShared {
    println!("create_stream_data_shared()");
    ffi::StreamDataImplShared {
        inner: create_stream_data_box(),
    }
}

pub fn create_stream_data_box() -> Box<StreamDataImpl> {
    println!("create_stream_data_box()");
    Box::new(StreamDataImpl::new())
}

fn calculate_hash<T: Hash>(t: &T) -> Identifier {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn generate_id_from_name(name: &str) -> Identifier {
    calculate_hash::<&str>(&name)
}

fn generate_random_id() -> Identifier {
    // Create small, cheap to initialize and fast RNG with a random seed.
    // The randomness is supplied by the operating system.
    let mut rng = SmallRng::from_entropy();
    rng.gen()
}
