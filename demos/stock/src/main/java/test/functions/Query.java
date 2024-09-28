package test.functions;

import java.util.List;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import manage.mapper.StockMngMapper;
import manage.model.DatabaseSeed;
import manage.model.ProductInfo;
import manage.session.SqlSessionFactoryUtil;

public class Query {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();

        JsonObject result = new JsonObject();

        // 使用 try-with-resources 来管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getStockDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            StockMngMapper stockMngMapper = session.getMapper(StockMngMapper.class);

            // 检查该用户的库存记录数量
            Integer stockRecordCount = stockMngMapper.getStockRecordCountForUserName(userName);
            if (stockRecordCount == null) {
                initUser(stockMngMapper, userName);
                session.commit();  // 初始化用户后提交事务
            }

            // 查询该用户的商品信息
            List<ProductInfo> productList = stockMngMapper.query(userName);

            // 将查询结果转换为 JsonObject
            JsonArray productsArray = new JsonArray();
            for (ProductInfo product : productList) {
                JsonObject productObj = new JsonObject();
                productObj.addProperty("productCode", product.getProductCode());
                productObj.addProperty("name", product.getName());
                productObj.addProperty("description", product.getDescription());
                productObj.addProperty("price", product.getPrice().toString());
                productObj.addProperty("stockCount", product.getStockCount());
                productsArray.add(productObj);
            }

            result.add("products", productsArray);  // 将所有商品信息添加到结果中
            session.commit();  // 提交查询后的事务

        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Query failed: " + e.getMessage());
        }

        return result;
    }

    private void initUser(StockMngMapper stockMngMapper, String userName) {
        for (int i = 0; i < 5; i++) {
            stockMngMapper.insertStockRecord(
                    "0000" + (i + 1),
                    DatabaseSeed.name[i],
                    DatabaseSeed.description[i],
                    DatabaseSeed.price[i],
                    10000,
                    userName);
        }
    }

}
