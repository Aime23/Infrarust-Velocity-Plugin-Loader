use infrarust_api::event::ResultedEvent;
use jni::objects::JString;

use crate::java::{
    ToJni, TryFromJni,
    generated::{
        com::velocitypowered::api::event::player::{
            KickedFromServerEvent, KickedFromServerEventDisconnectPlayer,
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
            .get_player_by_id(value.player_id)
            .ok_or(TryFromInfrarustEventError::MissingPlayer(value.player_id))?;
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
