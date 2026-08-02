# Infrarust Velocity Plugin Loader

A plugin loader that enables loading Java [Velocity](https://papermc.io/software/velocity) plugins into [Infrarust](https://github.com/Shadowner/Infrarust).

## Development

### Requirements

To build and develop this project, you will need the following:

* Required:
  - **Rust** 1.96.0 or higher
  - **Java** JDK 21 or higher (required for building [ivpl-java](crates/ivpl-java) and [ivpl-java-classloader](crates/ivpl-java-classloader))
  - **Maven** 3.6.x or higher (used for the ProxyServer implementation in [ivpl-java](crates/ivpl-java))
  - **Gradle** (used for building Velocity from source)
* Optional:
  - **jbindgen** use the [provided fork](vendor/jbindgen) (for generating Rust bindings from Java classes)

### Building the project

Even though this project involve Java components, the build process is sufficiently integrated so that everything can be built using cargo.

```bash
cargo build
```

### Updating the generated bindings

When you modify the java code you might want to update the generated rust bindings.

You can do that using the Make target `bindgen`

```bash
make bindgen
```

## Overview

This project consists of multiple interconnected Rust crates that work together to load and manage Velocity plugins:

| Crate | Purpose |
|-------|---------|
| `ivpl` | Main crate containing the layer between `Infrarust` and the custom `ProxyServer` |
| `ivpl-java` | Provides the custom `ProxyServer` implementation, the embedded JAR loading and class loader initialization |
| `ivpl-java-binding` | JNI bindings for java.* classes |
| `ivpl-java-classloader` | Custom `ByteArrayClassLoader` for loading classes from byte arrays |
| `ivpl-loader` | The actual plugin loader that implements Infrarust's `PluginLoader` trait |

## Acknowledgments

This project would not be possible without:

- **[Infrarust](https://github.com/Shadowner/Infrarust)** - The High-Performance Minecraft Reverse Proxy in Rust
- **[Velocity](https://papermc.io/software/velocity)** - The modern, next-generation Minecraft server proxy.
- **[jni-rs](https://github.com/jni-rs/jni-rs)** - For providing JNI bindings in Rust

## License

This project is licensed under the **GNU Affero General Public License version 3 (AGPL-3.0)**.

See the [LICENSE](LICENSE) file for the full license text.
