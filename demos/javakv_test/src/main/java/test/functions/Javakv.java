package test.functions;

import java.util.Arrays;
import com.google.gson.JsonObject;
import io.serverless_lib.KvBatch;
import org.springframework.context.ApplicationContext;
import org.springframework.stereotype.Component;
import javax.annotation.PostConstruct;

@Component
public class Javakv {

    private final ApplicationContext applicationContext;
    private KvBatch kvBatch;

    public Javakv(ApplicationContext applicationContext) {
        this.applicationContext = applicationContext;
    }

    @PostConstruct
    public void init() {
        // 在上下文初始化完成后获取 KvBatch
        this.kvBatch = applicationContext.getBean(KvBatch.class);
    }

    public JsonObject call(JsonObject args, ApplicationContext applicationContext) {
        if (kvBatch == null) {
            throw new IllegalStateException("KvBatch is not initialized.");
        }

        JsonObject result = new JsonObject();

        try {
            // 从 Spring 上下文中获取 KvBatch 实例
            // KvBatch kvBatch = applicationContext.getBean(KvBatch.class);

            // 使用 KvBatch 实例执行操作
            kvBatch.addGetRequest("get_start".getBytes(), "get_end".getBytes());
            kvBatch.addPutRequest("put_key".getBytes(), "put_value".getBytes());
            kvBatch.addDeleteRequest("delete_start".getBytes(), "delete_end".getBytes());
            kvBatch.addLockRequest(true, Arrays.asList(0), "read_lock_start".getBytes(), "read_lock_end".getBytes());
            kvBatch.addLockRequest(false, Arrays.asList(1), "write_lock_start".getBytes(), "write_lock_end".getBytes());

            // 发送批量请求
            kvBatch.sendBatch();

            // 等待5s，等待服务端处理完成
            Thread.sleep(5000);

            // 记录结果
            System.out.println("Get result: " + kvBatch.getResult().toString());
            result.addProperty("KvBatch result", kvBatch.getResult().toString());
        } catch (Exception e) {
            e.printStackTrace();
            result.addProperty("error", e.getMessage());
        }

        return result;
    }
}
