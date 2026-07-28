use jni::bind_java_type;

bind_java_type! {
    rust_type = pub Paths,
    java_type = java.nio.file.Paths,
    type_map = {
            Path => java.nio.file.Path,
        },
    methods {
        static fn get(first: JString, more: JString[]) -> java.nio.file.Path
    }

}

bind_java_type! {
    rust_type = pub Path,
    java_type = java.nio.file.Path,

    constructors {
        pub fn of(first: JString, more: JString[]),
    },

}
