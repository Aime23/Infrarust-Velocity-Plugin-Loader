use jni::{bind_java_type, sys::jlong};

use crate::{ToJni, TryFromJni};

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

impl<'local> TryFromJni<'local, UUID<'local>> for uuid::Uuid {
    fn try_from_jni(env: &mut jni::Env<'local>, value: UUID<'local>) -> Result<Self, jni::errors::Error> {
        let most_sig_bits = value.get_most_significant_bits(env)?;
        let least_sig_bits = value.get_least_significant_bits(env)?;
        Ok(Self::from_u64_pair(most_sig_bits as u64, least_sig_bits as u64))
    }
}

impl<'local> ToJni<'local> for uuid::Uuid {
    type Kind = UUID<'local>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let (most_sig, least_sig) = self.as_u64_pair();
        UUID::new(env, most_sig as jlong, least_sig as jlong)
    }
}
