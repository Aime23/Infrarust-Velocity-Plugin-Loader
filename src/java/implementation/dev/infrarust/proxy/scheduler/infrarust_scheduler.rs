use crate::java::generated::dev::infrarust::scheduler::{
    InfrarustScheduler, InfrarustSchedulerAPI, InfrarustSchedulerNativeInterface,
};

impl InfrarustSchedulerNativeInterface for InfrarustSchedulerAPI {
    type Error = jni::errors::Error;

    fn clone_handle<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        todo!()
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        todo!()
    }
}
