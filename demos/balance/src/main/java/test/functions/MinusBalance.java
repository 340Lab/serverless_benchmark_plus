package test.functions;

import java.math.BigDecimal;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.session.SqlSessionFactoryUtil;

public class MinusBalance {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();
        BigDecimal amount = args.get("amount").getAsBigDecimal();
        JsonObject result = new JsonObject();

        // 使用 try-with-resources 来管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);

            // 执行减余额操作
            balanceMngMapper.minusBalance(userName, amount);
            session.commit();  // 提交事务

            result.addProperty("success", true);
            result.addProperty("message", "Balance deducted successfully");
        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Operation failed: " + e.getMessage());
        }

        return result;
    }

}
