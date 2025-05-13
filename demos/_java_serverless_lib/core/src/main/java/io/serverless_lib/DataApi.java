package io.serverless_lib;

import io.minio.BucketExistsArgs;
import io.minio.GetObjectArgs;
import io.minio.MinioClient;
import io.minio.MakeBucketArgs;
import io.minio.PutObjectArgs;
import io.minio.RemoveObjectArgs;
import io.minio.errors.MinioException;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.util.concurrent.ConcurrentHashMap;
import java.util.Map;

/**
 * DataApi用于处理数据存储，支持内存模式和MinIO模式
 */
public class DataApi {
    
    private volatile boolean initialized = false;
    private Map<String, byte[]> memoryStorage;
    private MinioClient minioClient;
    private String storageBucket;
    private String storageMode;
    
    /**
     * 延迟初始化函数，使用线程安全的双重检查锁定模式
     * @param use_minio MinIO连接字符串，为空时使用内存Map存储
     */
    public void init(String use_minio) {
        // 第一次检查，避免每次调用都加锁
        if (initialized) {
            return;
        }
        
        // 使用同步块保护初始化过程
        synchronized (this) {
            // 二次检查，防止其他线程已经完成了初始化
            if (initialized) {
                return;
            }
            
            if (use_minio == null || use_minio.isEmpty()) {
                // 使用内存Map模式
                memoryStorage = new ConcurrentHashMap<>();
                storageMode = "memory";
                System.out.println("DataApi initialized in memory mode");
            } else {
                // 连接到MinIO
                try {
                    // 解析MinIO连接字符串 (格式示例: "http://localhost:9000,accessKey,secretKey,bucketName")
                    String[] parts = use_minio.split(",");
                    String endpoint = parts[0];
                    String accessKey = parts.length > 1 ? parts[1] : "";
                    String secretKey = parts.length > 2 ? parts[2] : "";
                    storageBucket = parts.length > 3 ? parts[3] : "serverless-bench";
                    
                    minioClient = MinioClient.builder()
                            .endpoint(endpoint)
                            .credentials(accessKey, secretKey)
                            .build();
                    
                    // 确保存储桶存在
                    boolean found = minioClient.bucketExists(BucketExistsArgs.builder().bucket(storageBucket).build());
                    if (!found) {
                        minioClient.makeBucket(MakeBucketArgs.builder().bucket(storageBucket).build());
                    }
                    
                    storageMode = "minio";
                    System.out.println("DataApi initialized in MinIO mode: " + endpoint);
                } catch (Exception e) {
                    // 连接MinIO失败，回退到内存模式
                    memoryStorage = new ConcurrentHashMap<>();
                    storageMode = "memory";
                    System.err.println("Failed to connect to MinIO, fallback to memory mode: " + e.getMessage());
                }
            }
            
            // 最后设置初始化标志
            initialized = true;
        }
    }
    
    /**
     * 保存数据
     * @param key 键
     * @param value 字节数组值
     * @throws IOException 操作失败时抛出异常
     */
    public void put(String key, byte[] value) throws IOException {
        if (!initialized) {
            throw new IllegalStateException("DataApi not initialized. Call init() first.");
        }
        
        if ("memory".equals(storageMode)) {
            memoryStorage.put(key, value);
        } else {
            try {
                // 将字节数组上传到MinIO
                ByteArrayInputStream bais = new ByteArrayInputStream(value);
                minioClient.putObject(
                    PutObjectArgs.builder()
                        .bucket(storageBucket)
                        .object(key)
                        .stream(bais, value.length, -1)
                        .contentType("application/octet-stream")
                        .build()
                );
                bais.close();
            } catch (MinioException | InvalidKeyException | NoSuchAlgorithmException e) {
                throw new IOException("Failed to put object to MinIO: " + e.getMessage(), e);
            }
        }
    }
    
    /**
     * 获取数据
     * @param key 键
     * @return 字节数组值，如果键不存在则返回null
     * @throws IOException 操作失败时抛出异常
     */
    public byte[] get(String key) throws IOException {
        if (!initialized) {
            throw new IllegalStateException("DataApi not initialized. Call init() first.");
        }
        
        if ("memory".equals(storageMode)) {
            return memoryStorage.get(key);
        } else {
            try {
                // 从MinIO获取对象
                ByteArrayOutputStream baos = new ByteArrayOutputStream();
                InputStream stream = minioClient.getObject(
                    GetObjectArgs.builder()
                        .bucket(storageBucket)
                        .object(key)
                        .build()
                );
                
                // 读取字节到输出流
                byte[] buffer = new byte[16384];
                int bytesRead;
                while ((bytesRead = stream.read(buffer)) > 0) {
                    baos.write(buffer, 0, bytesRead);
                }
                
                stream.close();
                baos.close();
                return baos.toByteArray();
                
            } catch (MinioException | InvalidKeyException | NoSuchAlgorithmException e) {
                throw new IOException("Failed to get object from MinIO: " + e.getMessage(), e);
            }
        }
    }
    
    /**
     * 删除数据
     * @param key 要删除的键
     * @throws IOException 操作失败时抛出异常
     */
    public void delete(String key) throws IOException {
        if (!initialized) {
            throw new IllegalStateException("DataApi not initialized. Call init() first.");
        }
        
        if ("memory".equals(storageMode)) {
            memoryStorage.remove(key);
        } else {
            try {
                // 从MinIO删除对象
                minioClient.removeObject(
                    RemoveObjectArgs.builder()
                        .bucket(storageBucket)
                        .object(key)
                        .build()
                );
            } catch (MinioException | InvalidKeyException | NoSuchAlgorithmException e) {
                throw new IOException("Failed to delete object from MinIO: " + e.getMessage(), e);
            }
        }
    }
    
    /**
     * 检查是否已初始化
     */
    public boolean isInitialized() {
        return initialized;
    }
    
    /**
     * 获取当前存储模式
     */
    public String getStorageMode() {
        return storageMode;
    }
} 