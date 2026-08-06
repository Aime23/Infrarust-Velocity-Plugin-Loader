use jni::bind_java_type;

bind_java_type! {
    pub CompletableFuture => "java.util.concurrent.CompletableFuture",
    methods {
        fn join() -> "java.lang.Object",
    },
}
