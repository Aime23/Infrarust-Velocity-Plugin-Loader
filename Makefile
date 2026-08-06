bindgen-java-build:
	cd crates/ivpl-java/infrarust-velocity && mvn package
bindgen-generate:
	mkdir -p crates/ivpl/src/java/generated
	jbindgen classfile crates/ivpl-java/infrarust-velocity/target/shaded.jar --output-dir crates/ivpl/src/java/generated --type-map crates/ivpl/src/java/implementation/type_map --output-type-map crates/ivpl/src/java/generated/type_map --pattern="dev.*" --pattern="com.velocitypowered.api.proxy.server.ServerInfo" --pattern="com.velocitypowered.api.proxy.server.ServerInfo" --pattern="com.velocitypowered.proxy.plugin.loader.VelocityPluginContainer" --pattern="com.velocitypowered.api.plugin.PluginContainer" --pattern="com.velocitypowered.api.event" --pattern="com.velocitypowered.api.proxy.InboundConnection" --pattern="com.velocitypowered.api.proxy.Player" --pattern="com.velocitypowered.api.proxy.server.RegisteredServer" --pattern="com.velocitypowered.api.network.*" --pattern="net.kyori.adventure.text.Component" --pattern="net.kyori.adventure.text.TextComponent" --root crate::java::generated
bindgen: bindgen-java-build bindgen-generate
