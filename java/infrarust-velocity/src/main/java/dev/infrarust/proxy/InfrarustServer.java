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

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    public InfrarustServer(@RustPrimitive("crate::java::handle::PluginContextHandle") long plugin_context_handle) {
        this.plugin_context_handle = plugin_context_handle;
    }

    public native void native_finalize();

    public native Optional<Player> native_get_player_by_uuid(long uuid_1, long uuid_2);
    public native Optional<Player> native_get_player_by_name(String name);
    public native InfrarustPlayer[] native_get_all_players();
    public native InfrarustPlayer[] native_match_player(String name);
    public native int native_get_player_count();
    public native Optional<RegisteredServer> native_get_server(String server_name);
    public native InfrarustRegisteredServer[] native_get_all_servers();
    public native InfrarustRegisteredServer[] native_match_server(String name);

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
        return this.native_get_player_by_name(name);
    }

    @Override
    public Optional<Player> getPlayer(UUID uuid) {
        return this.native_get_player_by_uuid(uuid.getMostSignificantBits(), uuid.getLeastSignificantBits());
    }

    @Override
    public Collection<Player> getAllPlayers() {
        return List.of(this.native_get_all_players());
    }

    @Override
    public Collection<Player> matchPlayer(String s) {
        return List.of(this.native_match_player(s));
    }

    @Override
    public int getPlayerCount() {
        return this.native_get_player_count();
    }

    @Override
    public Optional<RegisteredServer> getServer(String s) {
        return this.native_get_server(s);
    }

    @Override
    public Collection<RegisteredServer> getAllServers() {
        return List.of(this.native_get_all_servers());
    }

    @Override
    public Collection<RegisteredServer> matchServer(String s) {
        return List.of(this.native_match_server(s));
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
