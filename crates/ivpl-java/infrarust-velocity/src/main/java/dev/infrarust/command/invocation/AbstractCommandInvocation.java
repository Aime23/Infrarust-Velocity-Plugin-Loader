package dev.infrarust.command.invocation;

import com.google.common.base.Preconditions;
import com.velocitypowered.api.command.CommandInvocation;
import com.velocitypowered.api.command.CommandSource;

/**
 * AbstractCommandInvocation
 */
public class AbstractCommandInvocation<T> implements CommandInvocation<T> {

    private CommandSource source;
    private T arguments;

    protected AbstractCommandInvocation(final CommandSource source, final T arguments) {
        this.source = Preconditions.checkNotNull(source, "source");
        this.arguments = Preconditions.checkNotNull(arguments, "arguments");
    }

    @Override
    public CommandSource source() {
        return source;
    }

    @Override
    public T arguments() {
        return arguments;
    }

}
