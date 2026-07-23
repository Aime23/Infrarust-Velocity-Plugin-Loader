use crate::{io::input_stream::InputStream, util::jar::jar_entry::JarEntry};
use jni::bind_java_type;

bind_java_type! {
    rust_type = pub JarInputStream,
    java_type = java.util.jar.JarInputStream,
    type_map = {
            InputStream => java.io.InputStream,
            JarEntry => java.util.jar.JarEntry
        },
    constructors {
        pub fn new(input: InputStream),
        // pub fn new(input: InputStream, validate: jbool),
    },
    methods {
        fn get_next_jar_entry() -> JarEntry
    },
    is_instance_of {
            input_stream: InputStream,
    },
}
