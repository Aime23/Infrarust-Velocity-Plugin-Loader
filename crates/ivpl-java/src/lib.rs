use jni::{
    objects::{JByteArray, JClassLoader},
    strings::JNIString,
};

use ivpl_java_binding::{
    io::byte_array_input_stream::ByteArrayInputStream, util::jar::jar_input_stream::JarInputStream,
};

pub const JAR_BYTES: &[i8] =
    unsafe { std::mem::transmute::<&[u8], &[i8]>(include_bytes!(env!("JAR_PATH"))) };

pub const CLASS_EXT: &str = ".class";

pub fn load_jar(env: &mut jni::Env) -> jni::errors::Result<()> {
    let byte_array = JByteArray::new(env, JAR_BYTES.len())?;
    byte_array.set_region(env, 0, JAR_BYTES as &[i8])?;
    let bais = ByteArrayInputStream::new(env, byte_array)?;
    let jis = JarInputStream::new(env, bais)?;

    loop {
        let entry = jis.get_next_jar_entry(env)?;
        if entry.is_null() {
            break;
        }
        let name = entry.get_name(env)?;
        let name = name.try_to_string(env)?;
        let ext_pos = name.rfind(CLASS_EXT).unwrap_or_default();
        if (name.len() > CLASS_EXT.len() && ext_pos == name.len() - CLASS_EXT.len()) {
            let class_bytes = jis.as_input_stream().read_all_bytes(env)?;
            let class_bytes = env.convert_byte_array(class_bytes)?;
            let mut class_name = name.clone();
            class_name.replace_range(ext_pos..name.len(), "");
            let system_class_loader = JClassLoader::get_system_class_loader(env)?;
            let jclass_name = JNIString::new(class_name);
            let cls = env.define_class(Some(jclass_name), system_class_loader, &class_bytes)?;
            let loaded_class_name = {
                let loaded_class_name = cls.get_name(env)?;
                loaded_class_name.try_to_string(env)?
            };
            println!("Loaded {} into {}", &name, &loaded_class_name);
        }
    }
    return Ok(());
}
