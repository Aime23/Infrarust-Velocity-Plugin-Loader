use jni::bind_java_type;

bind_java_type! {
    rust_type = pub InputStream,
    java_type = java.io.InputStream,
    methods {
        fn read(b: jbyte[]) -> jint,
        fn read_all_bytes() -> jbyte[]
    },
}
