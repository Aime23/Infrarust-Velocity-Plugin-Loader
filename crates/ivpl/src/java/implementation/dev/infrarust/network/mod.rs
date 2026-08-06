use crate::java::{ToJni, generated::com::velocitypowered::api::network::ProtocolVersion};

impl<'local> ToJni<'local> for infrarust_api::types::ProtocolVersion {
    type Kind = ProtocolVersion<'local>;

    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        return ProtocolVersion::get_protocol_version(env, self.raw());
    }
}
