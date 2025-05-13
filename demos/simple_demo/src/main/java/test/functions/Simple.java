package test.functions;

import com.google.gson.JsonObject;

public class Simple {
    public JsonObject call(JsonObject input) {
        JsonObject result = new JsonObject();
        result.addProperty("message", "Hello from Simple function!");
        return result;
    }
}