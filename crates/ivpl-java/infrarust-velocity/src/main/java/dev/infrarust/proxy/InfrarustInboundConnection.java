package dev.infrarust.proxy;

import java.net.InetSocketAddress;
import java.util.Optional;
import org.checkerframework.checker.nullness.qual.Nullable;
import com.velocitypowered.api.network.HandshakeIntent;
import com.velocitypowered.api.network.ProtocolState;
import com.velocitypowered.api.network.ProtocolVersion;
import com.velocitypowered.api.proxy.InboundConnection;

public class InfrarustInboundConnection implements InboundConnection {

    private final InetSocketAddress remoteAddress;
    private final InetSocketAddress virtualHost;
    private final String rawVirtualHost;
    private final ProtocolVersion protocolVersion;
    private final ProtocolState protocolState;
    private final HandshakeIntent handshakeIntent;


    public InfrarustInboundConnection(final InetSocketAddress remoteAddress,
            final ProtocolVersion protocolVersion, final ProtocolState protocolState,
            final HandshakeIntent handshakeIntent) {
        this(remoteAddress, null, null, protocolVersion, protocolState, handshakeIntent);
    }

    public InfrarustInboundConnection(final InetSocketAddress remoteAddress,
            @Nullable final InetSocketAddress virtualHost, @Nullable final String rawVirtualHost,
            final ProtocolVersion protocolVersion, final ProtocolState protocolState,
            final HandshakeIntent handshakeIntent) {
        this.remoteAddress = remoteAddress;
        this.virtualHost = virtualHost;
        this.rawVirtualHost = rawVirtualHost;
        this.protocolVersion = protocolVersion;
        this.protocolState = protocolState;
        this.handshakeIntent = handshakeIntent;
    }

    @Override
    public InetSocketAddress getRemoteAddress() {
        return this.remoteAddress;
    }

    @Override
    public Optional<InetSocketAddress> getVirtualHost() {
        return Optional.ofNullable(this.virtualHost);
    }

    @Override
    public Optional<String> getRawVirtualHost() {
        return Optional.ofNullable(this.rawVirtualHost);
    }

    @Override
    public boolean isActive() {
        return true;
    }

    @Override
    public ProtocolVersion getProtocolVersion() {
        return this.protocolVersion;
    }

    @Override
    public ProtocolState getProtocolState() {
        return this.protocolState;
    }

    @Override
    public HandshakeIntent getHandshakeIntent() {
        return this.handshakeIntent;
    }


}
