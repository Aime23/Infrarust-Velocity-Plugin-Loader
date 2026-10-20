use crate::java::{
    generated::com::velocitypowered::api::command::{Command, InvocableCommand},
    implementation::dev::infrarust::command::java_command_handler::JavaCommandHandler,
};

impl<'local> Command<'local> {
    pub fn into_command_handler<'env_local>(
        self,
        env: &mut ::jni::Env<'env_local>,
    ) -> ::jni::errors::Result<JavaCommandHandler> {
        let invocable: InvocableCommand = env.cast_local::<InvocableCommand>(self)?;

        let weak_ref = env.new_weak_ref(invocable)?;
        return Ok(JavaCommandHandler::new(weak_ref));
    }
}
