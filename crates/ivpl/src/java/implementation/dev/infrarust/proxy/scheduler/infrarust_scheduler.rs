use crate::java::{
    generated::dev::infrarust::scheduler::{
        InfrarustScheduler, InfrarustSchedulerAPI, InfrarustSchedulerNativeInterface,
    },
    handle::{NewTypeHandle, PluginContextHandle},
};

impl InfrarustSchedulerNativeInterface for InfrarustSchedulerAPI {
    type Error = jni::errors::Error;

    fn clone_handle<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        let plugin_context_handle = PluginContextHandle::from_instance(
            this.plugin_context_handle(env)?.into_instance().clone(),
        );
        return Ok(plugin_context_handle.into());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        this.plugin_context_handle(env)?.delete_handle();
        return Ok(());
    }
}
