use infrarust_api::{plugin::PluginContext, types::PlayerId};
use jni::{objects::JObject, refs::Reference};

use crate::java::handle::PluginContextHandle;

pub mod connection;

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

// pub trait TryIntoJavaEvent<'local, T>: Sized
// where
//     T: AsRef<JObject<'local>>,
// {
//     fn try_into_java_event(
//         &self,
//         env: &mut ::jni::Env<'local>,
//         plugin_context_handle: PluginContextHandle,
//     ) -> Result<T, TryFromInfrarustEventError>;
// }

// impl<'local, I, J> TryIntoJavaEvent<'local, J> for I
// where
//     J: TryFromInfrarustEvent<'local, I> + AsRef<JObject<'local>>,
//     I: infrarust_api::event::Event,
// {
//     fn try_into_java_event(
//         &self,
//         env: &mut jni::Env<'local>,
//         plugin_context_handle: PluginContextHandle,
//     ) -> Result<J, TryFromInfrarustEventError> {
//         J::try_from_infrarust_event(&self, env, plugin_context_handle)
//     }
// }
