package dev.infrarust.command;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.LinkedHashSet;
import java.util.List;
import org.checkerframework.checker.nullness.qual.Nullable;
import com.mojang.brigadier.tree.CommandNode;
import com.velocitypowered.api.command.CommandMeta;
import com.velocitypowered.api.command.CommandSource;

public class InfrarustCommandMeta implements CommandMeta {

    private final Collection<String> aliases;
    private final List<CommandNode<CommandSource>> hints;
    private final @Nullable Object plugin;

    public InfrarustCommandMeta() {
        this.aliases = Collections.emptySet();
        this.hints = Collections.emptyList();
        this.plugin = null;
    }

    public InfrarustCommandMeta(Collection<String> aliases, List<CommandNode<CommandSource>> hints,
            @Nullable Object plugin) {
        this.aliases = Collections.unmodifiableCollection(new LinkedHashSet<>(aliases));
        this.hints = Collections.unmodifiableList(new ArrayList<>(hints));
        this.plugin = plugin;
    }

    @Override
    public Collection<String> getAliases() {
        return this.aliases;
    }

    @Override
    public Collection<CommandNode<CommandSource>> getHints() {
        return this.hints;
    }

    @Override
    public @Nullable Object getPlugin() {
        return this.plugin;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }

        InfrarustCommandMeta that = (InfrarustCommandMeta) o;

        return this.aliases.equals(that.aliases) && this.hints.equals(that.hints);
    }

    @Override
    public int hashCode() {
        int result = this.aliases.hashCode();
        result = 31 * result + this.hints.hashCode();
        return result;
    }

    @Override
    public String toString() {
        return "InfrarustCommandMeta{aliases=" + this.aliases + ", hints=" + this.hints.size()
                + ", plugin=" + this.plugin + '}';
    }
}
