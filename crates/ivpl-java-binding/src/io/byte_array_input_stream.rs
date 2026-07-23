use crate::io::input_stream::InputStream;
use jni::bind_java_type;

bind_java_type! {
    rust_type = pub ByteArrayInputStream,
    java_type = java.io.ByteArrayInputStream,
    type_map {
      InputStream => java.io.InputStream
    },
    constructors {
        pub fn new(buf: jbyte[]),
        // pub fn new(buf: JByteArray, offset: jint, lenght: jint),
    },
    methods {
        fn read() -> jint
    },
    is_instance_of {
            input_stream: InputStream,
    },

}
