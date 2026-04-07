use std::{mem::ManuallyDrop, sync::Arc};

use derive_more::{From, Into};
use infrarust_api::{plugin::PluginContext, prelude::{ConfigService, PlayerRegistry}};
use jni::sys::jlong;

use crate::handle::Handle;

pub trait NewTypeHandle<T>: Into<jlong> + From<jlong>
where
    T: Sized + 'static,
{
    fn into_instance(self) -> ManuallyDrop<&'static mut T> {
        Handle::from_raw(self.into()).into::<T>()
    }
    fn delete_handle(self) {
        Handle::from_raw(self.into()).delete::<T>();
    }
    fn from_instance(instance: T) -> Self {
        Self::from(Handle::from::<T>(instance).raw())
    }
}

// Define every handle new type here and in the typemap

#[derive(Debug, Into, From)]
pub struct PluginContextHandle(jlong);
impl NewTypeHandle<Box<Arc<dyn PluginContext>>> for PluginContextHandle {}


#[derive(Debug, Into, From)]
pub struct PlayerRegistryHandle(jlong);
impl NewTypeHandle<Box<Arc<dyn PlayerRegistry>>> for PlayerRegistryHandle {}

#[derive(Debug, Into, From)]
pub struct ConfigServiceHandle(jlong);
impl NewTypeHandle<Box<Arc<dyn ConfigService>>> for ConfigServiceHandle {}


#[derive(Debug, Into, From)]
pub struct PlayerHandle(jlong);
impl NewTypeHandle<Box<Arc<dyn infrarust_api::player::Player>>> for PlayerHandle {}
