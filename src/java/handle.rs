use std::{mem::ManuallyDrop};

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
