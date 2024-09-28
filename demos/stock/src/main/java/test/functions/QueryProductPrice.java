package test.functions;

import java.math.BigDecimal;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.StockMngMapper;
import manage.session.SqlSessionFactoryUtil;

public class QueryProductPrice {

    public JsonObject call(JsonObject args) {
        String productCode = args.get("productCode").getAsString();
        String userName = args.get("userName").getAsString();
        JsonObject result = new JsonObject();

        // 使用 try-with-resources 来管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getStockDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            StockMngMapper stockMngMapper = session.getMapper(StockMngMapper.class);

            BigDecimal price = stockMngMapper.queryProductPrice(productCode, userName);
            if (price == null) {
                throw new RuntimeException("Product code or user name does not exist");
            }

            // 创建返回的JsonObject
            result.addProperty("price", price.toString());

            session.commit(); // 提交事务
        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Query failed: " + e.getMessage());
        }

        return result;
    }

}
