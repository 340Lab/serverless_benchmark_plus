package test.functions;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import org.apache.ibatis.session.SqlSession;// ！！！请勿修改此文件，此文件由脚本生成
import org.apache.ibatis.session.SqlSessionFactory;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import com.google.gson.JsonObject;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import manage.mapper.BalanceMngMapper;// ！！！请勿修改此文件，此文件由脚本生成
import manage.model.Balance;// ！！！请勿修改此文件，此文件由脚本生成
import manage.session.SqlSessionFactoryUtil;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
public class CreateUser {// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
    public JsonObject call(JsonObject args) {// ！！！请勿修改此文件，此文件由脚本生成
        String userName = args.get("userName").getAsString();// ！！！请勿修改此文件，此文件由脚本生成
        JsonObject result = new JsonObject();// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        // 使用 try-with-resources 来管理 SqlSession// ！！！请勿修改此文件，此文件由脚本生成
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();// ！！！请勿修改此文件，此文件由脚本生成
        try (SqlSession session = sqlSessionFactory.openSession()) {// ！！！请勿修改此文件，此文件由脚本生成
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // 检查用户是否存在// ！！！请勿修改此文件，此文件由脚本生成
            Balance balance = balanceMngMapper.userExists(userName);// ！！！请勿修改此文件，此文件由脚本生成
            if (balance == null) {// ！！！请勿修改此文件，此文件由脚本生成
                // 创建新用户// ！！！请勿修改此文件，此文件由脚本生成
                balanceMngMapper.createUser(userName);// ！！！请勿修改此文件，此文件由脚本生成
                session.commit();  // 提交事务// ！！！请勿修改此文件，此文件由脚本生成
                result.addProperty("success", true);// ！！！请勿修改此文件，此文件由脚本生成
                result.addProperty("message", "User created successfully");// ！！！请勿修改此文件，此文件由脚本生成
            } else {// ！！！请勿修改此文件，此文件由脚本生成
                result.addProperty("success", false);// ！！！请勿修改此文件，此文件由脚本生成
                result.addProperty("message", "User already exists");// ！！！请勿修改此文件，此文件由脚本生成
            }// ！！！请勿修改此文件，此文件由脚本生成
        } catch (Exception e) {// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("success", false);// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("message", "Operation failed: " + e.getMessage());// ！！！请勿修改此文件，此文件由脚本生成
        }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        return result;// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
}// ！！！请勿修改此文件，此文件由脚本生成
