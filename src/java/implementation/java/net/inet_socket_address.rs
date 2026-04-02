use jni::{bind_java_type, objects::JObject, sys::jlong};
use crate::handle::Handle;

bind_java_type! {
    rust_type = pub InetSocketAddress,
    java_type = java.net.InetSocketAddress,

    constructors {
        pub fn new(hostname: JString, port: jint),
    },
}
