package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.StockMngMapper;
import manage.session.SqlSessionFactoryUtil;

public class CreateOrder {

    public JsonObject call(JsonObject args) {

        String userName = args.get("userName").getAsString();
        String productCode = args.get("productCode").getAsString();
        Integer count = args.get("count").getAsInt();

        JsonObject result = new JsonObject();

        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getStockDbSessionFactory();
        if (sqlSessionFactory == null) {
            throw new IllegalStateException("SqlSessionFactory not initialized.");
        }

        // 使用 try-with-resources 语句自动管理 SqlSession
        try (SqlSession session = sqlSessionFactory.openSession()) {
            StockMngMapper stockMngMapper = session.getMapper(StockMngMapper.class);
            result.addProperty("success", stockMngMapper.createOrder(userName, productCode, count) > 0);
            session.commit(); 
        } catch (Exception e) {
            result.addProperty("success", false); 
        }

        return result;
    }

}
