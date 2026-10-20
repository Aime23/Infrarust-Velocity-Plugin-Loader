package dev.infrarust.command.invocation;

import com.velocitypowered.api.command.CommandSource;
import com.velocitypowered.api.command.SimpleCommand;

public class SimpleCommandInvocation extends AbstractCommandInvocation<String[]>
        implements SimpleCommand.Invocation {

    private String alias;

    protected SimpleCommandInvocation(CommandSource source, String alias, String[] arguments) {
        super(source, arguments);
        this.alias = alias;
    }

    @Override
    public String alias() {
        return alias;
    }

}
