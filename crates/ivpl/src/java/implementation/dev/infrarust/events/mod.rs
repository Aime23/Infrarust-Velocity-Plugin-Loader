use infrarust_api::{plugin::PluginContext, types::PlayerId};
use jni::{objects::JObject, refs::Reference};

use crate::java::handle::PluginContextHandle;

pub mod connection;
pub mod proxy;

#[non_exhaustive]
#[derive(Debug)]
pub enum TryFromInfrarustEventError {
    Unknown,
    MissingPlayer(PlayerId),
    Java(jni::errors::Error),
}

pub trait TryFromInfrarustEvent<'local, T>: Sized + AsRef<JObject<'local>> {
    fn try_from_infrarust_event(
        value: &T,
        env: &mut ::jni::Env<'local>,
        plugin_context_handle: PluginContextHandle,
    ) -> Result<Self, TryFromInfrarustEventError>;
}

pub trait ApplyEventResult<'local, T>: TryFromInfrarustEvent<'local, T> {
    fn apply_event_result(
        &self,
        env: &mut ::jni::Env<'local>,
        event: &mut T,
    ) -> jni::errors::Result<()>;
}
