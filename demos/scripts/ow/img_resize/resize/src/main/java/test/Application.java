// ！！！请勿修改此文件，此文件由脚本生成
package test;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
import com.google.gson.JsonObject;// ！！！请勿修改此文件，此文件由脚本生成
import com.google.gson.JsonParser;// ！！！请勿修改此文件，此文件由脚本生成
import test.functions.Resize;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
class NullOutputStream extends java.io.OutputStream {// ！！！请勿修改此文件，此文件由脚本生成
    @Override// ！！！请勿修改此文件，此文件由脚本生成
    public void write(int b) {// ！！！请勿修改此文件，此文件由脚本生成
        // 不做任何处理// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
}// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
public class Application {// ！！！请勿修改此文件，此文件由脚本生成
    // ！！！请勿修改此文件，此文件由脚本生成
    public static JsonObject main(JsonObject args) {  // ！！！请勿修改此文件，此文件由脚本生成
        return new Resize().call(args);// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
    // for simple call// ！！！请勿修改此文件，此文件由脚本生成
    public static void main(String[] args){// ！！！请勿修改此文件，此文件由脚本生成
        java.io.PrintStream out=System.out;// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
        // 禁用System.out// ！！！请勿修改此文件，此文件由脚本生成
        System.setOut(new java.io.PrintStream(new NullOutputStream()));// ！！！请勿修改此文件，此文件由脚本生成
        // 禁用System.err// ！！！请勿修改此文件，此文件由脚本生成
        System.setErr(new java.io.PrintStream(new NullOutputStream()));// ！！！请勿修改此文件，此文件由脚本生成
        // ！！！请勿修改此文件，此文件由脚本生成
        JsonParser parser = new JsonParser();// ！！！请勿修改此文件，此文件由脚本生成
        // 将JSON字符串解析为JsonObject// ！！！请勿修改此文件，此文件由脚本生成
        JsonObject req = parser.parse(args[0]).getAsJsonObject();// ！！！请勿修改此文件，此文件由脚本生成
        JsonObject resp=new Resize().call(req);// ！！！请勿修改此文件，此文件由脚本生成
        // ！！！请勿修改此文件，此文件由脚本生成
        out.println(resp.toString());// ！！！请勿修改此文件，此文件由脚本生成
    }// ！！！请勿修改此文件，此文件由脚本生成
}// ！！！请勿修改此文件，此文件由脚本生成
// ！！！请勿修改此文件，此文件由脚本生成
