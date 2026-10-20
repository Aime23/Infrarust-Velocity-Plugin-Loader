package dev.infrarust.command;

import java.util.ArrayList;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Locale;
import java.util.Objects;
import org.checkerframework.checker.nullness.qual.Nullable;
import com.mojang.brigadier.tree.CommandNode;
import com.velocitypowered.api.command.CommandMeta;
import com.velocitypowered.api.command.CommandSource;

public class InfrarustCommandMetaBuilder implements CommandMeta.Builder {

    private final LinkedHashSet<String> aliases = new LinkedHashSet<>();
    private final List<CommandNode<CommandSource>> hints = new ArrayList<>();
    private @Nullable Object plugin;

    public InfrarustCommandMetaBuilder() {
    }

    @Override
    public CommandMeta.Builder aliases(String... aliases) {
        Objects.requireNonNull(aliases, "aliases");
        for (int i = 0, length = aliases.length; i < length; i++) {
            String alias = aliases[i];
            Objects.requireNonNull(alias, "alias at index " + i);
            this.aliases.add(alias.toLowerCase(Locale.ENGLISH));
        }
        return this;
    }

    @Override
    public CommandMeta.Builder hint(CommandNode<CommandSource> node) {
        Objects.requireNonNull(node, "node");
        if (node.getCommand() != null) {
            throw new IllegalArgumentException("Cannot use executable node for hinting");
        }
        if (node.getRedirect() != null) {
            throw new IllegalArgumentException("Cannot use a node with a redirect for hinting");
        }
        this.hints.add(node);
        return this;
    }

    @Override
    public CommandMeta.Builder plugin(Object plugin) {
        Objects.requireNonNull(plugin, "plugin");
        this.plugin = plugin;
        return this;
    }

    @Override
    public CommandMeta build() {
        return new InfrarustCommandMeta(this.aliases, this.hints, this.plugin);
    }
}
