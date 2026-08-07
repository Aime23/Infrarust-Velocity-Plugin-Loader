pub mod adventure {
    pub mod text {
        use jni::objects::JString;
use log::error;

        use crate::java::{
            ToJni, TryFromJni, generated::{dev::infrarust::InfrarustUtils, net::kyori::adventure::text::Component},
        };

        impl<'local> ToJni<'local> for infrarust_api::types::Component {
            type Kind = Component<'local>;

            fn to_jni(self, env: &mut jni::Env<'local>) -> Result<Self::Kind, jni::errors::Error> {
                let json = self.to_json().to_jni(env)?;
                return InfrarustUtils::deserialize_component(env, json);
            }
        }


        impl<'local> TryFromJni<'local, Component<'local>> for infrarust_api::types::Component {
            fn try_from_jni(env: &mut jni::Env<'local>, value: Component<'local>) -> Result<Self, jni::errors::Error> {

                let json = InfrarustUtils::serialize_component(env, value)?;
                let json = json.try_to_string(env)?;
                match infrarust_api::types::Component::from_json(&json) {
                    Ok(component) => return Ok(component),
                    Err(err) => {
                        error!("Unable to parse component from json: {}. Passing a default value instead", err);
                        return Ok(infrarust_api::types::Component::default())
                    },
                }
            }
        }
    }
}
