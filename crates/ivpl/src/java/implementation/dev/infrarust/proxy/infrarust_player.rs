use std::sync::Arc;

use infrarust_api::types::Component;
use ivpl_java_binding::util::{optional::Optional, uuid::UUID};
use jni::{bind_java_type, objects::JString};

use crate::{
    handle::Handle,
    java::{
        ToJni, TryFromJni,
        generated::dev::infrarust::proxy::{
            InfrarustPlayer, InfrarustPlayerAPI, InfrarustPlayerNativeInterface,
        },
        handle::{NewTypeHandle, PlayerHandle},
    },
};

impl InfrarustPlayerNativeInterface for InfrarustPlayerAPI {
    type Error = jni::errors::Error;

    fn native_disconnect<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
        component: crate::java::generated::net::kyori::adventure::text::Component<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let player = this.player_handle(env)?.into_instance();
        let message = Component::try_from_jni(env, component)?;
        player.disconnect(message);
        return Ok(());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        this.player_handle(env)?.delete_handle();
        Ok(())
    }

    fn native_get_current_server<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
    ) -> ::std::result::Result<Optional<'local>, Self::Error> {
        let player = this.player_handle(env)?.into_instance();
        return player.current_server().map(|v| v.to_string()).to_jni(env);
    }

    fn native_get_unique_id<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
    ) -> Result<UUID<'local>, jni::errors::Error> {
        let player = this.player_handle(env)?.into_instance();
        return player.profile().uuid.to_jni(env);
    }

    fn native_get_username<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
    ) -> ::std::result::Result<::jni::objects::JString<'local>, Self::Error> {
        let player = this.player_handle(env)?.into_instance();
        let username = player.profile().username.clone();
        return JString::from_str(env, username);
    }

    fn native_send_message<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
        arg1: crate::java::generated::net::kyori::adventure::text::Component<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let player = this.player_handle(env)?.into_instance();
        let message = Component::try_from_jni(env, arg1)?;
        player.send_message(message);
        return Ok(());
    }
}

impl<'local> ToJni<'local> for Arc<dyn infrarust_api::player::Player> {
    type Kind = InfrarustPlayer<'local>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let handle = PlayerHandle::from_instance(Box::new(self.clone()));
        return InfrarustPlayer::new(env, handle);
    }
}
