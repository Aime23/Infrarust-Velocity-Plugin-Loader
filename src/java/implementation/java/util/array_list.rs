use crate::handle::Handle;
use jni::{bind_java_type, objects::JObject, sys::jlong};

bind_java_type! {
    rust_type = pub ArrayList,
    java_type = java.util.ArrayList,
    constructors {
        fn new()
    },
    methods {
        fn add(value: JObject)
    }
}
