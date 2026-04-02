use jni::{bind_java_type, objects::JObject, sys::jlong};
use crate::{handle::Handle, java::ToJni};

bind_java_type! {
    rust_type = pub Path,
    java_type = java.nio.file.Path,

    constructors {
        pub fn of(first: JString, more: JString[]),
    },
}


impl<'local> ToJni<'local> for &std::path::Path {
    type Kind = Path<'local>;

    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {

        todo!()
    }
}
