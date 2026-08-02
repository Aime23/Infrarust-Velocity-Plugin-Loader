use std::{env, path::PathBuf};

fn main() {
    javac::Build::new()
        .file("src/ByteArrayClassLoader.java")
        .compile();

    println!("cargo:rerun-if-changed=src/ByteArrayClassLoader.java",);

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap_or("./".into()));
    println!(
        "cargo:rustc-env=BYTE_ARRAY_CLASS_LOADER_PATH={}",
        out_dir
            .join("javac-build/classes/ByteArrayClassLoader.class")
            .display()
    );
}
