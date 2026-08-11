package dev.infrarust.scheduler;

import java.util.Collection;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import org.jetbrains.annotations.NotNull;
import com.velocitypowered.api.scheduler.ScheduledTask;
import com.velocitypowered.api.scheduler.Scheduler;
import dev.infrarust.NativeFinalize;
import dev.infrarust.proxy.InfrarustServer;
import io.github.jni_rs.jbindgen.RustPrimitive;

public class InfrarustScheduler extends NativeFinalize implements Scheduler {

    protected InfrarustServer server;
    private Map<UUID, InfrarustScheduledTask> taskMap = new HashMap<>();

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    public InfrarustScheduler(
            @RustPrimitive("crate::java::handle::PluginContextHandle") long plugin_context_handle,
            InfrarustServer server) {
        this.plugin_context_handle = plugin_context_handle;
        this.server = server;
    }

    public native void native_finalize();

    private native long clone_handle();

    @Override
    public TaskBuilder buildTask(@NotNull Object plugin, @NotNull Runnable runnable) {
        return new InfrarustTaskBuilder(this.clone_handle(), this, plugin, runnable);
    }

    @Override
    public TaskBuilder buildTask(@NotNull Object plugin,
            @NotNull Consumer<ScheduledTask> consumer) {
        return new InfrarustTaskBuilder(this.clone_handle(), this, plugin, consumer);
    }

    @Override
    public @NotNull Collection<ScheduledTask> tasksByPlugin(@NotNull Object plugin) {
        return this.taskMap.values().stream().filter(arg0 -> arg0.plugin() == plugin)
                .collect(Collectors.toList());
    }
    }
}
