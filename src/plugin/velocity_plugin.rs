use infrarust_api::plugin::Plugin;
use jni::refs::Global;

use crate::{
    java::generated::com::velocitypowered::{
        api::plugin::PluginContainer, proxy::plugin::loader::VelocityPluginContainer,
    },
    plugin::plugin_candidate::PluginCandidate,
};

pub struct VelocityPlugin {
    candidate: PluginCandidate,
    java_plugin: Global<PluginContainer<'static>>,
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
        todo!()
    }
}

impl VelocityPlugin {
    pub fn new(candidate: PluginCandidate, java_plugin: Global<PluginContainer>) -> Self {
        Self {
            candidate,
            java_plugin,
        }
    }
}
