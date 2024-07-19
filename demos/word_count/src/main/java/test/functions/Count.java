package test.functions;

import com.google.gson.JsonObject;

import io.minio.GetObjectArgs;
import io.minio.MinioClient;
import io.minio.PutObjectArgs;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;

public class Count {

    private static final int BUFFER_SIZE = 1024 * 1024; // 1MB
    private static final Map<String, byte[]> KV_STORE = new ConcurrentHashMap<>();
    MinioClient minioClient = 
        MinioClient.builder()
            .endpoint("http://192.168.31.96:9009")
            .credentials("minioadmin", "minioadmin123")
            .build();

    public void splitFile(InputStream inputStream, AtomicInteger sliceIdCounter) throws IOException {
        byte[] buffer = new byte[BUFFER_SIZE];
        int sliceId = sliceIdCounter.get();
        KV_STORE.clear();
        
        int bytesRead;
        int offset = 0;

        while ((bytesRead = inputStream.read(buffer, offset, BUFFER_SIZE - offset)) != -1) {
            int totalBytes = offset + bytesRead;

            // If buffer is full or reached end of file, store the slice
            if (totalBytes == BUFFER_SIZE || bytesRead == -1) {
                byte[] slice = new byte[totalBytes];
                System.arraycopy(buffer, 0, slice, 0, totalBytes);
                String key = "wordcount_slice_" + sliceId++;
                KV_STORE.put(key, slice);
                System.out.println("Stored slice: " + key);

                // Reset the offset for the next read
                offset = 0;
            } else {
                // Adjust offset for the next read
                offset = totalBytes;
            }
        }

        // Store any remaining data
        if (offset > 0) {
            byte[] lastSlice = new byte[offset];
            System.arraycopy(buffer, 0, lastSlice, 0, offset);
            String key = "wordcount_slice_" + sliceId++;
            KV_STORE.put(key, lastSlice);
            System.out.println("Stored last slice: " + key);
        }

        sliceIdCounter.set(sliceId); // Update the sliceId counter
    }

    public JsonObject call(JsonObject args) {
        JsonObject result = new JsonObject();
        AtomicInteger sliceIdCounter = new AtomicInteger(0);
        try {
            System.out.println("Received arguments: " + args.toString());
            String textS3Path = args.get("text_s3_path").getAsString();
            System.out.println("Downloading file from MinIO: " + textS3Path);

            // Download the file from MinIO
            GetObjectArgs getObjectArgs = GetObjectArgs.builder()
                    .bucket("serverless-bench")
                    .object(textS3Path)
                    .build();
            InputStream inputStream = minioClient.getObject(getObjectArgs);
            System.out.println("File downloaded successfully from MinIO");

            // Split the file
            splitFile(inputStream, sliceIdCounter);
            System.out.println("File split into " + KV_STORE.size() + " slices");

            // Upload each slice to MinIO with a unique ID
            for (Map.Entry<String, byte[]> entry : KV_STORE.entrySet()) {
                String key = entry.getKey();
                byte[] value = entry.getValue();
                ByteArrayInputStream sliceInputStream = new ByteArrayInputStream(value);
                System.out.println("Uploading slice: " + key);

                // Upload slice to MinIO
                minioClient.putObject(
                        PutObjectArgs.builder()
                                .bucket("serverless-bench")
                                .object(key)
                                .stream(sliceInputStream, value.length, -1)
                                .contentType("text/plain")
                                .build()
                );

                // Add each slice key to result
                result.addProperty("slice_" + key, key);
                System.out.println("Slice uploaded successfully: " + key);
            }

            System.out.println("All slices uploaded successfully");

            return result;
        } catch (Exception e) {
            e.printStackTrace();
            result.addProperty("error", "An error occurred while processing the file: " + e.getMessage());
            return result;
        }
    }
}
