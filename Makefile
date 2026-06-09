build-java:
	cd java/infrarust-velocity && mvn package
bindgen:
	mkdir -p src/java/generated
	jbindgen classfile java/infrarust-velocity/target/infrarust-velocity-1.0-SNAPSHOT.jar --output-dir src/java/generated --type-map src/java/implementation/type_map --output-type-map src/java/generated/type_map --pattern="dev.*" --pattern="com.velocitypowered.api.proxy.server.ServerInfo" --pattern="com.velocitypowered.api.proxy.server.ServerInfo" --pattern="com.velocitypowered.proxy.plugin.loader.VelocityPluginContainer" --pattern="com.velocitypowered.api.plugin.PluginContainer" --pattern="com.velocitypowered.api.event" --pattern="net.kyori.adventure.text.Component" --root crate::java::generated
