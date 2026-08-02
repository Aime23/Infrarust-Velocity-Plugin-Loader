import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.net.URLClassLoader;
import java.util.HashMap;
import java.util.Map;
import java.util.jar.JarEntry;
import java.util.jar.JarInputStream;

public class ByteArrayClassLoader extends ClassLoader {
    private final Map<String, byte[]> classBytes = new HashMap<>();

    public ByteArrayClassLoader(byte[] jarBytes, ClassLoader parent) throws IOException {
        super(parent);
        try (JarInputStream jarIn = new JarInputStream(new ByteArrayInputStream(jarBytes))) {
            JarEntry entry;
            while ((entry = jarIn.getNextJarEntry()) != null) {
                if (!entry.isDirectory() && entry.getName().endsWith(".class")) {
                    String className = entry.getName()
                        .replace('/', '.')
                        .replace(".class", "");
                    classBytes.put(className, jarIn.readAllBytes());
                }
                jarIn.closeEntry();
            }
        }
    }

    @Override
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        byte[] bytes = classBytes.get(name);
        if (bytes == null) {
            throw new ClassNotFoundException(name);
        }
        return defineClass(name, bytes, 0, bytes.length);
    }
}
