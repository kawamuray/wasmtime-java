package io.github.kawamuray.wasmtime;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.Properties;

import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public final class NativeLibraryLoader {
    private static final String NATIVE_LIBRARY_NAME = "wasmtime_jni";
    private static final String DISABLE_AUTO_LOAD_ENV = "WASMTIME_JNI_LOAD_DISABLED";
    private static final String META_PROPS_FILE = "wasmtime-java-meta.properties";
    private static final String JNI_LIB_VERSION_PROP = "jnilib.version";
    private static boolean loaded;

    private NativeLibraryLoader() {}

    public static synchronized void init() {
        if (System.getenv(DISABLE_AUTO_LOAD_ENV) == null) {
            load();
        }
    }

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
        Platform platform = detectPlatform();
        String version = libVersion();
        String ext = platform.ext;
        String fileName = platform.prefix + NATIVE_LIBRARY_NAME + '_' + version + '_' + platform.os.value + '_' + platform.arch.value;
        Path tempFile = Files.createTempFile(fileName, ext);
        try (InputStream in = NativeLibraryLoader.class.getResourceAsStream('/' + fileName + ext)) {
            Files.copy(in, tempFile, StandardCopyOption.REPLACE_EXISTING);
        }
        return tempFile.toString();
    }

    private static String libVersion() throws IOException {
        final Properties props;
        try (InputStream in = NativeLibraryLoader.class.getResourceAsStream('/' + META_PROPS_FILE)) {
            props = new Properties();
            props.load(in);
        }
        return props.getProperty(JNI_LIB_VERSION_PROP);
    }

    @AllArgsConstructor
    private enum Os {
        LINUX("linux"),
        MACOS("macos"),
        WINDOWS("windows");

        final String value;
    }

    @AllArgsConstructor
    private enum Arch {
        X86_64("x86_64"),
        AARCH64("aarch64");

        final String value;
    }

    @AllArgsConstructor
    private enum Platform {
        LINUX(Os.LINUX, Arch.X86_64, "lib", ".so"),
        LINUX_AARCH64(Os.LINUX, Arch.AARCH64, "lib", ".so"),
        MACOS(Os.MACOS, Arch.X86_64, "lib", ".dylib"),
        MACOS_AARCH64(Os.MACOS, Arch.AARCH64, "lib", ".dylib"),
        WINDOWS(Os.WINDOWS, Arch.X86_64, "", ".dll");

        final Os os;
        final Arch arch;
        final String prefix;
        final String ext;
    }

    private static Platform detectPlatform() {
        String os = System.getProperty("os.name").toLowerCase();
        String arch = System.getProperty("os.arch").toLowerCase();
        if (os.contains("linux")) {
            if (arch.equals("aarch64")) {
                return Platform.LINUX_AARCH64;
            }
            return Platform.LINUX;
        }
        if (os.contains("mac os") || os.contains("darwin")) {
            if (arch.equals("aarch64")) {
                return Platform.MACOS_AARCH64;
            }
            return Platform.MACOS;
        }
        if (os.toLowerCase().contains("windows")) {
            return Platform.WINDOWS;
        }
        throw new RuntimeException("platform not supported: " + os);
    }
}
