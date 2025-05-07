package test;

import com.google.gson.JsonObject;
import test.functions.Simple;

public class Application {
    public static void main(String[] args) {
        Simple simple = new Simple();
        JsonObject input = new JsonObject();
        JsonObject result = simple.call(input);
        System.out.println(result.toString());
    }
} 