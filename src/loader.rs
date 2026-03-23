use infrarust_api::{
    event::BoxFuture,
    plugin::{Plugin, PluginMetadata},
};
use infrarust_core::plugin::{LoaderError, PluginContextFactory, PluginLoader};

struct PluginLoaderVelocity;

impl PluginLoader for PluginLoaderVelocity {
    fn name(&self) -> &str {
        return "plugin-loader-velocity";
    }

    fn discover<'a>(
        &'a self,
        plugin_dir: &'a std::path::Path,
    ) -> BoxFuture<'a, Result<Vec<PluginMetadata>, LoaderError>> {
        todo!()
    }

    fn load<'a>(
        &'a self,
        plugin_id: &'a str,
        context_factory: &'a dyn PluginContextFactory,
    ) -> BoxFuture<'a, Result<Box<dyn Plugin>, LoaderError>> {
        todo!()
    }

    fn unload<'a>(
        &'a self,
        plugin_id: &'a str,
    ) -> BoxFuture<'a, Result<(), LoaderError>> {
        todo!()
    }
}
