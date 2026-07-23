package dev.infrarust.scheduler;

import org.jetbrains.annotations.NotNull;
import com.velocitypowered.api.scheduler.ScheduledTask;
import com.velocitypowered.api.scheduler.TaskStatus;
import dev.infrarust.NativeFinalize;

public class InfrarustScheduledTask extends NativeFinalize implements ScheduledTask {

    protected final long task_handle;
    protected final long scheduler_handle;

    private final Object plugin;

    private TaskStatus status;

    public InfrarustScheduledTask(long taskHandle, long schedulerHandle, Object plugin) {
        task_handle = taskHandle;
        scheduler_handle = schedulerHandle;
        this.plugin = plugin;
        this.status = TaskStatus.SCHEDULED;
    }

    public native void native_finalize();

    private native void native_cancel();

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
        this.native_cancel();
        this.status = TaskStatus.CANCELLED;
    }
}
