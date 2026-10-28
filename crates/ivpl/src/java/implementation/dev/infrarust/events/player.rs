use infrarust_api::{event::ResultedEvent, types::ServerId};
use jni::{objects::JString, refs::Reference};

use crate::java::{
    ToJni, TryFromJni,
    generated::{
        com::velocitypowered::api::event::player::{
            KickedFromServerEvent, KickedFromServerEventDisconnectPlayer,
            KickedFromServerEventNotify, KickedFromServerEventRedirectPlayer,
            KickedFromServerEventServerKickResult, PlayerChatEvent, PlayerChatEventChatResult,
        },
        dev::infrarust::proxy::{InfrarustPlayer, server::InfrarustRegisteredServer},
        net::kyori::adventure::text::Component,
    },
    handle::{ConfigServiceHandle, NewTypeHandle, PlayerRegistryHandle, PluginContextHandle},
    implementation::dev::infrarust::events::{
        ApplyEventResult, TryFromInfrarustEvent, TryFromInfrarustEventError,
    },
};

// PlayerChatEvent

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::ChatMessageEvent>
    for PlayerChatEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::ChatMessageEvent,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError> {
        let plugin_context = plugin_context_handle.into_instance();

        let username = JString::from_str(env, &value.message)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let player = plugin_context
            .player_registry()
            .get_player_by_id(value.player_id())
            .ok_or(TryFromInfrarustEventError::MissingPlayer(value.player_id()))?;
        let player = player
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let pre_login_event = PlayerChatEvent::new(env, player, username)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(pre_login_event);
    }
}

impl<'local> ApplyEventResult<'local, infrarust_api::events::ChatMessageEvent>
    for PlayerChatEvent<'local>
{
    fn apply_event_result(
        &self,
        env: &mut ::jni::Env<'local>,
        event: &mut infrarust_api::events::ChatMessageEvent,
    ) -> jni::errors::Result<()> {
        let result = self.get_result_1(env)?;

        let message = result.get_message(env)?;
        let message = Option::<JString>::try_from_jni(env, message)?;
        if let Some(message) = message {
            event.modify(message.try_to_string(env)?);
        }
        if !result.is_allowed(env)? {
            event.deny(infrarust_api::types::Component::default());
        }
        return Ok(());
    }
}

// KickedFromServerEvent

impl<'local> TryFromInfrarustEvent<'local, infrarust_api::events::KickedFromServerEvent>
    for KickedFromServerEvent<'local>
{
    fn try_from_infrarust_event(
        value: &infrarust_api::events::KickedFromServerEvent,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError> {
        let plugin_context = plugin_context_handle.into_instance();
        let player_registry =
            PlayerRegistryHandle::from_instance(Box::new(plugin_context.player_registry()));
        let config_service =
            ConfigServiceHandle::from_instance(Box::new(plugin_context.config_service()));

        let player = plugin_context
            .player_registry()
            .get_player_by_id(value.player_id())
            .ok_or(TryFromInfrarustEventError::MissingPlayer(value.player_id()))?;
        let player = player
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let server_id = value
            .server
            .to_string()
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        let registered_server =
            InfrarustRegisteredServer::new(env, player_registry, config_service, server_id)
                .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        let reason = value
            .reason
            .clone()
            .unwrap_or_default()
            .to_jni(env)
            .map_err(|err| TryFromInfrarustEventError::Java(err))?;

        // Reflect the default result chosen by the proxy so Java listeners see the pending
        // action. KickedFromServerResult::SendToLimbo has no Velocity equivalent and falls
        // back to disconnecting the player.
        let result: KickedFromServerEventServerKickResult = match value.result() {
            infrarust_api::events::KickedFromServerResult::DisconnectPlayer { reason } => {
                let reason = reason
                    .clone()
                    .unwrap_or_else(|| value.reason.clone().unwrap_or_default());
                let reason = reason
                    .to_jni(env)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?;
                KickedFromServerEventDisconnectPlayer::create(env, &reason)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?
                    .into()
            }
            infrarust_api::events::KickedFromServerResult::Redirect(server) => {
                let player_registry =
                    PlayerRegistryHandle::from_instance(Box::new(plugin_context.player_registry()));
                let config_service =
                    ConfigServiceHandle::from_instance(Box::new(plugin_context.config_service()));
                let server_id = server
                    .to_string()
                    .to_jni(env)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?;
                let registered_server = InfrarustRegisteredServer::new(
                    env,
                    player_registry,
                    config_service,
                    server_id,
                )
                .map_err(|err| TryFromInfrarustEventError::Java(err))?;
                KickedFromServerEventRedirectPlayer::create(env, &registered_server)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?
            }
            infrarust_api::events::KickedFromServerResult::Notify { message } => {
                let message = message
                    .clone()
                    .to_jni(env)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?;
                KickedFromServerEventNotify::create(env, &message)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?
                    .into()
            }
            _ => {
                let reason = value.reason.clone().unwrap_or_default();
                let reason = reason
                    .to_jni(env)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?;
                KickedFromServerEventDisconnectPlayer::create(env, &reason)
                    .map_err(|err| TryFromInfrarustEventError::Java(err))?
                    .into()
            }
        };

        let event = KickedFromServerEvent::new(
            env,
            &player,
            &registered_server,
            &reason,
            value.during_connect,
            &result,
        )
        .map_err(|err| TryFromInfrarustEventError::Java(err))?;
        return Ok(event);
    }
}

impl<'local> ApplyEventResult<'local, infrarust_api::events::KickedFromServerEvent>
    for KickedFromServerEvent<'local>
{
    fn apply_event_result(
        &self,
        env: &mut ::jni::Env<'local>,
        event: &mut infrarust_api::events::KickedFromServerEvent,
    ) -> jni::errors::Result<()> {
        let result = self.get_result_1(env)?;

        if env.is_instance_of(
            &result,
            &*KickedFromServerEventDisconnectPlayer::lookup_class(
                env,
                &jni::refs::LoaderContext::None,
            )?,
        )? {
            let result = KickedFromServerEventDisconnectPlayer::cast_local(env, result)?;
            let reason = result.get_reason_component(env)?;
            let reason = infrarust_api::types::Component::try_from_jni(env, reason)?;
            event.set_result(
                infrarust_api::events::KickedFromServerResult::DisconnectPlayer {
                    reason: Some(reason),
                },
            );
        } else if env.is_instance_of(
            &result,
            &*KickedFromServerEventRedirectPlayer::lookup_class(
                env,
                &jni::refs::LoaderContext::None,
            )?,
        )? {
            let result = KickedFromServerEventRedirectPlayer::cast_local(env, result)?;
            let server = result.get_server(env)?;
            let server = InfrarustRegisteredServer::cast_local(env, server)?;
            let server_id = server.server_id(env)?;
            let server_id = ServerId::new(server_id.try_to_string(env)?);
            event.set_result(infrarust_api::events::KickedFromServerResult::Redirect(
                server_id,
            ));
        } else if env.is_instance_of(
            &result,
            &*KickedFromServerEventNotify::lookup_class(env, &jni::refs::LoaderContext::None)?,
        )? {
            let result = KickedFromServerEventNotify::cast_local(env, result)?;
            let message = result.get_message_component(env)?;
            let message = infrarust_api::types::Component::try_from_jni(env, message)?;
            event.set_result(infrarust_api::events::KickedFromServerResult::Notify { message });
        }
        return Ok(());
    }
}
