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
public class QueryBalance {// ！！！请勿修改此文件，此文件由脚本生成
    // ！！！请勿修改此文件，此文件由脚本生成
    public JsonObject call(JsonObject args) {// ！！！请勿修改此文件，此文件由脚本生成
        String userName = args.get("username").getAsString();// ！！！请勿修改此文件，此文件由脚本生成
        JsonObject result = new JsonObject();// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        // 使用 try-with-resources 来管理 SqlSession// ！！！请勿修改此文件，此文件由脚本生成
        SqlSessionFactory sqlSessionFactory = SqlSessionFactoryUtil.getBalanceDbSessionFactory();// ！！！请勿修改此文件，此文件由脚本生成
        try (SqlSession session = sqlSessionFactory.openSession()) {// ！！！请勿修改此文件，此文件由脚本生成
            BalanceMngMapper balanceMngMapper = session.getMapper(BalanceMngMapper.class);// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
            // 查询用户余额// ！！！请勿修改此文件，此文件由脚本生成
            Balance balance = balanceMngMapper.queryBalance(userName);// ！！！请勿修改此文件，此文件由脚本生成
            if (balance == null) {// ！！！请勿修改此文件，此文件由脚本生成
                throw new RuntimeException("user name does not exist");// ！！！请勿修改此文件，此文件由脚本生成
            }// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("balance", balance.getBalance().toString());// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("success", true);// ！！！请勿修改此文件，此文件由脚本生成
        } catch (Exception e) {// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("success", false);// ！！！请勿修改此文件，此文件由脚本生成
            result.addProperty("message", "Operation failed: " + e.getMessage());// ！！！请勿修改此文件，此文件由脚本生成
        }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        return result;// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
}// ！！！请勿修改此文件，此文件由脚本生成
