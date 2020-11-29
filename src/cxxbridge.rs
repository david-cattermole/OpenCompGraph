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

        // Operation
        type Operation;
        fn get_id(self: &mut Operation) -> usize;
        fn get_op_type(self: &mut Operation) -> OperationType;
        fn get_op_type_id(self: &mut Operation) -> u8;
        fn compute(self: &mut Operation) -> Result<bool>;

        // Graph Create, Delete and Connect
        fn create_operation(id: usize, op_type: OperationType) -> Result<Box<Operation>>;

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
    op_type: ffi::OperationType,
}

impl Operation {
    pub fn get_id(&mut self) -> usize {
        println!("Operation.get_id() -> {}", self.id);
        self.id
    }

    pub fn get_op_type(&mut self) -> ffi::OperationType {
        println!("Operation.get_op_type() -> {}", self.op_type.repr);
        self.op_type
    }

    pub fn get_op_type_id(&mut self) -> u8 {
        println!("Operation.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    pub fn compute(&mut self) -> Result<bool, &'static str> {
        println!("Operation.compute()");
        Ok(true)
    }
}

pub fn create_operation(
    id: usize,
    op_type: ffi::OperationType,
) -> Result<Box<Operation>, &'static str> {
    println!("create_operation()");
    match op_type {
        ffi::OperationType::ReadImage => Ok(Box::new(Operation { id, op_type })),
        ffi::OperationType::WriteImage => Ok(Box::new(Operation { id, op_type })),
        _ => Err("Invalid OperationType"),
    }
}
