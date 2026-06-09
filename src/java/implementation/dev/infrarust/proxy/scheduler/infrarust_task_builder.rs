use crate::java::generated::dev::infrarust::scheduler::{
    InfrarustTaskBuilder, InfrarustTaskBuilderAPI, InfrarustTaskBuilderNativeInterface,
};

impl InfrarustTaskBuilderNativeInterface for InfrarustTaskBuilderAPI {
    type Error = jni::errors::Error;

    fn clone_handle<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustTaskBuilder<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        todo!()
    }

    fn native_finalize<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustTaskBuilder<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        todo!()
    }

    fn native_schedule<'local>(
        env: &mut ::jni::Env<'local>,
        this: InfrarustTaskBuilder<'local>,
    ) -> ::std::result::Result<::jni::sys::jlong, Self::Error> {
        todo!()
    }
}
