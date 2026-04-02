use std::sync::Arc;

use jni::bind_java_type;

use crate::{
    handle::Handle,
    java::{
        ToJni,
        generated::dev::infrarust::proxy::{
            InfrarustPlayer, InfrarustPlayerAPI, InfrarustPlayerNativeInterface,
        }, implementation::java::util::optional::Optional,
    },
};

impl<'local> ToJni<'local> for Arc<dyn infrarust_api::player::Player> {
    type Kind = InfrarustPlayer<'local>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let handle = Handle::from(self.clone()).raw();
        return InfrarustPlayer::new(env, handle);
    }
}
