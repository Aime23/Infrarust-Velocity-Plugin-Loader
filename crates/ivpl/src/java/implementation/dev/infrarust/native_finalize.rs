use crate::java::generated::dev::infrarust::{
    NativeFinalize, NativeFinalizeAPI, NativeFinalizeNativeInterface,
};

impl NativeFinalizeNativeInterface for NativeFinalizeAPI {
    type Error = jni::errors::Error;

    fn native_finalize<'local>(
        _env: &mut ::jni::Env<'local>,
        _this: NativeFinalize<'local>,
    ) -> ::std::result::Result<(), Self::Error> {
        Ok(())
    }
}
