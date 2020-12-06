use crate::ops::create_operation;
use crate::ops::Operation;

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
    enum OperationType {
        // Creation / Input / Output
        ReadImage = 0,
        WriteImage = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    enum AttrState {
        Missing = 0,
        Exists = 1,
    }

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    enum AttrDataType {
        None = 0,
        UnsignedInteger8 = 1,  // C++ uint8_t
        UnsignedInteger16 = 2, // C++ uint16_t
        UnsignedInteger32 = 3, // C++ uint32_t
        UnsignedInteger64 = 4, // C++ uint64_t
        SignedInteger8 = 5,    // C++ int8_t
        SignedInteger16 = 6,   // C++ int16_t
        SignedInteger32 = 7,   // C++ int32_t
        SignedInteger64 = 8,   // C++ int64_t
        Float32 = 9,           // C++ float
        Float64 = 10,          // C++ double
        String = 11,
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

    // Operation
    extern "Rust" {
        type Operation;
        fn get_id(&self) -> usize;
        fn get_op_type(&self) -> OperationType;
        fn get_op_type_id(&self) -> u8;
        fn compute(&mut self) -> Result<bool>;

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
