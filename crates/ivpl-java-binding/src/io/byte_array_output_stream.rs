use jni::bind_java_type;

bind_java_type! {
    rust_type = pub ByteArrayOutputStream,
    java_type = java.io.ByteArrayOutputStream,
    constructors {
        pub fn new(),
        // pub fn new(buf: JByteArray, offset: jint, lenght: jint),
    },
    methods {
        fn write(b: jint),
        fn to_byte_array() -> jbyte[]
    }

}
