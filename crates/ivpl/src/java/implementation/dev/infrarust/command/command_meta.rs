use crate::java::TryFromJni;
use jni::{
    objects::{JCollection, JList, JObjectArray, JString},
    sys::JNIEnv,
};

use crate::java::generated::com::velocitypowered::api::command::{CommandMeta, CommandMetaAPI};

impl<'local> CommandMeta<'local> {
    pub fn name<'env_local>(
        &self,
        env: &mut ::jni::Env<'env_local>,
    ) -> ::jni::errors::Result<String> {
        let aliases = self.get_aliases(env)?;
        let aliases = aliases.to_array(env)?;
        let name = aliases.get_element(env, 0)?;
        let name = env.cast_local::<JString>(name)?;
        let name = name.try_to_string(env)?;
        return Ok(name);
    }

    pub fn aliases<'env_local>(
        &self,
        env: &mut ::jni::Env<'env_local>,
    ) -> ::jni::errors::Result<Vec<String>> {
        let aliases = self.get_aliases(env)?;
        let aliases = aliases.to_array(env)?;
        let len = aliases.len(env)?;
        let mut out: Vec<String> = Vec::with_capacity(len);
        for i in 0..len {
            let element = aliases.get_element(env, i)?;
            let element = env.cast_local::<JString>(element)?;
            let element = String::try_from_jni(env, element)?;
            out.push(element);
        }
        return Ok(out);
    }
}
