package dev.infrarust.proxy.server;

import com.velocitypowered.api.proxy.Player;
import com.velocitypowered.api.proxy.messages.ChannelIdentifier;
import com.velocitypowered.api.proxy.messages.PluginMessageEncoder;
import com.velocitypowered.api.proxy.server.PingOptions;
import com.velocitypowered.api.proxy.server.RegisteredServer;
import com.velocitypowered.api.proxy.server.ServerInfo;
import com.velocitypowered.api.proxy.server.ServerPing;
import dev.infrarust.NativeFinalize;
import dev.infrarust.proxy.InfrarustPlayer;
import org.jetbrains.annotations.NotNull;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.concurrent.CompletableFuture;

public class InfrarustRegisteredServer extends NativeFinalize implements RegisteredServer {

    protected final long player_registry_handle;
    protected final long config_service_handle;
    protected final String server_id;

    public InfrarustRegisteredServer(long playerRegistryHandle, long configServiceHandle, String serverId) {
        player_registry_handle = playerRegistryHandle;
        config_service_handle = configServiceHandle;
        server_id = serverId;
    }

    public native void native_finalize();

    @Override
    public ServerInfo getServerInfo() {
        return null;
    }

    @Override
    public Collection<Player> getPlayersConnected() {
        return List.of();
    }

    @Override
    public CompletableFuture<ServerPing> ping() {
        return null;
    }

    @Override
    public CompletableFuture<ServerPing> ping(PingOptions pingOptions) {
        return null;
    }

    @Override
    public boolean sendPluginMessage(@NotNull ChannelIdentifier identifier, byte @NotNull [] data) {
        return false;
    }

    @Override
    public boolean sendPluginMessage(@NotNull ChannelIdentifier identifier, @NotNull PluginMessageEncoder dataEncoder) {
        return false;
    }
}
