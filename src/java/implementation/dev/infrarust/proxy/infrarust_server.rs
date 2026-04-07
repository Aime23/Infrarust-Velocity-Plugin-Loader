use std::{ops::Deref, sync::Arc};

use infrarust_api::{plugin::PluginContext, prelude::EventBusExt};
use jni::{
    bind_java_type,
    objects::{JCollection, JObject, JObjectArray, JString, JValue},
    sys::{jboolean, jint},
};

use crate::{
    handle::Handle,
    java::{
        ToJni, ToJniArray,
        generated::dev::infrarust::proxy::{
            InfrarustPlayer, InfrarustServer, InfrarustServerAPI, InfrarustServerNativeInterface,
            server::InfrarustRegisteredServer,
        },
        handle::NewTypeHandle,
        implementation::java::util::optional::Optional,
    },
};

impl InfrarustServerNativeInterface for InfrarustServerAPI {
    type Error = jni::errors::Error;

    fn native_get_player_by_name<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        name: ::jni::objects::JString<'local>,
    ) -> ::std::result::Result<Optional<'local>, Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        let name_str: String = name.to_string();
        return Ok(context
            .player_registry()
            .get_player(&name_str)
            .to_jni(env)?
            .into());
    }

    fn native_get_player_by_uuid<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        uuid_1: ::jni::sys::jlong,
        uuid_2: ::jni::sys::jlong,
    ) -> ::std::result::Result<Optional<'local>, Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        let uuid = uuid::Uuid::from_u64_pair(uuid_1 as u64, uuid_2 as u64);
        return Ok(context
            .player_registry()
            .get_player_by_uuid(&uuid)
            .to_jni(env)?
            .into());
    }

    fn native_get_all_players<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
    ) -> ::std::result::Result<JObjectArray<'local, InfrarustPlayer<'local>>, Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        let players = context.player_registry().get_all_players();
        return players.to_jni(env);
    }

    fn native_match_player<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        name: ::jni::objects::JString<'local>,
    ) -> ::std::result::Result<JObjectArray<'local, InfrarustPlayer<'local>>, Self::Error> {
        let pattern = name.to_string();
        let context = this.plugin_context_handle(env)?.into_instance();

        let players: Vec<Arc<dyn infrarust_api::player::Player>> = context
            .player_registry()
            .get_all_players()
            .into_iter()
            .filter(|v| v.profile().username.starts_with(&pattern))
            .collect();
        return players.to_jni(env);
    }

    fn native_get_player_count<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
    ) -> ::std::result::Result<jint, Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        let count = context.player_registry().online_count() as jint;
        return Ok(count);
    }

    fn native_get_server<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        server_name: ::jni::objects::JString<'local>,
    ) -> ::std::result::Result<Optional<'local>, Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        // let server_name = server_name.to_string();
        let registered_server = InfrarustRegisteredServer::new(
            env,
            Handle::from(context.player_registry_handle().clone()).raw(),
            Handle::from(context.config_service_handle().clone()).raw(),
            server_name,
        )?;
        return Ok(Optional::of(env, registered_server)?);
    }

    fn native_get_all_servers<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
    ) -> ::std::result::Result<JObjectArray<'local, InfrarustRegisteredServer<'local>>, Self::Error>
    {
        let context = this.plugin_context_handle(env)?.into_instance();

        let servers = context.server_manager().get_all_servers();
        let mut servers_obj: Vec<InfrarustRegisteredServer> = Vec::with_capacity(servers.len());
        for (server_id, _) in servers.into_iter() {
            let server_name = JString::from_str(env, server_id.as_str())?;
            servers_obj.push(InfrarustRegisteredServer::new(
                env,
                Handle::from(context.player_registry_handle().clone()).raw(),
                Handle::from(context.config_service_handle().clone()).raw(),
                server_name,
            )?);
        }
        return servers_obj.to_jni(env);
    }

    fn native_shutdown<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        _component: ::jni::objects::JObject<'local>,
    ) -> Result<(), Self::Error> {
        let context = this.plugin_context_handle(env)?.into_instance();

        // Trigger proxy shutdown
        let _token = context.proxy_shutdown();
        return Ok(());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
    ) -> Result<(), Self::Error> {
        // Free the handle memory
        this.plugin_context_handle(env)?.delete_handle();
        Ok(())
    }

    fn native_get_event_manager<'local>(
        env: &mut ::jni::Env<'local>,
        class: ::jni::objects::JClass<'local>,
        arg0: ::jni::sys::jlong,
    ) -> ::std::result::Result<
        crate::java::generated::dev::infrarust::event::InfrarustEventManager<'local>,
        Self::Error,
    > {
        todo!()
    }

    fn native_get_scheduler<'local>(
        env: &mut ::jni::Env<'local>,
        class: ::jni::objects::JClass<'local>,
        arg0: ::jni::sys::jlong,
    ) -> ::std::result::Result<
        crate::java::generated::dev::infrarust::scheduler::InfrarustScheduler<'local>,
        Self::Error,
    > {
        todo!()
    }

    fn native_match_server<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustServer<'local>,
        arg0: ::jni::objects::JString<'local>,
    ) -> ::std::result::Result<
        ::jni::objects::JObjectArray<
            'local,
            crate::java::generated::dev::infrarust::proxy::server::InfrarustRegisteredServer<
                'local,
            >,
        >,
        Self::Error,
    > {
        todo!()
    }
}
