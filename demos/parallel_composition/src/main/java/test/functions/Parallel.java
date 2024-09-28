package test.functions;

import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import com.google.gson.JsonObject;

import test.Alu;

public class Parallel {
    private static final int DEFAULT_LOOP_TIME = 10000000;
    private static final int DEFAULT_PARALLEL_INDEX = 100;

    public static final ExecutorService executor;
    
    static {
        executor = Executors.newFixedThreadPool(8);
    }
    
    public JsonObject call(JsonObject args) {
        int loopTime = DEFAULT_LOOP_TIME;
        if (args.has("loopTime")) {
            loopTime = args.get("loopTime").getAsInt();
        }
        int parallelIndex = DEFAULT_PARALLEL_INDEX;
        if (args.has("parallelIndex")) {
            parallelIndex = args.get("parallelIndex").getAsInt();
        }

        int times = loopTime / parallelIndex;


        Alu alu = new Alu();
        List<CompletableFuture<Void>> futures = IntStream.range(0, parallelIndex)
                .mapToObj(i -> CompletableFuture.runAsync(() -> alu.singleAlu(times), executor))
                .collect(Collectors.toList());

        CompletableFuture.allOf(futures.toArray(new CompletableFuture[0])).join();

        // executor.shutdown();

        return new JsonObject();
    }
}
