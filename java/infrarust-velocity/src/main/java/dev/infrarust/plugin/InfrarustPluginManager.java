package dev.infrarust.plugin;

import java.nio.file.Path;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import com.google.common.base.Joiner;
import com.google.inject.AbstractModule;
import com.google.inject.Module;
import com.velocitypowered.api.plugin.PluginContainer;
import com.velocitypowered.api.plugin.PluginDescription;
import com.velocitypowered.api.plugin.meta.PluginDependency;
import com.velocitypowered.api.proxy.ProxyServer;
import com.velocitypowered.proxy.plugin.VelocityPluginManager;
import com.velocitypowered.proxy.plugin.loader.VelocityPluginContainer;
import com.velocitypowered.proxy.plugin.loader.java.JavaPluginLoader;
import dev.infrarust.proxy.InfrarustServer;

public class InfrarustPluginManager extends VelocityPluginManager {

    private static final Logger logger = LogManager.getLogger(InfrarustPluginManager.class);
    private final ProxyServer server;

    public InfrarustPluginManager(InfrarustServer server) {
        super(server);
        this.server = server;
    }

    public PluginContainer loadPlugin(Path directory, Path source) {
        JavaPluginLoader loader = new JavaPluginLoader(server, directory);
        PluginDescription candidate;
        try {
            candidate = loader.loadCandidate(source);
        } catch (Exception e) {
            logger.error("Unable to load plugin {}", source);
            return null;
        }

        for (PluginDependency dependency : candidate.getDependencies()) {
            if (!dependency.isOptional() && this.isLoaded(dependency.getId())) {
                logger.error("Missing required dependency {} to load {} at {}", dependency.getId(),
                        candidate.getId(), source);
                return null;
            }
        }
        PluginDescription description;
        VelocityPluginContainer container;
        Module module;
        try {
            description = loader.createPluginFromCandidate(candidate);
            container = new VelocityPluginContainer(description);
            module = loader.createModule(container);
        } catch (Throwable e) {
            logger.error("Can't create module for plugin {}", candidate.getId(), e);
            return null;
        }

        // Make a global Guice module that with common bindings for every plugin
        AbstractModule commonModule = new CommonModule(server, container);
        try {
            loader.createPlugin(container, module, commonModule);
        } catch (Throwable e) {
            logger.error("Can't create plugin {}", description.getId(), e);
            return null;
        }

        logger.info("Loaded plugin {} {} by {}", description.getId(),
                description.getVersion().orElse("<UNKNOWN>"),
                Joiner.on(", ").join(description.getAuthors()));
        this.registerPlugin(container);

        var plugin_instance = container.getInstance().get();
        this.server.getEventManager().register(plugin_instance, plugin_instance);
        return container;
    }
}
