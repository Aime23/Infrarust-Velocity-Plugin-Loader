package dev.infrarust.command;

import java.util.Collection;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import org.checkerframework.checker.nullness.qual.Nullable;
import com.mojang.brigadier.suggestion.Suggestions;
import com.velocitypowered.api.command.BrigadierCommand;
import com.velocitypowered.api.command.Command;
import com.velocitypowered.api.command.CommandManager;
import com.velocitypowered.api.command.CommandMeta;
import com.velocitypowered.api.command.CommandSource;
import dev.infrarust.NativeFinalize;
import io.github.jni_rs.jbindgen.RustPrimitive;

public class InfrarustCommandManager extends NativeFinalize implements CommandManager {

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    private final Map<String, CommandMeta> metaByAlias;
    private final Map<CommandMeta, Command> commandByMeta;

    public InfrarustCommandManager(
            @RustPrimitive("crate::java::handle::PluginContextHandle") long plugin_context_handle) {
        this.plugin_context_handle = plugin_context_handle;
        this.metaByAlias = new HashMap<>();
        this.commandByMeta = new HashMap<>();
    }

    private native void native_finalize();

    @Override
    public CommandMeta.Builder metaBuilder(String alias) {
        return new InfrarustCommandMetaBuilder().aliases(alias);
    }

    @Override
    public CommandMeta.Builder metaBuilder(BrigadierCommand command) {
        return new InfrarustCommandMetaBuilder().aliases(command.getNode().getName());
    }

    @Override
    public void register(BrigadierCommand command) {
        this.register(metaBuilder(command).build(), command);
    }

    @Override
    public void register(CommandMeta meta, Command command) {

        this.native_register(meta, command);
        for (String alias : meta.getAliases()) {
            this.metaByAlias.put(alias, meta);
        }
    }

    private native void native_register(CommandMeta meta, Command command);

    @Override
    public void unregister(String alias) {
        final CommandMeta meta = this.getCommandMeta(alias);
        if (meta != null) {
            this.unregister(meta);
        }
    }

    @Override
    public void unregister(CommandMeta meta) {
        this.native_unregister(meta);
    }

    private native void native_unregister(CommandMeta meta);


    @Override
    public @Nullable CommandMeta getCommandMeta(String alias) {
        return this.metaByAlias.get(alias);
    }

    @Override
    public CompletableFuture<Boolean> executeAsync(CommandSource source, String cmdLine) {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'executeAsync'");
    }

    @Override
    public CompletableFuture<Boolean> executeImmediatelyAsync(CommandSource source,
            String cmdLine) {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'executeImmediatelyAsync'");
    }

    @Override
    public CompletableFuture<List<String>> offerSuggestions(CommandSource source, String cmdLine) {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'offerSuggestions'");
    }

    @Override
    public CompletableFuture<Suggestions> offerBrigadierSuggestions(CommandSource source,
            String cmdLine) {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException("Unimplemented method 'offerBrigadierSuggestions'");
    }

    @Override
    public Collection<String> getAliases() {
        return this.metaByAlias.keySet();
    }

    @Override
    public boolean hasCommand(String alias) {
        return this.metaByAlias.containsKey(alias);
    }

    @Override
    public boolean hasCommand(String alias, CommandSource source) {
        return this.metaByAlias.containsKey(alias);
    }
}
