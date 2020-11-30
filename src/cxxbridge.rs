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
        type ReadImageOp;
        fn get_id(self: &mut ReadImageOp) -> usize;
        fn get_op_type(self: &mut ReadImageOp) -> OperationType;
        fn get_op_type_id(self: &mut ReadImageOp) -> u8;
        fn compute(self: &mut ReadImageOp) -> Result<bool>;
        fn create_read_image_op(id: usize) -> Box<ReadImageOp>;

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

pub struct ReadImageOp {
    id: usize,
    op_type: ffi::OperationType,
}

impl ReadImageOp {
    fn get_id(&self) -> usize {
        println!("ReadImageOp.get_id() -> {}", self.id);
        self.id
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("ReadImageOp.get_op_type() -> {}", self.op_type.repr);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("ReadImageOp.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("ReadImageOp.compute()");
        Ok(true)
    }
}

pub fn create_read_image_op(id: usize) -> Box<ReadImageOp> {
    println!("create_read_image_op()");
    let op_type = ffi::OperationType::ReadImage;
    Box::new(ReadImageOp { id, op_type })
}

pub struct WriteImageOp {
    id: usize,
    op_type: ffi::OperationType,
}

impl WriteImageOp {
    fn get_id(&self) -> usize {
        println!("WriteImageOp.get_id() -> {}", self.id);
        self.id
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("WriteImageOp.get_op_type() -> {}", self.op_type.repr);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("WriteImageOp.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        println!("WriteImageOp.compute()");
        Ok(true)
    }
}

pub fn create_write_image_op(id: usize) -> Box<WriteImageOp> {
    println!("create_write_image_op()");
    let op_type = ffi::OperationType::WriteImage;
    Box::new(WriteImageOp { id, op_type })
}
