package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.model.Balance;
import manage.session.SqlSessionFactoryUtil;

public class QueryBalance {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();
        JsonObject result = new JsonObject();

        // 使用 try-with-resources 管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);

            // 查询用户余额
            Balance balance = balanceMngMapper.queryBalance(userName);
            if (balance == null) {
                throw new RuntimeException("User name does not exist");
            }

            // 构建返回的 JSON 结果
            result.addProperty("userName", userName);
            result.addProperty("balance", balance.getBalance().toString());

            session.commit();  // 提交事务
        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Query failed: " + e.getMessage());
        }

        return result;
    }

}
