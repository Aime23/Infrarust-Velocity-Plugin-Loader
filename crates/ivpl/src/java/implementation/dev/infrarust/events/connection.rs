use infrarust_api::{event::ResultedEvent, permissions::PermissionLevel::Player};
use jni::objects::JString;

use crate::java::{
    ToJni, TryFromJni,
    generated::{
        com::velocitypowered::api::{
            event::{
                connection::{
                    ConnectionHandshakeEvent, DisconnectEvent, DisconnectEventLoginStatus,
                    PostLoginEvent, PreLoginEvent,
                },
                player::ServerConnectedEvent,
            },
            network::{HandshakeIntent, ProtocolState, ProtocolVersion},
        },
        dev::infrarust::proxy::{
            InfrarustInboundConnection, InfrarustPlayer, InfrarustServer,
            server::InfrarustRegisteredServer,
        },
        net::kyori::adventure::text::Component,
    },
    handle::{
        ConfigServiceHandle, NewTypeHandle, PlayerHandle, PlayerRegistryHandle, PluginContextHandle,
    },
    implementation::dev::infrarust::{
        events::{ApplyEventResult, TryFromInfrarustEvent, TryFromInfrarustEventError, player},
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

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::DisconnectEvent>
    for DisconnectEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::DisconnectEvent,
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

        let login_status = DisconnectEventLoginStatus::SUCCESSFUL_LOGIN(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let disconnect_event = DisconnectEvent::new(env, player, login_status)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(disconnect_event);
    }
}

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::PostLoginEvent>
    for PostLoginEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::PostLoginEvent,
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

        // let player = match player {
        //     Some(p) => p
        //         .to_jni(env)
        //         .map_err(|err| TryFromInfrarustEventError::Java(err))?,
        //     None => InfrarustPlayer::null(),
        // };

        let post_login_event = PostLoginEvent::new(env, player)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(post_login_event);
    }
}

// PreLoginEvent

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::PreLoginEvent>
    for PreLoginEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::PreLoginEvent,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError> {
        let plugin_context = plugin_context_handle.into_instance();

        let username = JString::from_str(env, &value.profile.username)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let uuid = &value
            .profile
            .uuid
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let inet_socket_address = value
            .remote_addr
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let protocol_version = value
            .protocol_version
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let protocol_state =
            ProtocolState::LOGIN(env).map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let handshake_intent =
            HandshakeIntent::LOGIN(env).map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let inbound_connection = InfrarustInboundConnection::new(
            env,
            inet_socket_address,
            protocol_version,
            protocol_state,
            handshake_intent,
        )
        .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let pre_login_event = PreLoginEvent::new3(env, inbound_connection, username, uuid)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(pre_login_event);
    }
}

impl<'local> ApplyEventResult<'local, infrarust_api::events::PreLoginEvent>
    for PreLoginEvent<'local>
{
    fn apply_event_result(
        &self,
        env: &mut ::jni::Env<'local>,
        event: &mut infrarust_api::events::PreLoginEvent,
    ) -> jni::errors::Result<()> {
        let result = self.get_result_1(env)?;
        let is_disallowed = !result.is_allowed(env)?;
        let is_force_offline_mode = result.is_force_offline_mode(env)?;
        let is_force_online_mode = result.is_online_mode_allowed(env)?;

        if is_disallowed {
            let reason = result.get_reason_component(env)?;
            let reason = Option::<Component>::try_from_jni(env, reason)?;
            let reason = if let Some(value) = reason {
                infrarust_api::types::Component::try_from_jni(env, value)?
            } else {
                infrarust_api::types::Component::default()
            };
            event.deny(reason);
        } else if is_force_offline_mode {
            event.set_result(infrarust_api::events::PreLoginResult::ForceOffline);
        } else if is_force_online_mode {
            event.set_result(infrarust_api::events::PreLoginResult::ForceOnline);
        }
        return Ok(());
    }
}
