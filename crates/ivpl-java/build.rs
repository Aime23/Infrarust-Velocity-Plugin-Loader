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
    println!("cargo:rerun-if-changed={}", java_dir.join("velocity").display());
    println!("cargo:rerun-if-changed={}", java_dir.join("pom.xml").display());

    let mvn_output = Command::new("mvn")
        .args(["package"])
        .current_dir(&java_dir)
        .output()
        .expect("Failed to run `mvn package`; ensure Maven is installed and the project builds");
    if !mvn_output.status.success() {
        eprintln!(
            "Maven build failed:\n{}\n{}",
            String::from_utf8_lossy(&mvn_output.stderr),
            String::from_utf8_lossy(&mvn_output.stdout)
        );
        panic!("Maven build failed");
    }

    let target_dir = java_dir.join("target");
    let jar_file = target_dir
        .read_dir()
        .expect("Cannot read target dir")
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "jar")
                .unwrap_or(false)
        })
        .expect("No .jar file found in target/; check pom.xml and build output");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap_or("./".into()));
    let dest = out_dir.join("lib.jar");
    fs::copy(jar_file.path(), &dest).expect("Failed to copy JAR to OUT_DIR");

    println!("cargo:rustc-env=JAR_PATH={}", dest.display());
}
