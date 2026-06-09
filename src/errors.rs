use infrarust_api::loader::LoaderError;

pub struct CustomError {
    pub reason: String,
}

impl From<jni::errors::Error> for CustomError {
    fn from(value: jni::errors::Error) -> Self {
        unreachable!()
    }
}
