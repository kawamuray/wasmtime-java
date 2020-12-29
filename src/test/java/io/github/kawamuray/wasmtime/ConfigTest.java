package io.github.kawamuray.wasmtime;

import org.junit.Assert;
import org.junit.Test;

public class ConfigTest {
    @Test
    public void testNewConfig(){
        try(Config config = new Config()){
            Assert.assertNotEquals(config.innerPtr(),0);
        }
    }
    @Test
    public void testCacheConfigLoadDefault(){
        try(Config config = new Config()){
            config.cacheConfigLoadDefault();
        }
    }
    @Test
    public void testStrategy(){
        try(Config config = new Config()){
            config.strategy(Strategy.AUTO).strategy(Strategy.CRANELIFT);
        }
    }
    @Test
    public void testOptLevel(){
        try(Config config = new Config();
            Config config1 = new Config();
            Config config2 = new Config();
            Config config3 = new Config();
            Config config4 = new Config();
            Config config5 = new Config()){
            config.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.SPEED_AND_SIZE);
            config1.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.SPEED);
            config2.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.NONE);
            config3.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.SPEED_AND_SIZE);
            config4.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.SPEED);
            config5.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.NONE);
        }
    }

    @Test
    public void testCustomConfig(){
        try(Config config = new Config()) {
            config.strategy(Strategy.CRANELIFT)
                    .craneliftOptLevel(OptLevel.SPEED_AND_SIZE)
                    .debugInfo(true);
            try (Engine engine = new Engine(config);
                 Store store = new Store(engine)) {
                 store.engine();
            }
        }
    }
}
