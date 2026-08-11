use crate::java::{
    generated::dev::infrarust::scheduler::{
        InfrarustTaskBuilder, InfrarustTaskBuilderAPI, InfrarustTaskBuilderNativeInterface,
    },
    handle::{NewTypeHandle, PluginContextHandle, SchedulerServiceHandle},
};

impl InfrarustTaskBuilderNativeInterface for InfrarustTaskBuilderAPI {
    type Error = jni::errors::Error;

    fn clone_handle<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustTaskBuilder<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        let scheduler_service_handle = SchedulerServiceHandle::from_instance(
            this.scheduler_handle(env)?.into_instance().clone(),
        );
        return Ok(scheduler_service_handle.into());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustTaskBuilder<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        this.scheduler_handle(env)?.delete_handle();
        return Ok(());
    }

}
