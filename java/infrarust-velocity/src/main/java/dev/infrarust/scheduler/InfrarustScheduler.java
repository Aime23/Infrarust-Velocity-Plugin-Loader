package dev.infrarust.scheduler;

import com.velocitypowered.api.scheduler.ScheduledTask;
import com.velocitypowered.api.scheduler.Scheduler;
import dev.infrarust.NativeFinalize;
import io.github.jni_rs.jbindgen.RustPrimitive;
import java.util.Collection;
import java.util.List;
import java.util.function.Consumer;
import org.jetbrains.annotations.NotNull;

public class InfrarustScheduler extends NativeFinalize implements Scheduler {

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    public InfrarustScheduler(
        @RustPrimitive(
            "crate::java::handle::PluginContextHandle"
        ) long plugin_context_handle
    ) {
        this.plugin_context_handle = plugin_context_handle;
    }

    public native void native_finalize();

    private native long clone_handle();

    @Override
    public TaskBuilder buildTask(
        @NotNull Object plugin,
        @NotNull Runnable runnable
    ) {
        return new InfrarustTaskBuilder(this.clone_handle(), plugin, runnable);
    }

    @Override
    public TaskBuilder buildTask(
        @NotNull Object plugin,
        @NotNull Consumer<ScheduledTask> consumer
    ) {
        return new InfrarustTaskBuilder(this.clone_handle(), plugin, consumer);
    }

    @Override
    public @NotNull Collection<ScheduledTask> tasksByPlugin(
        @NotNull Object plugin
    ) {
        return List.of();
    }
}
