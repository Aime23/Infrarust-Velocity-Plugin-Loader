package dev.infrarust;

import io.github.jni_rs.jbindgen.RustSkip;

import java.lang.ref.Cleaner;

public abstract class NativeFinalize implements AutoCloseable {
    private static final Cleaner cleaner = Cleaner.create();
    private final Cleaner.Cleanable cleanable;

    protected NativeFinalize() {
        cleanable = cleaner.register(this, this::native_finalize);
    }

    private native void native_finalize();

    public void close() {
        cleanable.clean();
    }

}
