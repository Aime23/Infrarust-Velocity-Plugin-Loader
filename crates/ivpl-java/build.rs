use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let java_dir =
        PathBuf::from(env::var_os("JAVA_PROJECT_DIR").unwrap_or("infrarust-velocity".into()));
    if !java_dir.is_dir() {
        panic!(
            "Java project directory not found at {}; set JAVA_PROJECT_DIR to point to it",
            java_dir.display()
        );
    }

    println!("cargo:rerun-if-changed={}", java_dir.join("src").display());
    println!(
        "cargo:rerun-if-changed={}",
        java_dir.join("velocity/source/proxy").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        java_dir.join("pom.xml").display()
    );

    let mvn_output = Command::new("mvn")
        .args(["package"])
        .current_dir(&java_dir)
        .output()
        .expect("Failed to run `mvn package`; ensure Maven is installed and the project builds");
    if !mvn_output.status.success() {
        println!(
            "cargo::error=Maven build failed:\n{}\n{}",
            String::from_utf8_lossy(&mvn_output.stderr),
            String::from_utf8_lossy(&mvn_output.stdout)
        );
        panic!("Maven build failed");
    }

    let target_dir = java_dir.join("target");
    let jar_name = env::var_os("JAVA_ARTIFACT_NAME").unwrap_or("shaded.jar".into());
    let jar_file = target_dir.join(&jar_name);

    if !jar_file.exists() {
        println!(
            "cargo::error={} not found int target folder",
            jar_name.to_string_lossy()
        );
        panic!("Output artefact not found")
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap_or("./".into()));
    let dest = out_dir.join("lib.jar");
    fs::copy(jar_file, &dest).expect("Failed to copy JAR to OUT_DIR");

    println!("cargo:rustc-env=JAR_PATH={}", dest.display());
}
