use crate::data::{BoundingBox2D, Matrix4, PixelBlock};
use crate::ops::create_operation;
use crate::ops::Operation;
use cxx::{CxxString, UniquePtr};

#[rustfmt::skip]
#[cxx::bridge(namespace = "opencompgraph")]
pub mod ffi {
    struct SharedThing {
        z: i32,
        y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    pub(crate) enum OperationType {
        // Creation / Input / Output
        ReadImage = 0,
        WriteImage = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    pub(crate) enum ComputeStatus {
        Failure = 0,
        Success = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    pub(crate) enum AttrState {
        Missing = 0,
        Exists = 1,
    }


    // ThingC
    unsafe extern "C++" {
        include!("opencompgraph/_cpp.h");
        include!("opencompgraph.h");

        type ThingC;
        fn make_thingc(appname: &str) -> UniquePtr<ThingC>;
        fn get_name(thing: &ThingC) -> &CxxString;
        fn run_sharedthing(state: SharedThing);
    }

    // ThingR
    extern "Rust" {
        type ThingR;
        fn print_r(r: &ThingR);
    }

    // PixelBlock
    extern "Rust" {
        type PixelBlock;
    }

    // BoundingBox2D
    extern "Rust" {
        type BoundingBox2D;
    }

    // Matrix4
    extern "Rust" {
        type Matrix4;
    }

    // Operation
    extern "Rust" {
        type Operation;
        fn get_id(&self) -> usize;
        fn get_op_type(&self) -> OperationType;
        fn get_op_type_id(&self) -> u8;

        // Compute
        fn hash(&mut self) -> usize;
        fn compute(&mut self) -> ComputeStatus;


        // AttrBlock
        fn attr_exists(&self, name: &str) -> AttrState;
        unsafe fn get_attr_string<'a, 'b>(&'b self, name: &'a str) -> &'b str;
        #[cxx_name = "set_attr"]
        fn set_attr_string(&mut self, name: &str, value: &str);
    }

    extern "Rust" {
        fn create_operation(id: usize, op_type: OperationType) -> Box<Operation>;
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
        let bbox = BoundingBox2D { data: true };
        let pixel_block = Box::new(PixelBlock { data: true });
        let color_matrix = Matrix4 { data: true };
        let transform_matrix = Matrix4 { data: true };
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
