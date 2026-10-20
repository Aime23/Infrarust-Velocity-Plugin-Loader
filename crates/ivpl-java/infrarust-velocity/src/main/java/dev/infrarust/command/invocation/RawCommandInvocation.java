package dev.infrarust.command.invocation;

import com.velocitypowered.api.command.CommandSource;
import com.velocitypowered.api.command.RawCommand;

public class RawCommandInvocation extends AbstractCommandInvocation<String> implements RawCommand.Invocation {

    private String alias;

    protected RawCommandInvocation(CommandSource source, String alias, String arguments) {
        super(source, arguments);
        this.alias = alias;
    }

	@Override
	public String alias() {
		return alias;
	}

}
