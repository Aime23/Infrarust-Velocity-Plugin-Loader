package dev.infrarust.event;

import com.velocitypowered.api.event.AwaitingEventExecutor;
import com.velocitypowered.api.event.EventHandler;
import com.velocitypowered.api.event.EventTask;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Method;

public class EventHandlerBuilder {

    public static EventHandler<Object> buildEventTaskHandler(
        final Object listener,
        Method method
    ) {
        try {
            final MethodHandle methodHandle = MethodHandles.lookup().unreflect(
                method
            );
            return (AwaitingEventExecutor<Object>) event -> {
                try {
                    return (EventTask) methodHandle.invoke(
                        listener,
                        event
                    );
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            };
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    public static EventHandler<Object> buildVoidHandler(
        final Object listener,
        Method method
    ) {
        try {
            final MethodHandle methodHandle = MethodHandles.lookup().unreflect(
                method
            );
            return (AwaitingEventExecutor<Object>) event -> {
                try {
                    methodHandle.invoke(listener, event);
                    return null;
                } catch (Throwable e) {
                    throw new IllegalStateException(e);
                }
            };
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }

    public static EventHandler<Object> buildContinuationHandler(
        final Object listener,
        Method method
    ) {
        try {
            final MethodHandle methodHandle = MethodHandles.lookup().unreflect(
                method
            );
            return (AwaitingEventExecutor<Object>) event -> {
                return EventTask.withContinuation(continuation -> {
                    try {
                        methodHandle.invoke(listener, event, continuation);
                    } catch (Throwable e) {
                        throw new IllegalStateException(e);
                    }
                });
            };
        } catch (IllegalAccessException e) {
            throw new IllegalStateException(e);
        }
    }
}
