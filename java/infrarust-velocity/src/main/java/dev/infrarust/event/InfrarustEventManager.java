package dev.infrarust.event;

import static java.util.Objects.requireNonNull;

import com.velocitypowered.api.event.Continuation;
import com.velocitypowered.api.event.EventHandler;
import com.velocitypowered.api.event.EventManager;
import com.velocitypowered.api.event.EventTask;
import com.velocitypowered.api.event.PostOrder;
import com.velocitypowered.api.event.Subscribe;
import com.velocitypowered.api.plugin.PluginContainer;
import com.velocitypowered.api.plugin.PluginManager;
import dev.infrarust.NativeFinalize;
import io.github.jni_rs.jbindgen.RustPrimitive;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.stream.Collectors;

public class InfrarustEventManager
    extends NativeFinalize
    implements EventManager
{

    @RustPrimitive("crate::java::handle::PluginContextHandle")
    protected final long plugin_context_handle;

    private final PluginManager pluginManager;

    private final List<RegisteredEventHandler> registeredEventHandlers =
        new ArrayList<RegisteredEventHandler>();

    public InfrarustEventManager(
        @RustPrimitive(
            "crate::java::handle::PluginContextHandle"
        ) final long plugin_context_handle,
        final PluginManager pluginManager
    ) {
        this.plugin_context_handle = plugin_context_handle;
        this.pluginManager = pluginManager;
    }

    private native void native_finalize();

    private native void native_register_event_handler(
        RegisteredEventHandler registeredEventHandler
    );

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
        this.registeredEventHandlers.add(eventHandler);
        this.native_register_event_handler(eventHandler);
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
            handler,
            AsyncLevel.Full,
            priority
        );
        this.registerEventHandler(rehm);
    }

    @Override
    public <E> CompletableFuture<E> fire(final E event) {
        return null;
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
        final List<RegisteredEventHandler> removed = new ArrayList<>();
        try {
            final Iterator<RegisteredEventHandler> it =
                registeredEventHandlers.iterator();
            while (it.hasNext()) {
                final RegisteredEventHandler registration = it.next();
                if (predicate.test(registration)) {
                    it.remove();
                    removed.add(registration);
                }
            }
        } finally {
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
        private final EventHandler<?> eventHandler;
        private final AsyncLevel asyncType;
        private final short priority;

        public RegisteredEventHandler(
            final PluginContainer pluginContainer,
            final Class<?> eventClass,
            final EventHandler<?> eventHandler,
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
                eventHandler,
                eventHandlingMethod.asyncType,
                eventHandlingMethod.priority
            );
        }
    }
}
