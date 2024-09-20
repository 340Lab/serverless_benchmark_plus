package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.StockMngMapper;
import manage.session.SqlSessionFactoryUtil;

public class MinusStockCount {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();
        String productCode = args.get("productCode").getAsString();
        int count = args.get("count").getAsInt();

        JsonObject result = new JsonObject();

        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getStockDbSessionFactory();
        if (sqlSessionFactory == null) {
            throw new IllegalStateException("SqlSessionFactory not initialized.");
        }

        // 使用 try-with-resources 语句自动管理 SqlSession
        try (SqlSession session = sqlSessionFactory.openSession()) {
            StockMngMapper stockMngMapper = session.getMapper(StockMngMapper.class);
            boolean success = stockMngMapper.minusStockCount(userName, productCode, count) > 0;

            result.addProperty("success", success);  
            session.commit();  // 提交事务
        } catch (Exception e) {
            result.addProperty("success", false);  // 如果发生异常，返回失败
        }

        return result;
    }

}
