use std::{any::{Any, TypeId}, mem::ManuallyDrop};

use jni::{bind_java_type, objects::JObject, refs::Reference};

use crate::java::{
    TryFromJni, ToJni, implementation::java::net::inet_socket_address::InetSocketAddress,
};

bind_java_type! {
    rust_type = pub Optional,
    java_type = java.util.Optional,

    constructors {
        fn of(value: JObject),
        fn empty()
    },
    methods {
        fn get() -> JObject
    },
    fields {
        value: JObject
    }
}

impl<
    'local,
    T: Sized + Reference + Default + Into<JObject<'local>> + AsRef<JObject<'local>> + 'local,
> TryFromJni<'local, Optional<'local>> for  Option<T>
{
    fn try_from_jni(env: &mut jni::Env<'local>, value: Optional<'local>) -> Result<Option<T>, jni::errors::Error> {
        let value = value.value(env)?;
        if value.is_null() {
            return Ok(None);
        }

        if std::mem::size_of::<T>() == std::mem::size_of::<JObject>() {
            return Ok(Some(unsafe { std::mem::transmute_copy(&ManuallyDrop::new(value)) }));
        }

        return Err(jni::errors::Error::ParseFailed(format!(
            "Invalid call to from_jni, trying to convert a JObject to {} of size {}",
            T::class_name().to_string(),
            std::mem::size_of::<T>()
        )));
    }
}

impl<'local, T, I> ToJni<'local> for Option<T>
where
    T: ToJni<'local, Kind = I>,
    I: jni::refs::Reference<Kind<'local> = I> + AsRef<jni::objects::JObject<'local>> + 'local,
{
    type Kind = Optional<'local>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        match self {
            Some(item) => {
                let item = item.to_jni(env)?;
                return Optional::of(env, item);
            }
            None => {
                return Optional::empty(env);
            }
        }
    }
}
