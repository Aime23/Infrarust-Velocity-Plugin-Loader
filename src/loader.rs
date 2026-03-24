use std::{collections::HashMap, sync::Mutex};

use infrarust_api::{
    event::BoxFuture, loader::{LoaderError, PluginContextFactory, PluginLoader}, plugin::{Plugin, PluginMetadata}
};

use crate::plugin::plugin_candidate::PluginCandidate;

struct PluginLoaderVelocity {
    // Store discovered plugin by id to be able to load them without rescanning every file in the plugin folder
    plugin_index: Mutex<HashMap<String, PluginCandidate>>,
}

impl PluginLoader for PluginLoaderVelocity {
    fn name(&self) -> &str {
        return "plugin-loader-velocity";
    }

    fn discover<'a>(
        &'a self,
        plugin_dir: &'a std::path::Path,
    ) -> BoxFuture<'a, Result<Vec<PluginMetadata>, LoaderError>> {
        return Box::pin(async move {
            let dir =
                plugin_dir
                    .read_dir()
                    .map_err(|error| LoaderError::DirectoryNotAccessible {
                        path: plugin_dir.to_owned(),
                        source: error,
                    })?;

            let candidates: Vec<PluginCandidate> = dir
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| PluginCandidate::try_from(entry.path().as_path()).ok())
                .collect();

            let metadata = candidates
                .iter()
                .map(|candidate| candidate.metadata().clone().into())
                .collect();

            // We ignore any lock error because the plugin_index is not essential
            if let Some(mut plugin_index) = self.plugin_index.lock().ok() {
                for candidate in candidates {
                    plugin_index.insert(candidate.metadata().clone().id, candidate);
                }
            }

            return Ok(metadata);
        });
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
