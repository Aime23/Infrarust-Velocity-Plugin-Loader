use jni::bind_java_type;

bind_java_type! {
    rust_type = pub Optional,
    java_type = java.util.Optional,

    constructors {

    },
    methods {
        static fn of(value: JObject) -> Optional,
        static fn empty() -> Optional,
        fn get() -> JObject
    },
    fields {
        value: JObject
    }
}
