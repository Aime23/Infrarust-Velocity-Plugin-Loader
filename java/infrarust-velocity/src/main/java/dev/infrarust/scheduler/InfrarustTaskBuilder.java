package dev.infrarust.scheduler;

import com.velocitypowered.api.scheduler.ScheduledTask;
import com.velocitypowered.api.scheduler.Scheduler;
import dev.infrarust.NativeFinalize;
import java.util.concurrent.TimeUnit;
import java.util.function.Consumer;
import org.checkerframework.common.value.qual.IntRange;
import org.jetbrains.annotations.NotNull;

public class InfrarustTaskBuilder
    extends NativeFinalize
    implements Scheduler.TaskBuilder
{

    protected final long scheduler_handle;

    private final Object plugin;
    private final Runnable runnable;
    private final Consumer<ScheduledTask> consumer;
    protected long delay_value;
    protected long repeat_value;

    public InfrarustTaskBuilder(
        long schedulerHandle,
        Object plugin,
        Runnable runnable
    ) {
        this.scheduler_handle = schedulerHandle;
        this.plugin = plugin;
        this.runnable = runnable;
        this.consumer = null;
    }

    public InfrarustTaskBuilder(
        long schedulerHandle,
        Object plugin,
        Consumer<ScheduledTask> consumer
    ) {
        this.scheduler_handle = schedulerHandle;
        this.plugin = plugin;
        this.runnable = null;
        this.consumer = consumer;
    }

    public native void native_finalize();

    private native long clone_handle();

    private native long native_schedule();

    @Override
    public Scheduler.TaskBuilder delay(
        @IntRange(from = 0L) long time,
        @NotNull TimeUnit unit
    ) {
        this.delay_value = unit.toMillis(time);
        return this;
    }

    @Override
    public Scheduler.TaskBuilder repeat(
        @IntRange(from = 0L) long time,
        @NotNull TimeUnit unit
    ) {
        this.repeat_value = unit.toMillis(time);
        return this;
    }

    @Override
    public Scheduler.TaskBuilder clearDelay() {
        this.delay_value = 0;
        return this;
    }

    @Override
    public Scheduler.TaskBuilder clearRepeat() {
        this.repeat_value = 0;
        return this;
    }

    @Override
    public ScheduledTask schedule() {
        // TODO: Do more things in Java
        return new InfrarustScheduledTask(
            this.native_schedule(),
            this.clone_handle(),
            this.plugin
        );
    }
}
