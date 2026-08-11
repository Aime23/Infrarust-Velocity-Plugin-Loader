use crate::{
    errors::CustomError,
    plugin::{plugin_candidate::PluginCandidate, velocity_plugin::VelocityPlugin},
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use infrarust_api::{
    event::BoxFuture,
    loader::{LoaderError, PluginContextFactory, PluginLoader},
    plugin::{Plugin, PluginContext, PluginMetadata},
};
use jni::{
    JNIVersion,
    refs::Global,
    vm::{InitArgsBuilder, JavaVM},
};

use ivpl::java::{
    ToJni, TryFromJniNullable, generated::{
        com::velocitypowered::api::plugin::PluginContainer, dev::infrarust::proxy::InfrarustServer,
    }, handle::{NewTypeHandle, PluginContextHandle, RuntimeHandle},
};

pub struct PluginLoaderVelocity {
    // Store discovered plugin by id to be able to load them without rescanning every file in the plugin folder
    plugin_index: Mutex<HashMap<String, PluginCandidate>>,
    jvm: Mutex<Option<JavaVM>>,
    server: Mutex<Option<Global<InfrarustServer<'static>>>>,
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
        _context_factory: &'a dyn PluginContextFactory,
    ) -> BoxFuture<'a, Result<Box<dyn Plugin>, LoaderError>> {
        // TODO: Propagate error
        return Box::pin(async move {
            let jvm = self.jvm.lock().map_err(|_err| LoaderError::LoadFailed {
                plugin_id: plugin_id.to_owned(),
                reason: "Unable to acquire read lock for JVM".to_owned(),
                source: None,
            })?;

            let jvm = jvm.as_ref().ok_or(LoaderError::LoadFailed {
                plugin_id: plugin_id.to_owned(),
                reason: "JVM not initialized".to_owned(),
                source: None,
            })?;
            let server = self.server.lock().map_err(|_err| LoaderError::LoadFailed {
                plugin_id: plugin_id.to_owned(),
                reason: "Unable to acquire read lock for InfrarustServer".to_owned(),
                source: None,
            })?;
            let server = server.as_ref().ok_or(LoaderError::LoadFailed {
                plugin_id: plugin_id.to_owned(),
                reason: "InfrarustServer not initialized".to_owned(),
                source: None,
            })?;
            let index_lock = self
                .plugin_index
                .lock()
                .map_err(|_err| LoaderError::LoadFailed {
                    plugin_id: plugin_id.to_owned(),
                    reason: "Unable to acquire read lock for plugin_index".to_owned(),
                    source: None,
                })?;

            let candidate = index_lock.get(plugin_id).unwrap();
            let plugin = jvm.attach_current_thread(|env| {
                let plugin_manager = server.plugin_manager(env)?;
                let jni_parent = candidate.path().parent().unwrap().to_jni(env)?;
                let jni_candidate = candidate.path().to_jni(env)?;
                let plugin = plugin_manager.load_plugin(env, jni_parent, jni_candidate)?;
                let plugin: Option<PluginContainer> = Option::try_from_jni_nullable(env, plugin)?;
                if let Some(plugin) = plugin {
                    let plugin = env.new_global_ref(plugin)?;
                    return Ok(plugin);
                }
                return Err(CustomError {
                    reason: "Java plugin instantiation error".to_owned(),
                });
            });
            if let Ok(plugin) = plugin {
                let ok: Box<dyn Plugin> =
                    Box::new(VelocityPlugin::new(candidate.clone(), plugin, jvm.clone()));
                return Ok(ok);
            } else {
                return Err(LoaderError::LoadFailed {
                    plugin_id: plugin_id.to_owned(),
                    reason: plugin.unwrap_err().reason,
                    source: None,
                });
            }
        });
    }

    fn unload<'a>(&'a self, _plugin_id: &'a str) -> BoxFuture<'a, Result<(), LoaderError>> {
        todo!()
    }

    fn on_load<'a>(
        &'a self,
        context_factory: &'a dyn PluginContextFactory,
    ) -> BoxFuture<'a, Result<(), LoaderError>> {
        Box::pin(async {
            self.init_jvm()?;
            self.init_and_start_infrarust_server(context_factory.create_context("VelocityLoader"))?;
            Ok(())
        })
    }
}

impl PluginLoaderVelocity {
    fn init_jvm(&self) -> Result<(), LoaderError> {
        // TODO: Allow passing those args throught config
        let args = InitArgsBuilder::new()
            .version(JNIVersion::V1_8)
            .option("-Xcheck:jni")
            .build()
            .map_err(|err| LoaderError::LoadFailed {
                plugin_id: "VelocityLoader".to_owned(),
                reason: "Unable to initilialize JVM options".to_owned(),
                source: Some(err.into()),
            })?;
        let jvm = JavaVM::new(args).map_err(|err| LoaderError::LoadFailed {
            plugin_id: "VelocityLoader".to_owned(),
            reason: "Unable to initilialize JVM".to_owned(),
            source: Some(err.into()),
        })?;

        let mut lock = self.jvm.lock().map_err(|_err| LoaderError::LoadFailed {
            plugin_id: "VelocityLoader".to_owned(),
            reason: "Unable to acquire JVM write lock".to_owned(),
            source: None,
        })?;
        *lock = Some(jvm);
        Ok(())
    }

    fn init_and_start_infrarust_server(
        &self,
        context: Arc<dyn PluginContext>,
    ) -> Result<(), LoaderError> {
        let jvm_lock = self.jvm.lock().map_err(|_err| LoaderError::LoadFailed {
            plugin_id: "VelocityLoader".to_owned(),
            reason: "Unable to acquire JVM read lock".to_owned(),
            source: None,
        })?;

        let mut server_lock = self.server.lock().map_err(|_err| LoaderError::LoadFailed {
            plugin_id: "VelocityLoader".to_owned(),
            reason: "Unable to acquire server write lock".to_owned(),
            source: None,
        })?;

        jvm_lock
            .as_ref()
            .unwrap()
            .attach_current_thread(|env| -> jni::errors::Result<()> {
                let loader = ivpl_java::setup_class_loader(env)?;
                let loader = loader.as_class_loader();
                ivpl::java::generated::jni_init(env, &jni::refs::LoaderContext::Loader(&loader))?;

                let handle = PluginContextHandle::from_instance(Box::new(context));
                let tokio_handle = tokio::runtime::Handle::current();
                let tokio_handle = RuntimeHandle::from_instance(Box::new(tokio_handle));
                let server = InfrarustServer::new(env, handle, tokio_handle)?;
                let server = env.new_global_ref(server)?;
                *server_lock = Some(server);
                Ok(())
            })
            .map_err(|err| LoaderError::LoadFailed {
                plugin_id: "".to_owned(),
                reason: err.to_string(),
                source: Some(Box::new(err)),
            })?;
        Ok(())
    }
    pub fn new() -> Self {
        Self {
            jvm: Mutex::new(None),
            server: Mutex::new(None),
            plugin_index: Mutex::new(HashMap::new()),
        }
    }
}
