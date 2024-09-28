package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.model.Balance;
import manage.session.SqlSessionFactoryUtil;

public class CreateUser {

    public JsonObject call(JsonObject args) {
        String userName = args.get("userName").getAsString();
        JsonObject result = new JsonObject();

        // 使用 try-with-resources 来管理 SqlSession
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();
        try (SqlSession session = sqlSessionFactory.openSession()) {
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);

            // 检查用户是否存在
            Balance balance = balanceMngMapper.userExists(userName);
            if (balance == null) {
                // 创建新用户
                balanceMngMapper.createUser(userName);
                session.commit();  // 提交事务
                result.addProperty("success", true);
                result.addProperty("message", "User created successfully");
            } else {
                result.addProperty("success", false);
                result.addProperty("message", "User already exists");
            }
        } catch (Exception e) {
            result.addProperty("success", false);
            result.addProperty("message", "Operation failed: " + e.getMessage());
        }

        return result;
    }

}
