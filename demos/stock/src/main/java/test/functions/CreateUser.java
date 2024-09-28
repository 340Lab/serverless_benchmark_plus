package test.functions;

import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;

import com.google.gson.JsonObject;

import manage.mapper.BalanceMngMapper;
import manage.model.Balance;
import manage.session.SqlSessionFactoryUtil;


public class CreateUser {

    public JsonObject call(JsonObject args) {
        String userName = args.get("username").getAsString();
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();
        if (sqlSessionFactory == null) {
            throw new IllegalStateException("SqlSessionFactory not initialized.");
        }

        // 使用 try-with-resources 语句自动关闭 session
        try (SqlSession session = sqlSessionFactory.openSession()) {
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);
            Balance balance = balanceMngMapper.userExists(userName);
            if (balance == null) {
                balanceMngMapper.createUser(userName);
                session.commit();  // 提交事务
            } else {
                // 用户已存在，可能选择回滚
                session.rollback();  // 如果不需要提交则回滚
            }
        } catch (Exception e) {
        }

        return null;
    }

}
