package io.github.kawamuray.wasmtime;

import org.junit.Test;

public class ConfigTest {
    @Test
    public void testNewConfig(){
        Config config = new Config();
        assert config.innerPtr()!=0;
    }
    @Test
    public void testCacheConfigLoadDefault(){
        Config config = new Config();
        assert config.innerPtr()!=0;
        config.cacheConfigLoadDefault();
    }
}
