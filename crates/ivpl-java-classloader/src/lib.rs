use jni::bind_java_type;

pub const BYTE_ARRAY_CLASS_LOADER_BYTES: &[u8] =
    include_bytes!(env!("BYTE_ARRAY_CLASS_LOADER_PATH"));

bind_java_type! {
    rust_type = pub ByteArrayClassLoader,
    java_type = .ByteArrayClassLoader,

    constructors {
        fn new(value: jbyte[], parent: JClassLoader),
    },
    methods {
        fn find_class(name: JString) -> JClass
    },
    is_instance_of {
            class_loader: JClassLoader,
    }
}
