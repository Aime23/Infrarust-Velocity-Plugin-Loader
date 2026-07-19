package dev.infrarust.event;

import static java.util.Objects.requireNonNull;

import com.google.common.collect.ArrayListMultimap;
import com.google.common.collect.ListMultimap;
import com.velocitypowered.api.event.Continuation;
import com.velocitypowered.api.event.EventHandler;
import com.velocitypowered.api.event.EventManager;
import com.velocitypowered.api.event.EventTask;
import com.velocitypowered.api.event.PostOrder;
import com.velocitypowered.api.event.Subscribe;
import com.velocitypowered.api.plugin.PluginContainer;
import com.velocitypowered.api.plugin.PluginDescription;
import com.velocitypowered.api.plugin.PluginManager;
import dev.infrarust.NativeFinalize;
import io.github.jni_rs.jbindgen.RustPrimitive;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.locks.ReentrantReadWriteLock;
import java.util.concurrent.locks.ReentrantReadWriteLock.ReadLock;
import java.util.concurrent.locks.ReentrantReadWriteLock.WriteLock;
import java.util.function.Predicate;
import java.util.stream.Collectors;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.checkerframework.checker.nullness.qual.Nullable;

public class InfrarustEventManager
    extends NativeFinalize
    implements EventManager
{

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    private final PluginManager pluginManager;

    private static final Logger logger = LogManager.getLogger(
        InfrarustEventManager.class
    );

    // Start of lock
    private final ReentrantReadWriteLock lock = new ReentrantReadWriteLock();
    private final WriteLock writeLock = lock.writeLock();
    private final ReadLock readLock = lock.readLock();

    private final List<RegisteredEventHandler> registeredEventHandlers =
        new ArrayList<>();

    private final Map<
        Class<?>,
        List<RegisteredEventHandler>
    > mappedEventHandlers = new HashMap<>();

    // End of lock

    public InfrarustEventManager(
        @RustPrimitive(
            "crate::java::handle::PluginContextHandle"
        ) final long plugin_context_handle,
        final PluginManager pluginManager
    ) {
        this.plugin_context_handle = plugin_context_handle;
        this.pluginManager = pluginManager;
        this.native_initialize();
    }

    private native void native_finalize();

    private native void native_initialize();

    /**
     * Extract all method with the Subscribe annotation in the given class.
     * The method should not be static or abstract
     *
     * @return
     */

    public Map<String, EventHandlingMethod> extractEventHandlingMethod(
        final Class<?> listener
    ) {
        final var extracted = new HashMap<String, EventHandlingMethod>();
        return extractEventHandlingMethod(listener, extracted);
    }

    public Map<String, EventHandlingMethod> extractEventHandlingMethod(
        final Class<?> listener,
        final Map<String, EventHandlingMethod> extracted
    ) {
        final List<String> errors = new ArrayList<String>();
        final var methods = listener.getMethods();

        for (final Method method : methods) {
            if (method.getAnnotation(Subscribe.class) == null) continue;

            final String key = String.format(
                "%s%s(%s)",
                Modifier.isPrivate(method.getModifiers()) ? "?" : "",
                method.getName(),
                Arrays.stream(method.getParameterTypes())
                    .map(Class::getName)
                    .collect(Collectors.joining(","))
            );
            // Prevent overwriting with parent methods
            if (extracted.containsKey(key)) continue;

            try {
                final EventHandlingMethod eventHandlingMethod =
                    EventHandlingMethod.ofMethod(method);

                extracted.put(key, eventHandlingMethod);
            } catch (final Exception e) {
                errors.add(String.format("Method %s: %s", key, e.getMessage()));
            }
        }

        if (listener.getSuperclass() == Object.class) return extracted;
        return extractEventHandlingMethod(listener.getSuperclass(), extracted);
    }

    public void registerInternally(
        PluginContainer pluginContainer,
        Object listener
    ) {
        Map<String, EventHandlingMethod> ehms = extractEventHandlingMethod(
            listener.getClass()
        );

        for (EventHandlingMethod eventHandlingMethod : ehms.values()) {
            this.registerEventHandlingMethod(
                eventHandlingMethod,
                listener,
                pluginContainer
            );
        }
    }

    private void registerEventHandlingMethod(
        EventHandlingMethod eventHandlingMethod,
        Object listener,
        PluginContainer pluginContainer
    ) {
        RegisteredEventHandler registeredEventHandler =
            RegisteredEventHandler.fromEventHandlingMethod(
                eventHandlingMethod,
                listener,
                pluginContainer
            );
        this.registerEventHandler(registeredEventHandler);
    }

    private void registerEventHandler(RegisteredEventHandler eventHandler) {
        try {
            writeLock.lock();
            this.registeredEventHandlers.add(eventHandler);

            if (
                !this.mappedEventHandlers.containsKey(eventHandler.eventClass)
            ) {
                this.mappedEventHandlers.put(
                    eventHandler.eventClass,
                    new ArrayList<>()
                );
            }
            this.mappedEventHandlers
                .get(eventHandler.eventClass)
                .add(eventHandler);
        } finally {
            writeLock.unlock();
        }
    }

    @Override
    public void register(final Object plugin, final Object listener) {
        requireNonNull(listener);
        PluginContainer container = pluginManager.ensurePluginContainer(plugin);
        if (plugin == listener) throw new IllegalArgumentException(
            "Main plugin class is automaticaly registered"
        );

        this.registerInternally(container, listener);
    }

    @Override
    public <E> void register(
        final Object plugin,
        final Class<E> eventClass,
        final PostOrder postOrder,
        final EventHandler<E> handler
    ) {
        register(plugin, eventClass, mapOrder(postOrder), handler);
    }

    @Override
    public <E> void register(
        final Object plugin,
        final Class<E> eventClass,
        final short priority,
        final EventHandler<E> handler
    ) {
        PluginContainer pluginContainer = pluginManager.ensurePluginContainer(
            plugin
        );

        RegisteredEventHandler rehm = new RegisteredEventHandler(
            pluginContainer,
            eventClass,
            (EventHandler<Object>) handler,
            AsyncLevel.Full,
            priority
        );
        this.registerEventHandler(rehm);
    }

    @Override
    public <E> CompletableFuture<E> fire(final E event) {
        Class<?> eventClass = event.getClass();
        List<RegisteredEventHandler> handlers =
            this.mappedEventHandlers.getOrDefault(
                eventClass,
                new ArrayList<>()
            );

        if (handlers.size() == 0) {
            return CompletableFuture.completedFuture(event);
        }
        final CompletableFuture<E> future = new CompletableFuture<>();

        RegisteredEventHandler handler = handlers.getFirst();
        if (handler.asyncType == AsyncLevel.Full) {
            handler.pluginContainer
                .getExecutorService()
                .execute(() ->
                    callEventHandlers(
                        event,
                        future,
                        0,
                        true,
                        handlers.toArray(new RegisteredEventHandler[0])
                    )
                );
        } else {
            callEventHandlers(
                event,
                future,
                0,
                false,
                handlers.toArray(new RegisteredEventHandler[0])
            );
        }
        return future;
    }

    private <E> void callEventHandlers(
        final E event,
        final @Nullable CompletableFuture<E> future,
        final int offset,
        final boolean currentlyAsync,
        final RegisteredEventHandler[] handlers
    ) {
        for (int i = offset; i < handlers.length; i++) {
            RegisteredEventHandler handler = handlers[i];
            final EventTask eventTask = handler.eventHandler.executeAsync(
                event
            );

            if (eventTask == null) continue;
            // Handling continuation
            ContinuationTask<E> continuationTask = new ContinuationTask<E>(
                eventTask,
                handlers,
                future,
                event,
                i,
                currentlyAsync
            );
            if (currentlyAsync || !eventTask.requiresAsync()) {
                // Already in an async context
                if (continuationTask.execute()) {
                    continue;
                }
            } else {
                // Execute asynchronously
                handler.pluginContainer
                    .getExecutorService()
                    .execute(continuationTask);
            }
        }
        if (future != null) {
            future.complete(event);
        }
    }

    @Override
    public void unregisterListeners(final Object plugin) {
        final PluginContainer pluginContainer =
            this.pluginManager.ensurePluginContainer(plugin);
        unregisterIf(
            registered -> registered.pluginContainer == pluginContainer
        );
    }

    @Override
    public void unregisterListener(final Object plugin, final Object listener) {
        final PluginContainer pluginContainer =
            this.pluginManager.ensurePluginContainer(plugin);
        unregisterIf(
            registered ->
                registered.pluginContainer == pluginContainer &&
                registered.eventHandler == listener
        );
    }

    @Override
    public <E> void unregister(
        final Object plugin,
        final EventHandler<E> handler
    ) {
        unregisterListener(plugin, handler);
    }

    // Really close to what velocity is doing be I found their solution elegant
    private void unregisterIf(
        final Predicate<RegisteredEventHandler> predicate
    ) {
        try {
            writeLock.lock();
            ListMultimap<Class<?>, RegisteredEventHandler> removedHandlers =
                ArrayListMultimap.create();
            final Iterator<RegisteredEventHandler> it =
                registeredEventHandlers.iterator();
            while (it.hasNext()) {
                final RegisteredEventHandler handler = it.next();
                if (predicate.test(handler)) {
                    it.remove();
                    removedHandlers.put(handler.eventClass, handler);
                }
            }
            for (Entry<
                Class<?>,
                Collection<RegisteredEventHandler>
            > removedHandler : removedHandlers.asMap().entrySet()) {
                this.mappedEventHandlers.computeIfPresent(
                    removedHandler.getKey(),
                    (arg0, arg1) -> {
                        arg1.removeAll(removedHandler.getValue());
                        return arg1;
                    }
                );
            }
        } finally {
            writeLock.unlock();
        }
    }

    public enum AsyncLevel {
        None, // Fully sync
        Partial, // Only the EventTask is run async
        Full, // The whole handler is run async
    }

    public static class EventHandlingMethod {

        private final Class<?> eventClass;
        private final Class<?> continuationClass;
        private final Method method;
        private final AsyncLevel asyncType;
        private final short priority;

        private EventHandlingMethod(
            final Class<?> eventClass,
            final Class<?> continuationClass,
            final Method method,
            final AsyncLevel asyncType,
            final short priority
        ) {
            this.eventClass = eventClass;
            this.continuationClass = continuationClass;
            this.method = method;
            this.asyncType = asyncType;
            this.priority = priority;
        }

        public static EventHandlingMethod ofMethod(final Method method)
            throws Exception {
            final Subscribe subscribeAnnotation = method.getAnnotation(
                Subscribe.class
            );
            if (Modifier.isStatic(method.getModifiers())) throw new Exception(
                "Method must not be static"
            );
            if (Modifier.isAbstract(method.getModifiers())) throw new Exception(
                "Method must not be abstract"
            );
            if (method.getParameterCount() == 0) throw new Exception(
                "Method must have at least 1 parameter which is the event"
            );

            if (method.getParameterCount() > 2) throw new Exception(
                "Method has to many parameter, expect maximum of 2"
            );

            final Class<?>[] parametersType = method.getParameterTypes();
            final var eventClass = parametersType[0];
            final var returnClass = method.getReturnType();

            AsyncLevel asyncType = AsyncLevel.None;

            if (method.getParameterCount() == 1) {
                if (
                    returnClass != void.class && returnClass != EventTask.class
                ) throw new Exception(
                    "Method must return either void or EventTask throught EventTask.async or EventTask.withContinuation"
                ); // https://docs.papermc.io/velocity/dev/event-api/#handling-events-asynchronously
                if (returnClass == EventTask.class) asyncType =
                    AsyncLevel.Partial;
                if (
                    returnClass == void.class && subscribeAnnotation.async()
                ) asyncType = AsyncLevel.Full;
            }
            Class<?> continuationClass = null;
            if (method.getParameterCount() == 2) {
                continuationClass = parametersType[1];
                if (
                    continuationClass != Continuation.class
                ) throw new Exception(
                    "Method has 2 parameters, however second is not a Continuation"
                );
                if (returnClass != void.class) throw new Exception(
                    "With continuation the method must return void"
                );

                asyncType = AsyncLevel.None;
            }

            short priority = 0;
            if (subscribeAnnotation.priority() != 0) {
                priority = subscribeAnnotation.priority();
            } else {
                priority = mapOrder(subscribeAnnotation.order());
            }

            return new EventHandlingMethod(
                eventClass,
                continuationClass,
                method,
                asyncType,
                priority
            );
        }
    }

    private static short mapOrder(final PostOrder order) {
        switch (order) {
            case FIRST:
                return Short.MAX_VALUE;
            case EARLY:
                return Short.MAX_VALUE / 2;
            case NORMAL:
                return 0;
            case LATE:
                return Short.MIN_VALUE / 2;
            case LAST:
                return Short.MIN_VALUE;
            default:
                return 0;
        }
    }

    public static class RegisteredEventHandler {

        private PluginContainer pluginContainer;
        private final Class<?> eventClass;
        private final EventHandler<Object> eventHandler;
        private final AsyncLevel asyncType;
        private final short priority;

        public RegisteredEventHandler(
            final PluginContainer pluginContainer,
            final Class<?> eventClass,
            final EventHandler<Object> eventHandler,
            final AsyncLevel asyncType,
            final short priority
        ) {
            this.pluginContainer = pluginContainer;
            this.eventClass = eventClass;
            this.eventHandler = eventHandler;
            this.asyncType = asyncType;
            this.priority = priority;
        }

        public static RegisteredEventHandler fromEventHandlingMethod(
            EventHandlingMethod eventHandlingMethod,
            Object listener,
            PluginContainer pluginContainer
        ) {
            final EventHandler<?> eventHandler;
            if (eventHandlingMethod.continuationClass != null) {
                eventHandler = EventHandlerBuilder.buildContinuationHandler(
                    listener,
                    eventHandlingMethod.method
                );
            } else if (
                EventTask.class.isAssignableFrom(
                    eventHandlingMethod.method.getReturnType()
                )
            ) {
                eventHandler = EventHandlerBuilder.buildEventTaskHandler(
                    listener,
                    eventHandlingMethod.method
                );
            } else {
                eventHandler = EventHandlerBuilder.buildVoidHandler(
                    listener,
                    eventHandlingMethod.method
                );
            }

            return new RegisteredEventHandler(
                pluginContainer,
                eventHandlingMethod.eventClass,
                (EventHandler<Object>) eventHandler,
                eventHandlingMethod.asyncType,
                eventHandlingMethod.priority
            );
        }
    }

    // From velocity-proxy
    // VelocityEventManager.java#566
    // The ContinuationTask is doing some memory optimization by using VarHandles to access the
    // state and resumed fields.
    // They are doing atomic CAS (Compare And Swap) by using VarHandle#compareAndSet.
    // They could have achived the same by using AtomicInteger and AtomicBoolean.
    // However this way thay can save at least 32 bytes per instances, based on my calculation
    // ╔═══════════════╦══════════════╗
    // ║ Type ║ Size (bytes) ║
    // ╠═══════════════╬══════════════╣
    // ║ bool ║ 4 ║
    // ║ int ║ 4 ║
    // ║ AtomicBoolean ║ 16 ║
    // ║ AtomicInt ║ 16 ║
    // ╚═══════════════╩══════════════╝
    // So that way we get with the Atomic version : (ref) 8 + (AtomicInteger) 16 + (AtomicBoolean)
    // 16 = 40 bytes
    // And with the VarHandle version : (int) 4 + (bool) 4 = 8 bytes
    private static final int TASK_STATE_DEFAULT = 0;
    private static final int TASK_STATE_EXECUTING = 1;
    private static final int TASK_STATE_CONTINUE_IMMEDIATELY = 2;

    private static final VarHandle CONTINUATION_TASK_RESUMED;
    private static final VarHandle CONTINUATION_TASK_STATE;

    static {
        try {
            CONTINUATION_TASK_RESUMED = MethodHandles.lookup().findVarHandle(
                ContinuationTask.class,
                "resumed",
                boolean.class
            );
            CONTINUATION_TASK_STATE = MethodHandles.lookup().findVarHandle(
                ContinuationTask.class,
                "state",
                int.class
            );
        } catch (final ReflectiveOperationException e) {
            throw new IllegalStateException();
        }
    }

    final class ContinuationTask<E> implements Continuation, Runnable {

        private final EventTask task;
        private final int index;
        private final RegisteredEventHandler[] registrations;
        private final @Nullable CompletableFuture<E> future;
        private final boolean currentlyAsync;
        private final E event;
        private final Thread firedOnThread;

        // This field is modified via a VarHandle, so this field is used and cannot be final.
        @SuppressWarnings({
            "UnusedVariable",
            "FieldMayBeFinal",
            "FieldCanBeLocal",
        })
        private volatile int state = TASK_STATE_DEFAULT;

        // This field is modified via a VarHandle, so this field is used and cannot be final.
        @SuppressWarnings({ "UnusedVariable", "FieldMayBeFinal" })
        private volatile boolean resumed = false;

        private ContinuationTask(
            final EventTask task,
            final RegisteredEventHandler[] registrations,
            final @Nullable CompletableFuture<E> future,
            final E event,
            final int index,
            final boolean currentlyAsync
        ) {
            this.task = task;
            this.registrations = registrations;
            this.future = future;
            this.event = event;
            this.index = index;
            this.currentlyAsync = currentlyAsync;
            this.firedOnThread = Thread.currentThread();
        }

        @Override
        public void run() {
            if (execute()) {
                callEventHandlers(
                    event,
                    future,
                    index + 1,
                    currentlyAsync,
                    registrations
                );
            }
        }

        /**
         * Executes the task and returns whether the next handler should be executed immediately
         * after this one, without additional scheduling.
         */
        boolean execute() {
            state = TASK_STATE_EXECUTING;
            try {
                task.execute(this);
            } catch (final Throwable t) {
                // validateOnlyOnce false here so don't get an exception if the
                // continuation was resumed before
                resume(t, false);
            }
            return !CONTINUATION_TASK_STATE.compareAndSet(
                this,
                TASK_STATE_EXECUTING,
                TASK_STATE_DEFAULT
            );
        }

        @Override
        public void resume() {
            resume(null, true);
        }

        void resume(
            final @Nullable Throwable exception,
            final boolean validateOnlyOnce
        ) {
            final boolean changed = CONTINUATION_TASK_RESUMED.compareAndSet(
                this,
                false,
                true
            );
            // Only allow the continuation to be resumed once
            if (!changed && validateOnlyOnce) {
                throw new IllegalStateException(
                    "The continuation can only be resumed once."
                );
            }
            final RegisteredEventHandler registration = registrations[index];
            if (exception != null) {
                logHandlerException(registration, exception);
            }
            if (!changed) {
                return;
            }
            if (index + 1 == registrations.length) {
                // Optimization: don't schedule a task just to complete the future
                if (future != null) {
                    future.complete(event);
                }
                return;
            }
            if (
                !CONTINUATION_TASK_STATE.compareAndSet(
                    this,
                    TASK_STATE_EXECUTING,
                    TASK_STATE_CONTINUE_IMMEDIATELY
                )
            ) {
                // We established earlier that registrations[index + 1] is a valid index.
                // If we are remaining in the same thread for the next handler, fire
                // the next event immediately, else fire it within the executor service
                // of the plugin with the next handler.
                final RegisteredEventHandler next = registrations[index + 1];
                final Thread currentThread = Thread.currentThread();
                if (
                    currentThread == firedOnThread &&
                    next.asyncType != AsyncLevel.Full
                ) {
                    callEventHandlers(
                        event,
                        future,
                        index + 1,
                        currentlyAsync,
                        registrations
                    );
                } else {
                    next.plugin
                        .getExecutorService()
                        .execute(() ->
                            fire(future, event, index + 1, true, registrations)
                        );
                }
            }
        }

        @Override
        public void resumeWithException(final Throwable exception) {
            resume(requireNonNull(exception, "exception"), true);
        }
    }

    // Copied from velocity-proxy
    // VelocityEventManager.java#702
    private static void logHandlerException(
        final RegisteredEventHandler registration,
        final Throwable t
    ) {
        final PluginDescription pluginDescription =
            registration.pluginContainer.getDescription();
        logger.error(
            "Couldn't pass {} to {} {}",
            registration.eventType.getSimpleName(),
            pluginDescription.getId(),
            pluginDescription.getVersion().orElse(""),
            t
        );
    }
}
