use jni::bind_java_type;

bind_java_type! {
    rust_type = pub UUID,
    java_type = java.util.UUID,
    constructors {
        fn new(most_sig_bits: jlong, least_sig_bits: jlong)
    },
    methods {
        fn get_most_significant_bits() -> jlong,
        fn get_least_significant_bits() -> jlong
    }
}
