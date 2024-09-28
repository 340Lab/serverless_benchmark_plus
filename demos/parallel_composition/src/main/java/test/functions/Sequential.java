package test.functions;


import com.google.gson.JsonObject;

import test.Alu;

public class Sequential {
    private static final int DEFAULT_LOOP_TIME = 10000000;

    public JsonObject call(JsonObject args) {
        int loopTime = DEFAULT_LOOP_TIME;
        if (args.has("loopTime")) {
            loopTime = args.get("loopTime").getAsInt();
        }
        
        Alu alu = new Alu();
        double temp = alu.singleAlu(loopTime);

        return new JsonObject();
    }
}