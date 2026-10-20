use crate::java::{
    generated::{
        com,
        dev::infrarust::command::{
            InfrarustCommandManager, InfrarustCommandManagerAPI,
            InfrarustCommandManagerNativeInterface,
        },
    },
    handle::NewTypeHandle,
};

impl InfrarustCommandManagerNativeInterface for InfrarustCommandManagerAPI {
    type Error = jni::errors::Error;

    fn native_register<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustCommandManager<'local>,
        meta: crate::java::generated::com::velocitypowered::api::command::CommandMeta<'local>,
        command: crate::java::generated::com::velocitypowered::api::command::Command<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context = this.plugin_context_handle(env)?.into_instance();
        let command_manager = plugin_context.command_manager();

        let name = meta.name(env)?;
        let aliases = meta.aliases(env)?;
        let aliases: Vec<&str> = aliases.iter().map(|x| &**x).collect();
        let handler = command.into_command_handler(env)?;
        command_manager.register(&name, &aliases, "", Box::new(handler));
        return Ok(());
    }

    fn native_unregister<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustCommandManager<'local>,
        meta: crate::java::generated::com::velocitypowered::api::command::CommandMeta<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context = this.plugin_context_handle(env)?.into_instance();
        let command_manager = plugin_context.command_manager();

        let name = meta.name(env)?;
        command_manager.unregister(&name);
        return Ok(());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustCommandManager<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context_handle = this.plugin_context_handle(env)?;
        plugin_context_handle.delete_handle();
        return Ok(());
    }
}
