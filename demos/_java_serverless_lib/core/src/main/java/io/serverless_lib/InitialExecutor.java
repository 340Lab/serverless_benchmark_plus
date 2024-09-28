package io.serverless_lib;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class InitialExecutor {
    public static final ExecutorService executor;
    
    static {
        executor = Executors.newFixedThreadPool(5);
    }
}
