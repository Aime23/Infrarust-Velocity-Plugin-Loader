package dev.infrarust.proxy;

import com.velocitypowered.api.command.CommandManager;
import com.velocitypowered.api.event.EventManager;
import com.velocitypowered.api.plugin.PluginManager;
import com.velocitypowered.api.proxy.ConsoleCommandSource;
import com.velocitypowered.api.proxy.Player;
import com.velocitypowered.api.proxy.ProxyServer;
import com.velocitypowered.api.proxy.config.ProxyConfig;
import com.velocitypowered.api.proxy.messages.ChannelRegistrar;
import com.velocitypowered.api.proxy.player.ResourcePackInfo;
import com.velocitypowered.api.proxy.server.RegisteredServer;
import com.velocitypowered.api.proxy.server.ServerInfo;
import com.velocitypowered.api.scheduler.Scheduler;
import com.velocitypowered.api.util.ProxyVersion;
import com.velocitypowered.proxy.plugin.VelocityPluginManager;
import dev.infrarust.NativeFinalize;
import dev.infrarust.event.InfrarustEventManager;
import dev.infrarust.plugin.InfrarustPluginManager;
import dev.infrarust.proxy.server.InfrarustRegisteredServer;
import dev.infrarust.scheduler.InfrarustScheduler;
import io.github.jni_rs.jbindgen.RustPrimitive;
import net.kyori.adventure.text.Component;

import java.net.InetSocketAddress;
import java.util.*;

public class InfrarustServer extends NativeFinalize implements ProxyServer {


    protected final long plugin_context_handle;

    public InfrarustServer(long plugin_context_handle) {
        this.plugin_context_handle = plugin_context_handle;
    }

    public native void native_finalize();

    @Override
    public void shutdown(Component component) {
    }

    @Override
    public void shutdown() {
    }

    @Override
    public boolean isShuttingDown() {
        return false;
    }

    @Override
    public void closeListeners() {
    }

    @Override
    public Optional<Player> getPlayer(String name) {
        return null;
    }

    @Override
    public Optional<Player> getPlayer(UUID uuid) {
        return null;
    }

    @Override
    public Collection<Player> getAllPlayers() {
        return List.of();
    }

    @Override
    public Collection<Player> matchPlayer(String s) {
        return List.of();
    }

    @Override
    public int getPlayerCount() {
        return 0;
    }

    @Override
    public Optional<RegisteredServer> getServer(String s) {
        return Optional.empty();
    }

    @Override
    public Collection<RegisteredServer> getAllServers() {
        return List.of();
    }

    @Override
    public Collection<RegisteredServer> matchServer(String s) {
        return List.of();
    }

    @Override
    public RegisteredServer createRawRegisteredServer(ServerInfo serverInfo) {
        return null;
    }

    @Override
    public RegisteredServer registerServer(ServerInfo serverInfo) {
        return null;
    }

    @Override
    public void unregisterServer(ServerInfo serverInfo) {

    }

    @Override
    public ConsoleCommandSource getConsoleCommandSource() {
        return null;
    }

    @Override
    public PluginManager getPluginManager() {
        return null;
    }

    @Override
    public EventManager getEventManager() {
        return null;
    }

    @Override
    public CommandManager getCommandManager() {
        return null;
    }

    @Override
    public Scheduler getScheduler() {
        return null;
    }

    @Override
    public ChannelRegistrar getChannelRegistrar() {
        return null;
    }

    @Override
    public InetSocketAddress getBoundAddress() {
        return null;
    }

    @Override
    public ProxyConfig getConfiguration() {
        return null;
    }

    @Override
    public ProxyVersion getVersion() {
        return new ProxyVersion("Infrarust", "Infrarust", "<unknown>");
    }

    @Override
    public ResourcePackInfo.Builder createResourcePackBuilder(String s) {
        return null;
    }
}
