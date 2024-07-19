package test.functions;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import com.google.gson.JsonObject;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import io.minio.GetObjectArgs;// ！！！请勿修改此文件，此文件由脚本生成
import io.minio.MinioClient;// ！！！请勿修改此文件，此文件由脚本生成
import io.minio.PutObjectArgs;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import java.io.ByteArrayInputStream;// ！！！请勿修改此文件，此文件由脚本生成
import java.io.IOException;// ！！！请勿修改此文件，此文件由脚本生成
import java.io.InputStream;// ！！！请勿修改此文件，此文件由脚本生成
import java.nio.ByteBuffer;// ！！！请勿修改此文件，此文件由脚本生成
import java.nio.charset.StandardCharsets;// ！！！请勿修改此文件，此文件由脚本生成
import java.util.Arrays;// ！！！请勿修改此文件，此文件由脚本生成
import java.util.Map;// ！！！请勿修改此文件，此文件由脚本生成
import java.util.concurrent.ConcurrentHashMap;// ！！！请勿修改此文件，此文件由脚本生成
import java.util.concurrent.atomic.AtomicInteger;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
public class Count {// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
    private static final int BUFFER_SIZE = 1024 * 1024; // 1MB// ！！！请勿修改此文件，此文件由脚本生成
    private static final Map<String, byte[]> KV_STORE = new ConcurrentHashMap<>();// ！！！请勿修改此文件，此文件由脚本生成
    MinioClient minioClient = // ！！！请勿修改此文件，此文件由脚本生成
        MinioClient.builder()// ！！！请勿修改此文件，此文件由脚本生成
            .endpoint("http://192.168.31.96:9009")// ！！！请勿修改此文件，此文件由脚本生成
            .credentials("minioadmin", "minioadmin123")// ！！！请勿修改此文件，此文件由脚本生成
            .build();// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
    public void splitFile(InputStream inputStream, AtomicInteger sliceIdCounter) throws IOException {// ！！！请勿修改此文件，此文件由脚本生成
        byte[] buffer = new byte[BUFFER_SIZE];// ！！！请勿修改此文件，此文件由脚本生成
        int sliceId = sliceIdCounter.get();// ！！！请勿修改此文件，此文件由脚本生成
        KV_STORE.clear();// ！！！请勿修改此文件，此文件由脚本生成
        // ！！！请勿修改此文件，此文件由脚本生成
        int bytesRead;// ！！！请勿修改此文件，此文件由脚本生成
        int offset = 0;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        while ((bytesRead = inputStream.read(buffer, offset, BUFFER_SIZE - offset)) != -1) {// ！！！请勿修改此文件，此文件由脚本生成
            int totalBytes = offset + bytesRead;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // If buffer is full or reached end of file, store the slice// ！！！请勿修改此文件，此文件由脚本生成
            if (totalBytes == BUFFER_SIZE || bytesRead == -1) {// ！！！请勿修改此文件，此文件由脚本生成
                byte[] slice = new byte[totalBytes];// ！！！请勿修改此文件，此文件由脚本生成
                System.arraycopy(buffer, 0, slice, 0, totalBytes);// ！！！请勿修改此文件，此文件由脚本生成
                String key = "wordcount_slice_" + sliceId++;// ！！！请勿修改此文件，此文件由脚本生成
                KV_STORE.put(key, slice);// ！！！请勿修改此文件，此文件由脚本生成
                System.out.println("Stored slice: " + key);// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
                // Reset the offset for the next read// ！！！请勿修改此文件，此文件由脚本生成
                offset = 0;// ！！！请勿修改此文件，此文件由脚本生成
            } else {// ！！！请勿修改此文件，此文件由脚本生成
                // Adjust offset for the next read// ！！！请勿修改此文件，此文件由脚本生成
                offset = totalBytes;// ！！！请勿修改此文件，此文件由脚本生成
            }// ！！！请勿修改此文件，此文件由脚本生成
        }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        // Store any remaining data// ！！！请勿修改此文件，此文件由脚本生成
        if (offset > 0) {// ！！！请勿修改此文件，此文件由脚本生成
            byte[] lastSlice = new byte[offset];// ！！！请勿修改此文件，此文件由脚本生成
            System.arraycopy(buffer, 0, lastSlice, 0, offset);// ！！！请勿修改此文件，此文件由脚本生成
            String key = "wordcount_slice_" + sliceId++;// ！！！请勿修改此文件，此文件由脚本生成
            KV_STORE.put(key, lastSlice);// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("Stored last slice: " + key);// ！！！请勿修改此文件，此文件由脚本生成
        }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        sliceIdCounter.set(sliceId); // Update the sliceId counter// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
    public JsonObject call(JsonObject args) {// ！！！请勿修改此文件，此文件由脚本生成
        JsonObject result = new JsonObject();// ！！！请勿修改此文件，此文件由脚本生成
        AtomicInteger sliceIdCounter = new AtomicInteger(0);// ！！！请勿修改此文件，此文件由脚本生成
        try {// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("Received arguments: " + args.toString());// ！！！请勿修改此文件，此文件由脚本生成
            String textS3Path = args.get("text_s3_path").getAsString();// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("Downloading file from MinIO: " + textS3Path);// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // Download the file from MinIO// ！！！请勿修改此文件，此文件由脚本生成
            GetObjectArgs getObjectArgs = GetObjectArgs.builder()// ！！！请勿修改此文件，此文件由脚本生成
                    .bucket("serverless-bench")// ！！！请勿修改此文件，此文件由脚本生成
                    .object(textS3Path)// ！！！请勿修改此文件，此文件由脚本生成
                    .build();// ！！！请勿修改此文件，此文件由脚本生成
            InputStream inputStream = minioClient.getObject(getObjectArgs);// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("File downloaded successfully from MinIO");// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // Split the file// ！！！请勿修改此文件，此文件由脚本生成
            splitFile(inputStream, sliceIdCounter);// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("File split into " + KV_STORE.size() + " slices");// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // Upload each slice to MinIO with a unique ID// ！！！请勿修改此文件，此文件由脚本生成
            for (Map.Entry<String, byte[]> entry : KV_STORE.entrySet()) {// ！！！请勿修改此文件，此文件由脚本生成
                String key = entry.getKey();// ！！！请勿修改此文件，此文件由脚本生成
                byte[] value = entry.getValue();// ！！！请勿修改此文件，此文件由脚本生成
                ByteArrayInputStream sliceInputStream = new ByteArrayInputStream(value);// ！！！请勿修改此文件，此文件由脚本生成
                System.out.println("Uploading slice: " + key);// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
                // Upload slice to MinIO// ！！！请勿修改此文件，此文件由脚本生成
                minioClient.putObject(// ！！！请勿修改此文件，此文件由脚本生成
                        PutObjectArgs.builder()// ！！！请勿修改此文件，此文件由脚本生成
                                .bucket("serverless-bench")// ！！！请勿修改此文件，此文件由脚本生成
                                .object(key)// ！！！请勿修改此文件，此文件由脚本生成
                                .stream(sliceInputStream, value.length, -1)// ！！！请勿修改此文件，此文件由脚本生成
                                .contentType("text/plain")// ！！！请勿修改此文件，此文件由脚本生成
                                .build()// ！！！请勿修改此文件，此文件由脚本生成
                );// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
                // Add each slice key to result// ！！！请勿修改此文件，此文件由脚本生成
                result.addProperty("slice_" + key, key);// ！！！请勿修改此文件，此文件由脚本生成
                System.out.println("Slice uploaded successfully: " + key);// ！！！请勿修改此文件，此文件由脚本生成
            }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            System.out.println("All slices uploaded successfully");// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            return result;// ！！！请勿修改此文件，此文件由脚本生成
        } catch (Exception e) {// ！！！请勿修改此文件，此文件由脚本生成
            e.printStackTrace();// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("error", "An error occurred while processing the file: " + e.getMessage());// ！！！请勿修改此文件，此文件由脚本生成
            return result;// ！！！请勿修改此文件，此文件由脚本生成
        }// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
}// ！！！请勿修改此文件，此文件由脚本生成
