use std::clone;

use infrarust_api::command::{CommandContext, CommandHandler};
use jni::{
    Env,
    descriptors::Desc,
    objects::JClass,
    refs::{IntoAuto, Reference, Weak},
    vm::JavaVM,
};

use crate::java::{
    ToJni,
    generated::{
        com::{
            self,
            velocitypowered::api::{
                command::{
                    Command, CommandSource, InvocableCommand, InvocableCommandAPI, RawCommand,
                    RawCommandAPI, SimpleCommand,
                },
                proxy::{ConsoleCommandSource, Player},
            },
        },
        dev::infrarust::{
            command::invocation::{
                AbstractCommandInvocation, RawCommandInvocation, SimpleCommandInvocation,
            },
            console::InfrarustConsole,
        },
    },
};

pub struct JavaCommandHandler(Weak<InvocableCommand<'static>>);

impl JavaCommandHandler {
    pub fn new(weak_ref: Weak<InvocableCommand<'static>>) -> Self {
        Self(weak_ref)
    }
}

impl CommandHandler for JavaCommandHandler {
    fn execute<'a>(
        &'a self,
        ctx: infrarust_api::prelude::CommandContext,
    ) -> infrarust_api::prelude::BoxFuture<'a, ()> {
        return Box::pin(async move {
            if let Ok(jvm) = JavaVM::singleton() {
                jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
                    let command = self
                        .0
                        .upgrade_local(env)?
                        .ok_or(jni::errors::Error::ObjectFreed)?;
                    let invocation = ctx.into_invocation(env, &command)?;
                    if let Err(error) = command.execute(env, invocation) {
                        println!("Error executing command {:#?}", error);
                        env.exception_describe();
                    }

                    return Ok(());
                });
            } else {
                println!("Unable to aquiare JVM singleton");
            }
            return ();
        });
    }
}

trait IntoInvocation {
    fn into_invocation<'local, 'env_local>(
        self,
        env: &mut jni::Env<'env_local>,
        command: &InvocableCommand<'local>,
    ) -> jni::errors::Result<AbstractCommandInvocation<'env_local>>;
}

impl IntoInvocation for CommandContext {
    fn into_invocation<'local, 'env_local>(
        self,
        env: &mut jni::Env<'env_local>,
        command: &InvocableCommand<'local>,
    ) -> jni::errors::Result<AbstractCommandInvocation<'env_local>> {
        let source: CommandSource = match (self.source) {
            infrarust_api::command::CommandSource::Player(player) => {
                CommandSource::from(Player::from(player.to_jni(env)?))
            }
            infrarust_api::command::CommandSource::Console(permission_checker) => {
                CommandSource::from(ConsoleCommandSource::from(InfrarustConsole::new(env)?))
            }
            _ => todo!(),
        };
        if (env.is_instance_of(
            command,
            &*RawCommand::lookup_class(env, &jni::refs::LoaderContext::None)?,
        )?) {
            let whitespace_index = self.raw.find(char::is_whitespace);
            let (alias, arguments) = if let Some(whitespace_index) = whitespace_index {
                self.raw.split_at(whitespace_index)
            } else {
                (self.raw.as_ref(), "")
            };
            println!(
                "Making RawCommandInvocation with {} and {}",
                alias, arguments
            );
            let alias = alias.to_jni(env)?;
            let arguments = arguments.to_jni(env)?;
            return Ok(AbstractCommandInvocation::from(RawCommandInvocation::new(
                env, source, alias, arguments,
            )?));
        } else if (env.is_instance_of(
            command,
            &*SimpleCommand::lookup_class(env, &jni::refs::LoaderContext::None)?,
        )?) {
            let whitespace_index = self.raw.find(char::is_whitespace);
            let (alias, _) = if let Some(whitespace_index) = whitespace_index {
                self.raw.split_at(whitespace_index)
            } else {
                (self.raw.as_ref(), "")
            };
            println!(
                "Making SimpleCommandInvocation with {} and {:#?}",
                alias, &self.args
            );
            let alias = alias.to_jni(env)?;
            let arguments = self.args.to_jni(env)?;
            return Ok(AbstractCommandInvocation::from(
                SimpleCommandInvocation::new(env, source, alias, arguments)?,
            ));
        } else {
            let whitespace_index = self.raw.find(char::is_whitespace);
            let (alias, _) = if let Some(whitespace_index) = whitespace_index {
                self.raw.split_at(whitespace_index)
            } else {
                (self.raw.as_ref(), "")
            };
            println!(
                "Making AbstractCommandInvocation with {} and {:#?}",
                alias, &self.args
            );
            let alias = alias.to_jni(env)?;
            let arguments = self.args.to_jni(env)?;
            return Ok(AbstractCommandInvocation::from(
                SimpleCommandInvocation::new(env, source, alias, arguments)?,
            ));
        }
    }
}
