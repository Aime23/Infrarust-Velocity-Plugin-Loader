use crate::ToJni;
use jni::{
    bind_java_type,
    objects::{JObjectArray, JString},
};

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

impl<'local> ToJni<'local> for &std::path::Path {
    type Kind = Path<'local>;

    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let path = JString::from_str(env, self.to_str().unwrap())?;
        let more = JObjectArray::<JString>::new(env, 0, JString::null())?;
        return Paths::get(env, path, more);
    }
}
