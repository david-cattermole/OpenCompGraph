use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::ops::create_operation_box;
use crate::ops::OperationImpl;
use cxx::{CxxString, UniquePtr};

#[rustfmt::skip]
#[cxx::bridge(namespace = "opencompgraph")]
pub mod ffi {
    #[namespace = "opencompgraph::shared"]
    struct SharedThing {
        z: i32,
        y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    #[namespace = "opencompgraph::internal"]
    pub(crate) struct GraphImplShared {
        inner: Box<GraphImpl>,
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

    // ThingC
    #[namespace = "opencompgraph::cpp"]
    unsafe extern "C++" {
        include!("opencompgraph/cpp.h");
        include!("opencompgraph.h");

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

    // OperationImpl
    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        type OperationImpl;
        fn get_id(&self) -> usize;
        fn get_op_type(&self) -> OperationType;
        fn get_op_type_id(&self) -> u8;
        fn get_status(&self) -> OperationStatus;
        fn get_status_id(&self) -> u8;

        // Compute
        fn hash(&mut self) -> usize;
        fn compute(&mut self) -> OperationStatus;


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
        fn add_op(&mut self, op: Box<OperationImpl>);


    }

    #[namespace = "opencompgraph::internal"]
    extern "Rust" {
        fn create_operation_box(id: usize, op_type: OperationType) -> Box<OperationImpl>;
        fn create_graph_box() -> Box<GraphImpl>;
        fn create_graph_shared() -> GraphImplShared;
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

fn create_graph_shared() -> ffi::GraphImplShared {
    ffi::GraphImplShared {
        inner: create_graph_box(),
    }
}

#[derive(Debug)]
pub struct GraphImpl {
    ops: Vec<Box<OperationImpl>>,
}

fn create_graph_box() -> Box<GraphImpl> {
    let ops = Vec::new();
    Box::new(GraphImpl { ops })
}

impl GraphImpl {
    pub fn add_op(&mut self, op: Box<OperationImpl>) {
        self.ops.push(op);
    }

    pub fn connect(&mut self, src_op_id: usize, dst_op_id: usize) {
        println!("Connect {} to {}", src_op_id, dst_op_id);
    }
}

#[derive(Debug, Clone, Hash)]
pub struct Output {
    hash: usize,
    bbox: BoundingBox2D,
    pixel_block: Box<PixelBlock>,
    color_matrix: Matrix4,
    transform_matrix: Matrix4,
}

impl Output {
    pub fn new() -> Output {
        let hash = 0;
        let bbox = BoundingBox2D::default();
        let pixel_block = Box::new(PixelBlock::default());
        let color_matrix = Matrix4::default();
        let transform_matrix = Matrix4::default();
        Output {
            hash,
            bbox,
            pixel_block,
            color_matrix,
            transform_matrix,
        }
    }

    pub fn get_hash(&self) -> usize {
        self.hash
    }
    pub fn get_bounding_box(&self) -> BoundingBox2D {
        self.bbox
    }
    pub fn get_pixel_block(&self) -> &PixelBlock {
        &self.pixel_block
    }
    pub fn get_color_matrix(&self) -> Matrix4 {
        self.color_matrix
    }
    pub fn get_transform_matrix(&self) -> Matrix4 {
        self.transform_matrix
    }
}
