package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.model.Balance;
import manage.session.SqlSessionFactoryUtil;

public class QueryBalance {
    
    public JsonObject call(JsonObject args) {
        String userName = args.get("username").getAsString();
        JsonObject result = new JsonObject();

        // 使用 try-with-resources 来管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);

            // 查询用户余额
            Balance balance = balanceMngMapper.queryBalance(userName);
            if (balance == null) {
                throw new RuntimeException("user name does not exist");
            }
            result.addProperty("balance", balance.getBalance().toString());
            result.addProperty("success", true);
        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Operation failed: " + e.getMessage());
        }

        return result;
    }

}
