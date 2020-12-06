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

    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Hash)]
    enum AttrId {
        ReadImage_FilePath = 0,
        WriteImage_FilePath = 1,
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

trait AttrBlock {
    fn attr_exists(&self, name: &str) -> ffi::AttrState;
    fn get_attr_string(&self, name: &str) -> &str;
    fn set_attr_string(&mut self, name: &str, value: &str);
}

trait Compute {
    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>) -> Result<bool, &'static str>;
}

#[derive(Debug, Clone, Default)]
struct ReadImageCompute {}

#[derive(Debug, Clone, Default)]
struct ReadImageAttrs {
    file_path: String,
}

impl Compute for ReadImageCompute {
    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>) -> Result<bool, &'static str> {
        println!("ReadImageCompute.compute()");
        println!("file_name={}", attr_block.get_attr_string("file_path"));
        Ok(true)
    }
}

impl AttrBlock for ReadImageAttrs {
    fn attr_exists(&self, name: &str) -> ffi::AttrState {
        match name {
            "file_path" => ffi::AttrState::Exists,
            _ => ffi::AttrState::Missing,
        }
    }

    fn get_attr_string(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }
}

#[derive(Debug, Clone, Default)]
struct WriteImageCompute {}

#[derive(Debug, Clone, Default)]
struct WriteImageAttrs {
    file_path: String,
}

impl Compute for WriteImageCompute {
    fn compute(&mut self, attr_block: &Box<dyn AttrBlock>) -> Result<bool, &'static str> {
        println!("WriteImageCompute.compute()");
        println!("file_name={}", attr_block.get_attr_string("file_path"));
        Ok(true)
    }
}

impl AttrBlock for WriteImageAttrs {
    fn attr_exists(&self, name: &str) -> ffi::AttrState {
        match name {
            "file_path" => ffi::AttrState::Exists,
            _ => ffi::AttrState::Missing,
        }
    }

    fn get_attr_string(&self, name: &str) -> &str {
        match name {
            "file_path" => &self.file_path,
            _ => "",
        }
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        match name {
            "file_path" => self.file_path = value.to_string(),
            _ => (),
        };
    }
}

pub struct Operation {
    op_type: ffi::OperationType,
    id: usize,
    compute: Box<dyn Compute>,
    attr_block: Box<dyn AttrBlock>,
}

impl Operation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("Operation.get_op_type() -> {:?}", self.op_type);
        self.op_type
    }

    fn get_op_type_id(&self) -> u8 {
        println!("Operation.get_op_type_id() -> {}", self.op_type.repr);
        self.op_type.repr
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        self.compute.compute(&self.attr_block)
    }

    fn attr_exists(&self, name: &str) -> ffi::AttrState {
        self.attr_block.attr_exists(name)
    }

    fn get_attr_string(&self, name: &str) -> &str {
        self.attr_block.get_attr_string(name)
    }

    fn set_attr_string(&mut self, name: &str, value: &str) {
        self.attr_block.set_attr_string(name, value);
    }
}

pub fn create_operation(id: usize, op_type: ffi::OperationType) -> Box<Operation> {
    println!("create_operation(id={:?}, op_type={:?})", id, op_type);
    match op_type {
        ffi::OperationType::ReadImage => Box::new(Operation {
            op_type,
            id,
            compute: Box::new(ReadImageCompute {}),
            attr_block: Box::new(ReadImageAttrs {
                file_path: "".to_string(),
            }),
        }),
        ffi::OperationType::WriteImage => Box::new(Operation {
            op_type,
            id,
            compute: Box::new(WriteImageCompute {}),
            attr_block: Box::new(WriteImageAttrs {
                file_path: "".to_string(),
            }),
        }),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
