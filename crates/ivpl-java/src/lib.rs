use ivpl_java_classloader::{BYTE_ARRAY_CLASS_LOADER_BYTES, ByteArrayClassLoader};
use jni::{
    jni_str,
    objects::{JByteArray, JClassLoader},
};

use ivpl_java_binding::{
    io::byte_array_input_stream::ByteArrayInputStream, util::jar::jar_input_stream::JarInputStream,
};

pub const JAR_BYTES: &[i8] =
    unsafe { std::mem::transmute::<&[u8], &[i8]>(include_bytes!(env!("JAR_PATH"))) };

pub fn setup_class_loader<'local>(
    env: &mut jni::Env<'local>,
) -> jni::errors::Result<ByteArrayClassLoader<'local>> {
    let system_class_loader = JClassLoader::get_system_class_loader(env)?;

    env.define_class(
        Some(jni_str!("ByteArrayClassLoader")),
        &system_class_loader,
        BYTE_ARRAY_CLASS_LOADER_BYTES,
    )?;

    let value = JByteArray::new(env, JAR_BYTES.len())?;
    value.set_region(env, 0, JAR_BYTES)?;
    return ByteArrayClassLoader::new(env, value, system_class_loader);
}
