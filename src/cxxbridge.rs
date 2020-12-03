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

        fn attr_exists(&self, attr: AttrId) -> AttrState;

        fn get_attr_string(&self, attr: AttrId) -> String;

        #[cxx_name = "set_attr"]
        fn set_attr_string(&mut self, attr: AttrId, value: String);
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

trait Compute {
    fn attr_exists(&self, attr: ffi::AttrId) -> ffi::AttrState;
    fn get_attr_string(&self, attr: ffi::AttrId) -> String;
    fn set_attr_string(&mut self, attr: ffi::AttrId, value: String);
}

#[derive(Debug, Clone, Default)]
struct ReadImageOpImpl {
    file_path: String,
}

fn read_image_compute() -> Result<bool, &'static str> {
    println!("read_image_compute()");
    Ok(true)
}

fn write_image_compute() -> Result<bool, &'static str> {
    println!("write_image_compute()");
    Ok(true)
}

impl Compute for ReadImageOpImpl {
    fn attr_exists(&self, attr: ffi::AttrId) -> ffi::AttrState {
        match attr {
            ffi::AttrId::ReadImage_FilePath => ffi::AttrState::Exists,
            _ => ffi::AttrState::Missing,
        }
    }

    fn get_attr_string(&self, attr: ffi::AttrId) -> String {
        match attr {
            ffi::AttrId::ReadImage_FilePath => self.file_path.clone(),
            _ => "".to_string(),
        }
    }

    fn set_attr_string(&mut self, attr: ffi::AttrId, value: String) {
        match attr {
            ffi::AttrId::ReadImage_FilePath => self.file_path = value,
            _ => (),
        };
    }
}

#[derive(Debug, Clone, Default)]
struct WriteImageOpImpl {
    file_path: String,
}

impl Compute for WriteImageOpImpl {
    fn attr_exists(&self, attr: ffi::AttrId) -> ffi::AttrState {
        match attr {
            ffi::AttrId::WriteImage_FilePath => ffi::AttrState::Exists,
            _ => ffi::AttrState::Missing,
        }
    }

    fn get_attr_string(&self, attr: ffi::AttrId) -> String {
        match attr {
            ffi::AttrId::WriteImage_FilePath => self.file_path.clone(),
            _ => "".to_string(),
        }
    }

    fn set_attr_string(&mut self, attr: ffi::AttrId, value: String) {
        match attr {
            ffi::AttrId::WriteImage_FilePath => self.file_path = value,
            _ => (),
        };
    }
}

pub struct Operation {
    op_type: ffi::OperationType,
    id: usize,
    inner: Box<dyn Compute>,
}

impl Operation {
    fn get_id(&self) -> usize {
        self.id
    }

    fn compute(&mut self) -> Result<bool, &'static str> {
        match self.op_type {
            ffi::OperationType::ReadImage => read_image_compute(),
            ffi::OperationType::WriteImage => write_image_compute(),
            _ => Ok(true),
        }
    }

    fn attr_exists(&self, attr: ffi::AttrId) -> ffi::AttrState {
        self.inner.attr_exists(attr)
    }

    fn get_attr_string(&self, attr: ffi::AttrId) -> String {
        self.inner.get_attr_string(attr)
    }

    fn set_attr_string(&mut self, attr: ffi::AttrId, value: String) {
        self.inner.set_attr_string(attr, value);
    }

    fn get_op_type(&self) -> ffi::OperationType {
        println!("Operation.get_op_type() -> {:?}", self.op_type);
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
            id,
            inner: Box::new(ReadImageOpImpl {
                file_path: "".to_string(),
            }),
        }),
        ffi::OperationType::WriteImage => Box::new(Operation {
            op_type,
            id,
            inner: Box::new(WriteImageOpImpl {
                file_path: "".to_string(),
            }),
        }),
        _ => panic!("Invalid OperationType: {:?}", op_type),
    }
}
