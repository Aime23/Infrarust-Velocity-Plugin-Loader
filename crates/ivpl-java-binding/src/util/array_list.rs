use jni::bind_java_type;

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
