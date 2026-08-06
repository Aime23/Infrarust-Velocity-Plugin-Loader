use crate::net::inet_address::InetAddress;
use jni::bind_java_type;

bind_java_type! {
    rust_type = pub InetSocketAddress,
    java_type = java.net.InetSocketAddress,
    type_map {
      InetAddress => java.net.InetAddress
    },
    constructors {
        pub fn new(hostname: JString, port: jint),
        #[allow(non_snake_case)]
        pub fn new2(hostname: InetAddress, port: jint),
    },
}
