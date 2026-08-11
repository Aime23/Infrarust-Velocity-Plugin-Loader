package dev.infrarust.scheduler;

import java.util.UUID;
import java.util.function.Consumer;
import org.jetbrains.annotations.NotNull;
import com.velocitypowered.api.scheduler.ScheduledTask;
import com.velocitypowered.api.scheduler.TaskStatus;
import dev.infrarust.NativeFinalize;
import io.github.jni_rs.jbindgen.RustPrimitive;

public class InfrarustScheduledTask extends NativeFinalize implements ScheduledTask, Runnable {

    @RustPrimitive("crate::java::handle::TaskHandle")
    private final long taskHandle;
    private final UUID uuid;
    private final InfrarustScheduler scheduler;
    private final Object plugin;
    private final Consumer<ScheduledTask> consumer;
    private final long delay;
    private final long repeat;
    private TaskStatus status;

    public InfrarustScheduledTask(@RustPrimitive("crate::java::handle::TaskHandle") long taskHandle,
            UUID uuid, InfrarustScheduler scheduler, Object plugin,
            Consumer<ScheduledTask> consumer, long delay, long repeat) {
        this.taskHandle = taskHandle;
        this.scheduler = scheduler;
        this.uuid = uuid;
        this.plugin = plugin;
        this.consumer = consumer;
        this.status = TaskStatus.SCHEDULED;
        this.delay = delay;
        this.repeat = repeat;
    }

    public native void native_finalize();

    @Override
    public @NotNull Object plugin() {
        return this.plugin;
    }

    @Override
    public TaskStatus status() {
        return this.status;
    }

    @Override
    public void cancel() {
        this.scheduler.unregisterTask(this);
        this.status = TaskStatus.CANCELLED;
    }

    public UUID uuid() {
        return this.uuid;
    }

    public long handle() {
        return this.taskHandle;
    }

    @Override
    public void run() {
        this.consumer.accept(this);
    }
}
