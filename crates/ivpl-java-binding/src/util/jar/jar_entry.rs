use jni::bind_java_type;

bind_java_type! {
    rust_type = pub JarEntry,
    java_type = java.util.jar.JarEntry,
    methods {
        fn get_name() -> JString
    }
}
