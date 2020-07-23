package wasmtime;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

import lombok.extern.slf4j.Slf4j;

@Slf4j
final class NativeLibraryLoader {
    private static final String NATIVE_LIBRARY_NAME = "wasmtime_jni";
    private static boolean loaded;

    private NativeLibraryLoader() {}

    public static synchronized void load() {
        if (loaded) {
            return;
        }
        if (tryLoadFromLibraryPath()) {
            log.debug("Wasmtime JNI library loaded from library.path");
            loaded = true;
            return;
        }

        final String libraryPath;
        try {
            libraryPath = libraryPath();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        log.debug("Loading Wasmtime JNI library from {}", libraryPath);
        System.load(libraryPath);
        loaded = true;
    }

    private static boolean tryLoadFromLibraryPath() {
        try {
            System.loadLibrary(NATIVE_LIBRARY_NAME);
        } catch (UnsatisfiedLinkError ignored) {
            return false;
        }
        return true;
    }

    private static String libraryPath() throws IOException {
        String libName = "lib" + NATIVE_LIBRARY_NAME;
        String ext = platformExtension();
        Path tempFile = Files.createTempFile(libName, ext);
        try (InputStream in = NativeLibraryLoader.class.getResourceAsStream('/' + libName + ext)) {
            Files.copy(in, tempFile, StandardCopyOption.REPLACE_EXISTING);
        }
        return tempFile.toString();
    }

    private static String platformExtension() {
        String os = System.getProperty("os.name").toLowerCase();
        if (os.contains("nix") || os.contains("nux")) {
            return ".so";
        }
        if (os.contains("mac")) {
            return ".dylib";
        }
        throw new RuntimeException("platform not supported: " + os);
    }
}
