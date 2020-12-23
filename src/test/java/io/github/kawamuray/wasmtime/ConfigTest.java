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
        config.cacheConfigLoadDefault();
    }
    @Test
    public void testStrategy(){
        Config config = new Config();
        config.strategy(Strategy.AUTO).strategy(Strategy.CRANELIFT);
    }
    @Test
    public void testOptLevel(){
        Config config = new Config();
        config.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.SPEED_AND_SIZE);
        Config config1 = new Config();
        config1.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.SPEED);
        Config config2 = new Config();
        config2.strategy(Strategy.CRANELIFT).craneliftOptLevel(OptLevel.NONE);
        Config config3 = new Config();
        config3.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.SPEED_AND_SIZE);
        Config config4 = new Config();
        config4.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.SPEED);
        Config config5 = new Config();
        config5.strategy(Strategy.AUTO).craneliftOptLevel(OptLevel.NONE);
    }

    @Test
    public void testCustomConfig(){
        Config config = new Config();
        config.strategy(Strategy.CRANELIFT)
                .craneliftOptLevel(OptLevel.SPEED_AND_SIZE)
                .debugInfo(true);
        Engine engine = new Engine(config);
        Store store = new Store(engine);
        store.engine();
    }
}
