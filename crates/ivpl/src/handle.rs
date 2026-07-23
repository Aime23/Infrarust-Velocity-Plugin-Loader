use jni::sys::jlong;

use std::{
    any::{self, TypeId},
    collections::HashMap,
    mem::ManuallyDrop,
    panic,
    sync::{LazyLock, RwLock},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Handle(jlong);

#[derive(Debug)]
struct HandleMetadata {
    type_id: TypeId,
    type_name: &'static str,
}

impl HandleMetadata {
    pub fn of<T: 'static>() -> Self {
        return Self {
            type_id: TypeId::of::<T>(),
            type_name: any::type_name::<T>(),
        };
    }
}

static HANDLE_MEMORY: LazyLock<RwLock<HashMap<jlong, HandleMetadata>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

impl Handle {
    pub fn from<T: Sized + 'static>(value: T) -> Self {
        let metadata = HandleMetadata::of::<T>();
        let handle = Self(Box::into_raw(Box::new(value)) as jlong);

        HANDLE_MEMORY
            .write()
            .expect("Unable to acquire write lock for HANDLE_MEMORY")
            .insert(handle.0, metadata);

        return handle;
    }

    pub fn into<T: Sized + 'static>(self) -> ManuallyDrop<&'static mut T> {
        let cast_type_name = any::type_name::<T>();
        match HANDLE_MEMORY
            .read()
            .expect("Unable to acquire read lock for HANDLE_MEMORY")
            .get(&self.0)
        {
            Some(metadata) => {
                if metadata.type_id != TypeId::of::<T>() {
                    panic!(
                        "Tried to cast an handle of type {} into {}",
                        metadata.type_name, cast_type_name
                    )
                }
                assert_ne!(self.0, 0, "Invalid handle value");
                let ptr = self.0 as *mut T;
                return unsafe { ManuallyDrop::new(&mut *ptr) };
            }
            None => panic!("Tried to cast an unknown handle in {}", cast_type_name),
        }
    }

    pub fn delete<T: Sized + 'static>(self) {
        let cast_type_name = any::type_name::<T>();
        let mut lock = HANDLE_MEMORY
            .write()
            .expect("Unable to acquire read lock for HANDLE_MEMORY");
        match lock.get(&self.0) {
            Some(metadata) => {
                if metadata.type_id != TypeId::of::<T>() {
                    panic!(
                        "Tried to cast an handle of type {} into {}",
                        metadata.type_name, cast_type_name
                    )
                }
                lock.remove(&self.0);
                if let Err(_) = panic::catch_unwind(|| {
                    let _ = self.0 as *mut T;
                }) {
                    println!("Unable to deallocate an handle of type {}", cast_type_name);
                }
                return;
            }
            None => panic!("Tried to cast an unknown handle in {}", cast_type_name),
        }
    }

    /// Get the raw handle value for use in native methods
    pub fn raw(&self) -> jlong {
        self.0
    }

    pub fn from_raw(value: jlong) -> Self {
        match HANDLE_MEMORY
            .read()
            .expect("Unable to acquire read lock for HANDLE_MEMORY")
            .get(&value)
        {
            Some(_) => Self(value),
            None => panic!("Tried to create Handle from an unknown pointer"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    struct TestStruct;
    struct OtherTestStruct;

    #[test]
    #[serial]
    fn should_work_on_valid() {
        HANDLE_MEMORY.clear_poison();
        let data = TestStruct;
        let handle = Handle::from(data);
        handle.into::<TestStruct>();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn should_panic_on_unknown_handle() {
        HANDLE_MEMORY.clear_poison();
        let handle = Handle::from_raw(100);
        handle.into::<TestStruct>();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn should_panic_on_miss_typed_handle() {
        HANDLE_MEMORY.clear_poison();
        let data = TestStruct;
        let handle = Handle::from(data);
        handle.into::<OtherTestStruct>();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn should_panic_deleting_unknown_handle() {
        HANDLE_MEMORY.clear_poison();
        let handle = Handle::from_raw(100);
        handle.delete::<TestStruct>();
    }

    #[test]
    #[serial]
    #[should_panic]
    fn should_panic_deleting_miss_typed_handle() {
        HANDLE_MEMORY.clear_poison();
        let data = TestStruct;
        let handle = Handle::from(data);
        handle.delete::<OtherTestStruct>();
    }


}
