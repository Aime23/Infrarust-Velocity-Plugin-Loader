use std::sync::Arc;

use infrarust_api::types::Component;
use jni::{bind_java_type, objects::JString};

use crate::{
    handle::Handle,
    java::{
        ToJni,
        generated::dev::infrarust::proxy::{
            InfrarustPlayer, InfrarustPlayerAPI, InfrarustPlayerNativeInterface,
        },
        handle::{NewTypeHandle, PlayerHandle},
        implementation::java::util::{optional::Optional, uuid::UUID},
    },
};

impl InfrarustPlayerNativeInterface for InfrarustPlayerAPI {
    type Error = jni::errors::Error;

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
    ) -> ::std::result::Result<
        crate::java::implementation::java::util::optional::Optional<'local>,
        Self::Error,
    > {
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

    fn native_disconnect<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustPlayer<'local>,
        component: crate::java::generated::net::kyori::adventure::text::Component<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let player = this.player_handle(env)?.into_instance();
        // TODO: Use provided component
        player.disconnect(Component::text("Disconnected"));
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
