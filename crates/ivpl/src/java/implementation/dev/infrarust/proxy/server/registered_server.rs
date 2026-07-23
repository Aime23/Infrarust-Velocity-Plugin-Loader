use infrarust_api::{
    prelude::{ConfigService, PlayerRegistry},
    types::ServerId,
};
use jni::objects::JString;

use crate::{
    handle::Handle,
    java::{
        ToJni,
        generated::{
            com::velocitypowered::api::proxy::server::ServerInfo,
            dev::infrarust::proxy::{
                InfrarustPlayer,
                server::{
                    InfrarustRegisteredServer, InfrarustRegisteredServerAPI,
                    InfrarustRegisteredServerNativeInterface,
                },
            },
        },
        handle::NewTypeHandle,
        implementation::java::net::inet_socket_address::InetSocketAddress,
    },
};

impl InfrarustRegisteredServerNativeInterface for InfrarustRegisteredServerAPI {
    type Error = jni::errors::Error;

    fn native_get_server_info<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustRegisteredServer<'local>,
    ) -> ::std::result::Result<ServerInfo<'local>, Self::Error> {
        let config_service = this.config_service_handle(env)?.into_instance();
        let server_id = ServerId::new(this.server_id(env)?.to_string());

        if let Some(config) = config_service.get_server_config(&server_id) {
            let name = JString::from_str(env, server_id.as_str())?;
            let hostname = JString::from_str(env, &config.addresses[0].host)?;
            let address = InetSocketAddress::new(env, hostname, config.addresses[0].port as i32)?;
            return ServerInfo::new(env, name, address);
        }
        return Ok(ServerInfo::null());
    }

    fn native_get_players_connected<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustRegisteredServer<'local>,
    ) -> ::std::result::Result<
        ::jni::objects::JObjectArray<'local, InfrarustPlayer<'local>>,
        Self::Error,
    > {
        let player_registry = this.player_registry_handle(env)?.into_instance();
        let server_id = ServerId::new(this.server_id(env)?.to_string());

        let players = player_registry.get_players_on_server(&server_id);
        return players.to_jni(env);
    }

    fn native_ping<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustRegisteredServer<'local>,
        ping_options: ::jni::objects::JObject<'local>,
    ) -> ::std::result::Result<::jni::objects::JObject<'local>, Self::Error> {
        todo!()
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustRegisteredServer<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        this.config_service_handle(env)?.delete_handle();
        this.player_registry_handle(env)?.delete_handle();
        Ok(())
    }
}
