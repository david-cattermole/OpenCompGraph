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
    // Read Operation
    extern "Rust" {
        type ReadImageOp;
        fn get_id(self: &mut ReadImageOp) -> usize;
        fn get_op_type(self: &mut ReadImageOp) -> OperationType;
        fn get_op_type_id(self: &mut ReadImageOp) -> u8;
        fn compute(self: &mut ReadImageOp) -> Result<bool>;
        fn create_read_image_op(id: usize) -> Box<ReadImageOp>;
    }

    // Write Operation
    extern "Rust" {
        type WriteImageOp;
        fn get_id(self: &mut WriteImageOp) -> usize;
        fn get_op_type(self: &mut WriteImageOp) -> OperationType;
        fn get_op_type_id(self: &mut WriteImageOp) -> u8;
        fn compute(self: &mut WriteImageOp) -> Result<bool>;
        fn create_write_image_op(id: usize) -> Box<WriteImageOp>;
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
    fn get_op_type(&self) -> ffi::OperationType;
    fn get_op_type_id(&self) -> u8;
    fn compute(&mut self) -> Result<bool, &'static str>;
}

pub struct ReadImageOpImpl {
    id: usize,
    op_type: ffi::OperationType,
}

impl Compute for ReadImageOpImpl {
    fn get_id(&self) -> usize {
        println!("ReadImageOpImpl.get_id() -> {}", self.id);
        self.id
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("ReadImageOpImpl.get_op_type() -> {}", self.op_type.repr);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("ReadImageOpImpl.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("ReadImageOpImpl.compute()");
        Ok(true)
    }
}

pub struct ReadImageOp {
    op: Box<dyn Compute>,
}

impl ReadImageOp {
    fn get_id(&self) -> usize {
        self.op.get_id()
    }

    fn get_op_type(&self) -> ffi::OperationType {
        self.op.get_op_type()
    }

    fn get_op_type_id(&self) -> u8 {
        self.op.get_op_type_id()
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        self.op.compute()
    }
}

pub fn create_read_image_op(id: usize) -> Box<ReadImageOp> {
    println!("create_read_image_op()");
    let op_type = ffi::OperationType::ReadImage;
    let internal = Box::new(ReadImageOpImpl { id, op_type });
    Box::new(ReadImageOp { op: internal })
}

pub struct WriteImageOpImpl {
    id: usize,
    op_type: ffi::OperationType,
}

impl Compute for WriteImageOpImpl {
    fn get_id(&self) -> usize {
        println!("WriteImageOpImpl.get_id() -> {}", self.id);
        self.id
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("WriteImageOpImpl.get_op_type() -> {}", self.op_type.repr);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("WriteImageOpImpl.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("WriteImageOpImpl.compute()");
        Ok(true)
    }
}

pub struct WriteImageOp {
    op: Box<dyn Compute>,
}

impl WriteImageOp {
    fn get_id(&self) -> usize {
        println!("WriteImageOp.get_id() -> {}", self.op.get_id());
        self.op.get_id()
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("WriteImageOp.get_op_type() -> {:?}", self.op.get_op_type());
        self.op.get_op_type()
    }

    fn get_op_type_id(&self) -> u8 {
        println!(
            "WriteImageOp.get_op_type_id() -> {}",
            self.op.get_op_type_id()
        );
        self.op.get_op_type_id()
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("WriteImageOp.compute()");
        self.op.compute()
    }
}

pub fn create_write_image_op(id: usize) -> Box<WriteImageOp> {
    println!("create_write_image_op()");
    let op_type = ffi::OperationType::WriteImage;
    let internal = Box::new(WriteImageOpImpl { id, op_type });
    Box::new(WriteImageOp { op: internal })
}
