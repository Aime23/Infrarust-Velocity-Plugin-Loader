package dev.infrarust.plugin;

import com.google.inject.AbstractModule;
import com.google.inject.name.Names;
import com.velocitypowered.api.event.EventManager;
import com.velocitypowered.api.plugin.PluginContainer;
import com.velocitypowered.api.plugin.PluginManager;
import com.velocitypowered.api.proxy.ProxyServer;

public class CommonModule extends AbstractModule {

    private final ProxyServer server;
    private final PluginContainer container;

    public CommonModule(ProxyServer server, PluginContainer container) {
        this.server = server;
        this.container = container;
    }

    @Override
    protected void configure() {
        bind(ProxyServer.class).toInstance(server);
        bind(PluginManager.class).toInstance(server.getPluginManager());
        bind(EventManager.class).toInstance(server.getEventManager());
        // TODO: Uncomment when implemented
        // bind(CommandManager.class).toInstance(server.getCommandManager());
        bind(PluginContainer.class).annotatedWith(Names.named(container.getDescription().getId()))
                .toInstance(container);
    }
}
