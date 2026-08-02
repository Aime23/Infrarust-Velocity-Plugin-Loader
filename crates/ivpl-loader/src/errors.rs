
pub struct CustomError {
    pub reason: String,
}

impl From<jni::errors::Error> for CustomError {
    fn from(value: jni::errors::Error) -> Self {
        CustomError {
            reason: value.to_string(),
        }
    }
}
