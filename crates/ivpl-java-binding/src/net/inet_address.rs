use jni::bind_java_type;

bind_java_type! {
    rust_type = pub InetAddress,
    java_type = java.net.InetAddress,
    methods {
        static fn get_by_address(jbyte[]) -> InetAddress
    }

}
