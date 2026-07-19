use std::sync::Mutex;

use infrarust_api::{error::PluginError, plugin::Plugin};
use jni::{refs::Global, vm::JavaVM};

use crate::{
    java::generated::com::velocitypowered::{
        api::plugin::PluginContainer, proxy::plugin::loader::VelocityPluginContainer,
    },
    plugin::plugin_candidate::PluginCandidate,
};

pub struct VelocityPlugin {
    candidate: PluginCandidate,
    java_plugin: Global<PluginContainer<'static>>,
    jvm: Mutex<JavaVM>,
}

impl Plugin for VelocityPlugin {
    fn metadata(&self) -> infrarust_api::prelude::PluginMetadata {
        return self.candidate.metadata().clone().into();
    }

    fn on_enable<'a>(
        &'a self,
        ctx: &'a dyn infrarust_api::prelude::PluginContext,
    ) -> infrarust_api::prelude::BoxFuture<'a, Result<(), infrarust_api::prelude::PluginError>>
    {
        return Box::pin(async move {
            println!("Enabled plugin {}", self.candidate.metadata().id);
            Ok(())
        });
    }
}

impl VelocityPlugin {
    pub fn new(
        candidate: PluginCandidate,
        java_plugin: Global<PluginContainer>,
        jvm: JavaVM,
    ) -> Self {
        Self {
            candidate,
            java_plugin,
            jvm: Mutex::new(jvm),
        }
    }
}
