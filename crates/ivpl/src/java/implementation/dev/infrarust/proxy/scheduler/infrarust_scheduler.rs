use std::time::Duration;

use infrarust_api::services::TaskHandle;
use ivpl_java_binding::util::uuid::UUID;
use jni::{Env, refs::Weak, vm::JavaVM};
use tokio::runtime::Handle;

use crate::java::{
    ToJni, TryFromJni, generated::dev::infrarust::scheduler::{
        InfrarustScheduler, InfrarustSchedulerAPI, InfrarustSchedulerNativeInterface,
    }, handle::{NewTypeHandle, PluginContextHandle},
};

impl InfrarustSchedulerNativeInterface for InfrarustSchedulerAPI {
    type Error = jni::errors::Error;

    fn clone_handle<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        let plugin_context_handle = PluginContextHandle::from_instance(
            this.plugin_context_handle(env)?.into_instance().clone(),
        );
        return Ok(plugin_context_handle.into());
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        this.plugin_context_handle(env)?.delete_handle();
        return Ok(());
    }

    fn native_register_task<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustScheduler<'local>,
        uuid: ivpl_java_binding::util::uuid::UUID<'local>,
        delay: ::jni::sys::jlong,
        repeat: ::jni::sys::jlong,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        let plugin_context = this.plugin_context_handle(env)?.into_instance();
        let scheduler = plugin_context.scheduler();
        let server = this.server(env)?;
        let tokio_handle = server.runtime_handle(env)?.into_instance();

        let uuid = uuid::Uuid::try_from_jni(env, uuid)?;

        let global_ref = env.new_weak_ref(this)?;
        let task: Box<dyn Fn() + Send + Sync> = Box::new(move || {
            Self::handle_task(uuid, &global_ref);
        });

        let _guard = tokio_handle.enter();
        let handle = match (delay, repeat) {
            (0, 1..) => scheduler.interval(Duration::from_millis(repeat as u64), task),
            (1.., 0) => scheduler.delay(Duration::from_millis(delay as u64), task),
            (1.., 1..) => scheduler.interval_with_delay(Duration::from_millis(repeat as u64), Duration::from_millis(delay as u64), task),
            (_, _) => scheduler.delay(Duration::from_millis(0), task),
        };

        return Ok(i64::from_le_bytes(handle.as_u64().to_le_bytes()));
    }

    fn native_unregister_task<'local>(env: &mut ::jni::Env<'local> ,this: InfrarustScheduler<'local> ,arg0: ::jni::sys::jlong) ->  ::std::result::Result<(),Self::Error>  {
        let plugin_context = this.plugin_context_handle(env)?.into_instance();
        let scheduler = plugin_context.scheduler();
        scheduler.cancel(TaskHandle::new(arg0 as u64));
        return Ok(());
    }
}

impl InfrarustSchedulerAPI {
    fn handle_task<'local>(
        uuid: uuid::Uuid,
        this: &Weak<InfrarustScheduler<'static>>,
    ) -> ::std::result::Result<(), jni::errors::Error>
    {
        let jvm = JavaVM::singleton()?;

        return jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
            let this = this
                .upgrade_local(env)?
                .ok_or(jni::errors::Error::ObjectFreed)?;
            let uuid = uuid.to_jni(env)?;
            this.fire_task(env, uuid)?;
            Ok(())
        });
    }
}
