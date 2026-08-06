use crate::java::{
    generated::com::velocitypowered::api::event::proxy::ProxyInitializeEvent,
    handle::PluginContextHandle,
    implementation::dev::infrarust::events::{TryFromInfrarustEvent, TryFromInfrarustEventError},
};

// ProxyInitializeEvent
impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::ProxyInitializeEvent>
    for ProxyInitializeEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::ProxyInitializeEvent,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError> {
        let pre_login_event =
            ProxyInitializeEvent::new(env).map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(pre_login_event);
    }
}
