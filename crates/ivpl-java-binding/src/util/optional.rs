use jni::bind_java_type;

bind_java_type! {
    rust_type = pub Optional,
    java_type = java.util.Optional,

    constructors {
        fn of(value: JObject),
        fn empty()
    },
    methods {
        fn get() -> JObject
    },
    fields {
        value: JObject
    }
}
