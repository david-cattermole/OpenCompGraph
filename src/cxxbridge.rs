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
        ReadImage = 0,
        WriteImage = 1,
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
        fn get_id(&mut self) -> usize;
        fn get_op_type(&mut self) -> OperationType;
        fn get_op_type_id(&mut self) -> u8;
        fn compute(&mut self) -> Result<bool>;
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

pub trait Compute {
    fn get_id(&self) -> usize;
    fn compute(&mut self) -> Result<bool, &'static str>;
}

pub struct ReadImageOpImpl {
    id: usize,
}

impl Compute for ReadImageOpImpl {
    fn get_id(&self) -> usize {
        println!("ReadImageOpImpl.get_id() -> {}", self.id);
        self.id
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("ReadImageOpImpl.compute()");
        Ok(true)
    }
}

pub struct WriteImageOpImpl {
    id: usize,
}

impl Compute for WriteImageOpImpl {
    fn get_id(&self) -> usize {
        println!("WriteImageOpImpl.get_id() -> {}", self.id);
        self.id
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("WriteImageOpImpl.compute()");
        Ok(true)
    }
}

pub struct Operation {
    inner: Box<dyn Compute>,
    op_type: ffi::OperationType,
}

impl Operation {
    fn get_id(&self) -> usize {
        self.inner.get_id()
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        self.inner.compute()
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("Operation.get_op_type() -> {:?}", self.op_type.repr);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("Operation.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }
}

pub fn create_operation(id: usize, op_type: ffi::OperationType) -> Box<Operation> {
    println!("create_operation(id={:?}, op_type={:?})", id, op_type);
    match op_type {
        ffi::OperationType::ReadImage => Box::new(Operation {
            op_type,
            inner: Box::new(ReadImageOpImpl { id }),
        }),
        ffi::OperationType::WriteImage => Box::new(Operation {
            op_type,
            inner: Box::new(WriteImageOpImpl { id }),
        }),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
