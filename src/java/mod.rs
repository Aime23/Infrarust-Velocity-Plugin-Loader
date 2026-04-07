use jni::{
    objects::{JObject, JObjectArray},
    refs::Reference,
};

pub mod generated;
pub mod implementation;
pub mod handle;

pub trait TryFromJni<'local, T>: Sized {
    fn try_from_jni(env: &mut ::jni::Env<'local>, value: T) -> Result<Self, jni::errors::Error>;
}

pub trait ToJni<'local> {
    type Kind: Reference + Default + Into<JObject<'local>> + AsRef<JObject<'local>> + 'local;
    fn to_jni(self, env: &mut ::jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error>;
}

impl<'local, T, I> ToJni<'local> for Vec<T>
where
    T: ToJni<'local, Kind = I>,
    I: Reference<Kind<'local> = I> + Default + AsRef<JObject<'local>> + AsRef<I> + 'local,
{
    type Kind = JObjectArray<'local, I>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let array_list = JObjectArray::<I>::new(env, self.len(), I::null())?;
        for (index, item) in self.into_iter().enumerate() {
            let obj = item.to_jni(env)?;
            array_list.set_element(env, index, obj)?;
        }
        return Ok(array_list);
    }
}

impl<'local> ToJni<'local> for Vec<JObject<'local>> {
    type Kind = JObjectArray<'local, JObject<'local>>;
    fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
        let array_list = JObjectArray::<JObject<'local>>::new(env, self.len(), JObject::null())?;
        for (index, item) in self.into_iter().enumerate() {
            array_list.set_element(env, index, item)?;
        }
        return Ok(array_list);
    }
}

trait ToJniArray<'local, T>
where
    T: Reference,
{
    fn to_jni(
        self,
        env: &mut ::jni::Env<'local>,
    ) -> Result<JObjectArray<'local, T>, jni::errors::Error>;
}

impl<'local, T> ToJniArray<'local, T> for Vec<T>
where
    T: Reference<Kind<'local> = T> + AsRef<T>,
{
    fn to_jni(
        self,
        env: &mut jni::Env<'local>,
    ) -> Result<JObjectArray<'local, T>, jni::errors::Error> {
        let array_list = JObjectArray::<T>::new(env, self.len(), T::null())?;
        for (index, item) in self.into_iter().enumerate() {
            array_list.set_element(env, index, item)?;
        }
        return Ok(array_list);
    }
}
