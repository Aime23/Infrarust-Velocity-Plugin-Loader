use crate::java::{
    ToJni,
    generated::{
        com::velocitypowered::api::event::player::ServerConnectedEvent,
        dev::infrarust::proxy::{
            InfrarustPlayer, InfrarustServer, server::InfrarustRegisteredServer,
        },
    },
    handle::{
        ConfigServiceHandle, NewTypeHandle, PlayerHandle, PlayerRegistryHandle, PluginContextHandle,
    },
    implementation::dev::infrarust::{
        events::{TryFromInfrarustEvent, TryFromInfrarustEventError},
        proxy::server::registered_server,
    },
};

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::ServerConnectedEvent>
    for ServerConnectedEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::ServerConnectedEvent,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError> {
        let plugin_context = plugin_context_handle.into_instance();
        let player = plugin_context
            .player_registry()
            .get_player_by_id(value.player_id)
            .ok_or(TryFromInfrarustEventError::MissingPlayer(value.player_id))?;
        let player = player
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let player_registry_handle =
            PlayerRegistryHandle::from_instance(Box::new(plugin_context.player_registry_handle()));
        let config_service_handle =
            ConfigServiceHandle::from_instance(Box::new(plugin_context.config_service_handle()));
        let server_id = value
            .server
            .to_string()
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let registered_server = InfrarustRegisteredServer::new(
            env,
            player_registry_handle,
            config_service_handle,
            server_id,
        )
        .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let server_connected_event = ServerConnectedEvent::new(
            env,
            player,
            registered_server,
            InfrarustRegisteredServer::null(),
        )
        .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(server_connected_event);
    }
}
