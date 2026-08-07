use std::pin::Pin;

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
        com::velocitypowered::api::event::player::ServerConnectedEvent,
        dev::infrarust::event::{
            InfrarustEventManager, InfrarustEventManagerAPI, InfrarustEventManagerNativeInterface,
        },
    },
    handle::NewTypeHandle,
    implementation::dev::infrarust::events::TryFromInfrarustEvent,
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
        plugin_context.event_bus().subscribe(
            EventPriority::NORMAL,
            move |event: &mut infrarust_api::events::ServerConnectedEvent| {
                Self::handle_event::<
                    ServerConnectedEvent,
                    infrarust_api::events::ServerConnectedEvent,
                >(event, &global_ref);
                return;
            },
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
            //
            let env: &mut Env<'local> = unsafe { std::mem::transmute(env) };
            let this = this
                .upgrade_local(env)?
                .ok_or(jni::errors::Error::ObjectFreed)?;
            // Not cloning, but using the same handle as the event_manager because else I would need to redo every event to implements cleanup logic
            let plugin_context_handle = this.plugin_context_handle(env)?;

            let java_event =
                T::try_from_infrarust_event(event, env, plugin_context_handle).unwrap();
            this.fire(env, java_event)?;
            Ok(())
        });
    }
}
