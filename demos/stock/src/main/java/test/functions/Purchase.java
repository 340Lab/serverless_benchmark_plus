package test.functions;

import java.math.BigDecimal;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.mapper.StockMngMapper;
import manage.session.SqlSessionFactoryUtil;

public class Purchase {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();
        String productCode = args.get("productCode").getAsString();
        int count = args.get("count").getAsInt();

        JsonObject result = new JsonObject();

        SqlSessionFactory stockDbSessionFactory = SqlSessionFactoryUtil.getStockDbSessionFactory();
        SqlSessionFactory balanceDbSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();

        // 使用 try-with-resources 来管理 stock_db 和 balance_db 的 SqlSession
        try (SqlSession stockSession = stockDbSessionFactory.openSession();
             SqlSession balanceSession = balanceDbSessionFactory.openSession()) {

            StockMngMapper stockMngMapper = stockSession.getMapper(StockMngMapper.class);
            BalanceMngMapper balanceMngMapper = balanceSession.getMapper(BalanceMngMapper.class);

            // 查询商品价格
            BigDecimal productPrice = stockMngMapper.queryProductPrice(productCode, userName);
            if (productPrice == null) {
                throw new RuntimeException("Product code does not exist");
            }
            if (count <= 0) {
                throw new RuntimeException("Purchase count should not be negative");
            }

            // 创建订单、扣减库存、扣减余额
            stockMngMapper.createOrder(userName, productCode, count);
            stockMngMapper.minusStockCount(userName, productCode, count);
            balanceMngMapper.minusBalance(userName, productPrice.multiply(new BigDecimal(count)));

            // 提交事务
            stockSession.commit();
            balanceSession.commit();

            result.addProperty("success", true);
            result.addProperty("message", "Purchase successful");

        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Purchase failed: " + e.getMessage());
        }

        return result;
    }

}
