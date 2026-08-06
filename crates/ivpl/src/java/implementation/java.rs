use std::{mem::ManuallyDrop, net::SocketAddr};

use ivpl_java_binding::{
    net::{inet_address::InetAddress, inet_socket_address::InetSocketAddress},
    nio::file::path::{Path, Paths},
    util::{optional::Optional, uuid::UUID},
};
use jni::{
    objects::{JByteArray, JObject, JObjectArray, JString},
    refs::Reference,
    sys::jlong,
};

use crate::java::{ToJni, TryFromJni, TryFromJniNullable};

// Uuid

impl<'local> TryFromJni<'local, UUID<'local>> for uuid::Uuid {
    fn try_from_jni(
        env: &mut jni::Env<'local>,
        value: UUID<'local>,
    ) -> Result<Self, jni::errors::Error> {
        let most_sig_bits = value.get_most_significant_bits(env)?;
        let least_sig_bits = value.get_least_significant_bits(env)?;
        Ok(Self::from_u64_pair(
            most_sig_bits as u64,
            least_sig_bits as u64,
        ))
    }
}

impl<'local> ToJni<'local> for uuid::Uuid {
    type Kind = UUID<'local>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let (most_sig, least_sig) = self.as_u64_pair();
        UUID::new(env, most_sig as jlong, least_sig as jlong)
    }
}

// Optional

impl<
    'local,
    T: Sized + Reference + Default + Into<JObject<'local>> + AsRef<JObject<'local>> + 'local,
> TryFromJni<'local, Optional<'local>> for Option<T>
{
    fn try_from_jni(
        env: &mut jni::Env<'local>,
        value: Optional<'local>,
    ) -> Result<Option<T>, jni::errors::Error> {
        let value = value.value(env)?;
        if value.is_null() {
            return Ok(None);
        }

        if std::mem::size_of::<T>() == std::mem::size_of::<JObject>() {
            return Ok(Some(unsafe {
                std::mem::transmute_copy(&ManuallyDrop::new(value))
            }));
        }

        return Err(jni::errors::Error::ParseFailed(format!(
            "Invalid call to from_jni, trying to convert a JObject to {} of size {}",
            T::class_name().to_string(),
            std::mem::size_of::<T>()
        )));
    }
}

impl<
    'local,
    T: Sized + Reference + Default + Into<JObject<'local>> + AsRef<JObject<'local>> + 'local,
> TryFromJniNullable<'local, T> for Option<T>
{
    fn try_from_jni_nullable(
        env: &mut jni::Env<'local>,
        value: T,
    ) -> Result<Option<T>, jni::errors::Error> {
        if value.is_null() {
            return Ok(None);
        }

        if std::mem::size_of::<T>() == std::mem::size_of::<JObject>() {
            return Ok(Some(unsafe {
                std::mem::transmute_copy(&ManuallyDrop::new(value))
            }));
        }

        return Err(jni::errors::Error::ParseFailed(format!(
            "Invalid call to try_from_jni_nullable, trying to convert a JObject to {} of size {}",
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

// Path

impl<'local> ToJni<'local> for &std::path::Path {
    type Kind = Path<'local>;

    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let path = JString::from_str(env, self.to_str().unwrap())?;
        let more = JObjectArray::<JString>::new(env, 0, JString::null())?;
        return Paths::get(env, path, more);
    }
}

// InetSocketAddr

impl<'local> ToJni<'local> for SocketAddr {
    type Kind = InetSocketAddress<'local>;

    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let (address_octets, port) = match self {
            SocketAddr::V4(socket_addr_v4) => (socket_addr_v4.ip().octets().to_vec(), socket_addr_v4.port()),
            SocketAddr::V6(socket_addr_v6) => (socket_addr_v6.ip().octets().to_vec(), socket_addr_v6.port()),
        };
        let address = JByteArray::new(env, address_octets.len())?;
        let address_octets: Vec<i8> = address_octets.into_iter().map(|i| i8::from_ne_bytes(i.to_ne_bytes())).collect();
        address.set_region(
            env,
            0,
            &address_octets,
        )?;
        let hostname = InetAddress::get_by_address(env, &address)?;
        return InetSocketAddress::new2(env, &hostname, port.into());
        todo!()
    }
}
