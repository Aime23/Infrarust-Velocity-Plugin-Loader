use crate::java::generated::dev::infrarust::scheduler::{
    InfrarustScheduledTask, InfrarustScheduledTaskAPI, InfrarustScheduledTaskNativeInterface,
};

impl InfrarustScheduledTaskNativeInterface for InfrarustScheduledTaskAPI {
    type Error = jni::errors::Error;

    fn native_cancel<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduledTask<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        todo!()
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduledTask<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        todo!()
    }
}
