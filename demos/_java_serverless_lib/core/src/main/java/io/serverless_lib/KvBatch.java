package io.serverless_lib;

import java.util.List;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.context.annotation.Lazy;

import process_rpc_proto.ProcessRpcProto.KvRequest;
import process_rpc_proto.ProcessRpcProto.KvRequests;
import process_rpc_proto.ProcessRpcProto.KvResponses;
import process_rpc_proto.ProcessRpcProto.KvRequest.KvPutRequest;
import process_rpc_proto.ProcessRpcProto.KvRequest.KvGetRequest;
import process_rpc_proto.ProcessRpcProto.KvRequest.KvDeleteRequest;
import process_rpc_proto.ProcessRpcProto.KvRequest.KvLockRequest;
import process_rpc_proto.ProcessRpcProto.KvPair;
import process_rpc_proto.ProcessRpcProto.KeyRange;


public class KvBatch {
    private KvRequests batchArgs;    // 用于发送批处理请求的KvRequests
    private KvResponses result;      // 用于接收响应的KvResponses
    private int taskId;              // 任务 ID

    @Autowired 
    UdsBackend uds;

    // 构造函数，初始化 KvRequests 和 KvResponses
    public KvBatch(String app, String func, int taskId) {
        this.batchArgs = KvRequests.newBuilder().setApp(app).setFunc(func).setPrevKvOpeid(8888).build();
        this.result = KvResponses.newBuilder().build();
        this.taskId = taskId;
        // TODO 注入this
        // uds.kvBatch = this;
    }

    // 设置批处理请求
    public void setBatchArgs(String app, String func, List<KvRequest> requests, long prevKvOpeId) {
        KvRequests.Builder builder = KvRequests.newBuilder();
        builder.setApp(app).setFunc(func).addAllRequests(requests).setPrevKvOpeid(prevKvOpeId);
        this.batchArgs = builder.build();
    }

    // 添加单个 PUT 请求
    public void addPutRequest(byte[] key, byte[] value) {
        KvPair kvPair = KvPair.newBuilder().setKey(com.google.protobuf.ByteString.copyFrom(key))
                                         .setValue(com.google.protobuf.ByteString.copyFrom(value)).build();
        KvPutRequest putRequest = KvPutRequest.newBuilder().setKv(kvPair).build();
        KvRequest request = KvRequest.newBuilder().setSet(putRequest).build();

        this.batchArgs = this.batchArgs.toBuilder().addRequests(request).build();
    }

    // 添加单个 GET 请求
    public void addGetRequest(byte[] startKey, byte[] endKey) {
        KeyRange range = KeyRange.newBuilder().setStart(com.google.protobuf.ByteString.copyFrom(startKey))
                                          .setEnd(com.google.protobuf.ByteString.copyFrom(endKey)).build();
        KvGetRequest getRequest = KvGetRequest.newBuilder().setRange(range).build();
        KvRequest request = KvRequest.newBuilder().setGet(getRequest).build();

        this.batchArgs = this.batchArgs.toBuilder().addRequests(request).build();
    }

    // 添加单个 DELETE 请求
    public void addDeleteRequest(byte[] startKey, byte[] endKey) {
        KeyRange range = KeyRange.newBuilder().setStart(com.google.protobuf.ByteString.copyFrom(startKey))
                                          .setEnd(com.google.protobuf.ByteString.copyFrom(endKey)).build();
        KvDeleteRequest deleteRequest = KvDeleteRequest.newBuilder().setRange(range).build();
        KvRequest request = KvRequest.newBuilder().setDelete(deleteRequest).build();

        this.batchArgs = this.batchArgs.toBuilder().addRequests(request).build();
    }

    // 添加单个 LOCK 请求
    public void addLockRequest(boolean readOrWrite, List<Integer> releaseIds, byte[] startKey, byte[] endKey) {
        KeyRange range = KeyRange.newBuilder().setStart(com.google.protobuf.ByteString.copyFrom(startKey))
                                          .setEnd(com.google.protobuf.ByteString.copyFrom(endKey)).build();
        KvLockRequest.Builder lockBuilder = KvLockRequest.newBuilder().setReadOrWrite(readOrWrite).setRange(range);
        lockBuilder.addAllReleaseId(releaseIds);
        KvRequest request = KvRequest.newBuilder().setLock(lockBuilder.build()).build();

        this.batchArgs = this.batchArgs.toBuilder().addRequests(request).build();
    }

    // 发送请求并接收响应
    public void sendBatch() throws Exception {
        // 打包成 UdsPack 发送数据
        UdsPack pack = new UdsPack(this.batchArgs, this.taskId);

        System.out.println("KvBatch sendBatch");
        // 发送请求
        uds.send(pack);

        // TODO 从 udsbackend 接收结果数据
        // this.result = ;
    }

    // 获取请求内容
    public KvRequests getBatchArgs() {
        return this.batchArgs;
    }

    public Integer getTackId() {
        return this.taskId;
    }

    // 获取响应内容
    public KvResponses getResult() {
        return this.result;
    }

    public void setResult(KvResponses result){
        this.result = result;
    }
}



/* 
用法示例：
KvBatch kvBatch = new KvBatch();
kvBatch.addPutRequest("key1".getBytes(), "value1".getBytes());
kvBatch.addGetRequest("key1".getBytes(), "key2".getBytes());
KvResponses responses = kvBatch.sendBatch(); 
*/
