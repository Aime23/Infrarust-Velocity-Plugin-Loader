use crate::java::{
    generated::dev::infrarust::event::{
        InfrarustEventManager, InfrarustEventManagerAPI, InfrarustEventManagerNativeInterface,
    },
    handle::NewTypeHandle,
};

impl InfrarustEventManagerNativeInterface for InfrarustEventManagerAPI {
    type Error = jni::errors::Error;

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustEventManager<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context_handle = this.plugin_context_handle(env)?;
        plugin_context_handle.delete_handle();
        Ok(())
    }

    fn native_register_event_handler<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustEventManager<'local>,
        arg0: crate::java::generated::dev::infrarust::event::InfrarustEventManagerRegisteredEventHandler<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        todo!()
    }
}
