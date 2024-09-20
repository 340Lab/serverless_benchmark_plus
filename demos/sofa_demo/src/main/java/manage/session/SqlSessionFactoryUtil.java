package manage.session;

import java.io.InputStream;

import org.apache.ibatis.session.SqlSessionFactory;
import org.apache.ibatis.session.SqlSessionFactoryBuilder;

public class SqlSessionFactoryUtil {
    private static SqlSessionFactory stockDbSessionFactory;
    private static SqlSessionFactory balanceDbSessionFactory;

    static {
        try {
            String resource = "mybatis-config.xml";
            
            // 重新打开 InputStream 用于 stock_db_env
            try (InputStream stockInputStream = SqlSessionFactoryUtil.class.getClassLoader().getResourceAsStream(resource)) {
                if (stockInputStream == null) {
                    throw new RuntimeException("Failed to load MyBatis configuration file for stock_db_env.");
                }
                stockDbSessionFactory = new SqlSessionFactoryBuilder().build(stockInputStream, "stock_db_env");
            }
            
            // 重新打开 InputStream 用于 balance_db_env
            try (InputStream balanceInputStream = SqlSessionFactoryUtil.class.getClassLoader().getResourceAsStream(resource)) {
                if (balanceInputStream == null) {
                    throw new RuntimeException("Failed to load MyBatis configuration file for balance_db_env.");
                }
                balanceDbSessionFactory = new SqlSessionFactoryBuilder().build(balanceInputStream, "balance_db_env");
            }
            
        } catch (Exception e) {
            throw new RuntimeException("Failed to initialize SqlSessionFactory", e);
        }
    }

    // 获取 stock_db 的 SqlSessionFactory
    public static SqlSessionFactory getStockDbSessionFactory() {
        return stockDbSessionFactory;
    }

    // 获取 balance_db 的 SqlSessionFactory
    public static SqlSessionFactory getBalanceDbSessionFactory() {
        return balanceDbSessionFactory;
    }

}
