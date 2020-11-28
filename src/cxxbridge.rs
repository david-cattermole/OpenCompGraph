#[cxx::bridge(namespace = "opencompgraph")]
pub mod ffi {
    struct SharedThing {
        z: i32,
        y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    #[repr(u8)]
    enum OperationType {
        ReadImage = 0,
        WriteImage = 1,
    }

    unsafe extern "C++" {
        include!("opencompgraph/_cpp.h");
        include!("opencompgraph.h");

        type ThingC;
        fn make_thingc(appname: &str) -> UniquePtr<ThingC>;
        fn get_name(thing: &ThingC) -> &CxxString;
        fn run_sharedthing(state: SharedThing);
    }

    extern "Rust" {
        type ThingR;
        fn print_r(r: &ThingR);

        type Operation;

        // Graph Create, Delete and Connect
        fn create_op(id: usize, op_type: OperationType) -> Result<Box<Operation>>;

        // Operation
        fn operation_compute(mut op: Box<Operation>) -> Result<bool>;
        fn operation_get_id(op: Box<Operation>) -> usize;
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

pub struct Operation {
    id: usize,
}

impl Operation {
    fn compute(&mut self, in_arg: &str) -> Result<bool, &'static str> {
        println!("Operation.compute()");
        // Call C++ Function...
        let x = ffi::make_thingc(in_arg);
        println!("this is a \"{}\".", ffi::get_name(x.as_ref().unwrap()));
        Ok(true)
    }
}

pub fn create_op(id: usize, op_type: ffi::OperationType) -> Result<Box<Operation>, &'static str> {
    println!("create_op()");
    match op_type {
        ffi::OperationType::ReadImage => Ok(Box::new(Operation { id })),
        ffi::OperationType::WriteImage => Ok(Box::new(Operation { id })),
        _ => Err("Invalid OperationType"),
    }
}

pub fn operation_compute(mut op: Box<Operation>) -> Result<bool, &'static str> {
    println!("operation_compute()");
    let x = op.compute("operation compute");
    println!("operation_compute result: ");
    x
}

pub fn operation_get_id(op: Box<Operation>) -> usize {
    println!("operation_get_id()");
    op.id
}
