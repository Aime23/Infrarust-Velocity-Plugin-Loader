use std::{
    fmt::{Debug, Display},
    pin::Pin,
};

use infrarust_api::event::{EventPriority, bus::EventBusExt};
use jni::{
    Env,
    objects::JObject,
    refs::{Global, Weak},
    sys::_jobject,
    vm::JavaVM,
};

use crate::java::{
    generated::{
        com::velocitypowered::api::event::{
            connection::{DisconnectEvent, PostLoginEvent, PreLoginEvent},
            player::{KickedFromServerEvent, PlayerChatEvent, ServerConnectedEvent},
            proxy::ProxyInitializeEvent,
        },
        dev::infrarust::event::{
            InfrarustEventManager, InfrarustEventManagerAPI, InfrarustEventManagerNativeInterface,
        },
    },
    handle::NewTypeHandle,
    implementation::dev::infrarust::events::{ApplyEventResult, TryFromInfrarustEvent},
};

macro_rules! register_events {
    ($env:expr, $global_ref:expr, $plugin_context:expr, $handler:tt, [ $( ($java_event:ty, $rust_event:ty) ),* ]) => {
        $(
            if let Some(global_ref) = $global_ref.clone_in_jvm($env)? {
                $plugin_context.event_bus().subscribe(
                    EventPriority::NORMAL,
                    move |event: &mut $rust_event| {
                        Self::$handler::<$java_event, $rust_event>(event, &global_ref);
                        return;
                    },
                );
            }
        )*
    };
}

impl InfrarustEventManagerNativeInterface for InfrarustEventManagerAPI {
    type Error = jni::errors::Error;

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustEventManager<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context_handle = this.plugin_context_handle(env)?;
        plugin_context_handle.delete_handle();
        Ok(())
    }

    fn native_initialize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustEventManager<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        let plugin_context = this.plugin_context_handle(env)?.into_instance();
        let global_ref = env.new_weak_ref(this)?;
        register_events!(
            env,
            global_ref,
            plugin_context,
            handle_event,
            [
                (
                    ProxyInitializeEvent,
                    infrarust_api::events::ProxyInitializeEvent
                ),
                (
                    ServerConnectedEvent,
                    infrarust_api::events::ServerConnectedEvent
                ),
                (DisconnectEvent, infrarust_api::events::DisconnectEvent) // (PostLoginEvent, infrarust_api::events::PostLoginEvent) // Infrarust has not yet registered the player when firing this event, however, Velocity needs Player
            ]
        );
        register_events!(
            env,
            global_ref,
            plugin_context,
            handle_event_blocking,
            [
                (PreLoginEvent, infrarust_api::events::PreLoginEvent),
                (PlayerChatEvent, infrarust_api::events::ChatMessageEvent),
                (
                    KickedFromServerEvent,
                    infrarust_api::events::KickedFromServerEvent
                )
            ]
        );
        Ok(())
    }
}

impl InfrarustEventManagerAPI {
    fn handle_event<'local, T, E>(
        event: &mut E,
        this: &Weak<InfrarustEventManager<'static>>,
    ) -> ::std::result::Result<(), jni::errors::Error>
    where
        E: infrarust_api::event::Event,
        T: AsRef<JObject<'local>> + TryFromInfrarustEvent<'local, E>,
    {
        let jvm = JavaVM::singleton()?;

        return jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
            // For some reason, the compiler thinks that env escapes the closure throught the java_event.
            // Use an unsafe transmute to force a "longer" lifetime to work around this.
            let env: &mut Env<'local> = unsafe { std::mem::transmute(env) };
            let this = this
                .upgrade_local(env)?
                .ok_or(jni::errors::Error::ObjectFreed)?;
            // Not cloning, but using the same handle as the event_manager because else
            // I would need to redo every event in java to implements cleanup logic.
            // Since the EventManager lifetime is tied to the ProxyServer it will outlive every event, it's alright.
            let plugin_context_handle = this.plugin_context_handle(env)?;
            let java_event =
                T::try_from_infrarust_event(event, env, plugin_context_handle).unwrap(); //TODO: Error handling
            this.fire(env, &java_event)?;
            Ok(())
        });
    }

    fn handle_event_blocking<'local, T, E>(
        event: &mut E,
        this: &Weak<InfrarustEventManager<'static>>,
    ) -> ::std::result::Result<(), jni::errors::Error>
    where
        E: infrarust_api::event::Event,
        T: AsRef<JObject<'local>> + TryFromInfrarustEvent<'local, E> + ApplyEventResult<'local, E>,
    {
        let jvm = JavaVM::singleton()?;

        return jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
            // For some reason, the compiler thinks that env escapes the closure throught the java_event.
            // Use an unsafe transmute to force a "longer" lifetime to work around this.
            let env: &mut Env<'local> = unsafe { std::mem::transmute(env) };
            let this = this
                .upgrade_local(env)?
                .ok_or(jni::errors::Error::ObjectFreed)?;
            // Not cloning, but using the same handle as the event_manager because else
            // I would need to redo every event in java to implements cleanup logic.
            // Since the EventManager lifetime is tied to the ProxyServer it will outlive every event, it's alright.
            let plugin_context_handle = this.plugin_context_handle(env)?;
            let java_event =
                T::try_from_infrarust_event(event, env, plugin_context_handle).unwrap(); //TODO: Error handling
            let future = this.fire(env, &java_event)?;
            // Block the thread till fire is done
            future.join(env)?;
            T::apply_event_result(&java_event, env, event)?;
            Ok(())
        });
    }
}
