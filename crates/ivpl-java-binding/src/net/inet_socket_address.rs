use jni::bind_java_type;

bind_java_type! {
    rust_type = pub InetSocketAddress,
    java_type = java.net.InetSocketAddress,

    constructors {
        pub fn new(hostname: JString, port: jint),
    },
}
