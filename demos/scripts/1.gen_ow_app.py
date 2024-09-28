### chdir
import os
import sys
import yaml
import zipfile
import re
import xml.etree.ElementTree as ET

CUR_FPATH = os.path.abspath(__file__)
CUR_FDIR = os.path.dirname(CUR_FPATH)
# chdir to the directory of this script
os.chdir(CUR_FDIR)

# os.system('ansible-playbook -vv 2.ans_install_build.yml -i ../local_ansible_conf.ini')
### utils
def os_system_sure(command):
    print(f"Run：{command}\n")
    result = os.system(command)
    if result != 0:
        print(f"\nFail：{command}\n\n")
        exit(1)
    print(f"\nSucc：{command}\n\n")


# result.returncode
# result.stdout
def run_cmd_return(cmd):
    print(f"Run：{cmd}\n")
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    print(f"\nStdout：{result.stdout}\n\n")
    return result


def print_title(title):
    print(f"\n\n>>> {title}")


def find_folders_recursively(directory,target:str):
    folders = []
    
    # 遍历目录及其子目录
    for root, dirs, files in os.walk(directory):
        if root.find("target")>=0:
            continue
        for dir in dirs:
            if dir == target:
                folders.append(os.path.join(root, dir))
            # folders.append(os.path.join(root, dir))
    return folders

def add_cant_change_comment(dir,comment):
    for root, dirs, files in os.walk(dir):
        for file in files:
            if file.endswith(".java"):
                file_path = os.path.join(root, file)
                with open(file_path, 'r') as f:
                    content = f.read()
                content=(comment+"\n").join(content.split("\n"))
                with open(file_path, 'w') as f:
                    f.write(content)

def bigcamel_to_snake(name):
    # 将大驼峰命名转换为蛇形命名
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    snake_case = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
    return snake_case

def add_dependency_to_pom(file_path, group_id, artifact_id, version=None):
    # 解析pom.xml文件
    ET.register_namespace('', 'http://maven.apache.org/POM/4.0.0')
    tree = ET.parse(file_path)
    root = tree.getroot()

    # 找到<dependencies>标签
    dependencies = root.find('{http://maven.apache.org/POM/4.0.0}dependencies')
    if dependencies is None:
        # 如果<dependencies>标签不存在，则创建一个新的
        dependencies = ET.SubElement(root, 'dependencies')

    # 创建新的<dependency>标签
    dependency = ET.SubElement(dependencies, 'dependency')
    
    group_id_element = ET.SubElement(dependency, 'groupId')
    group_id_element.text = group_id
    
    artifact_id_element = ET.SubElement(dependency, 'artifactId')
    artifact_id_element.text = artifact_id
    
    if version:
        version_element = ET.SubElement(dependency, 'version')
        version_element.text = version
    else:
        # 注释掉<version>标签
        version_element = ET.Comment(' <version>0.0.1-SNAPSHOT</version> ')
        dependency.append(version_element)

    # 将修改后的内容写回pom.xml文件
    tree.write(file_path, encoding='utf-8', xml_declaration=True)
    

def snake_to_big_camel(snake_str):
    # 使用split将蛇形命名拆分成单词列表
    components = snake_str.split('_')
    # 将每个单词的首字母大写并连接成一个字符串
    return ''.join(x.title() for x in components)

def check_and_compile_sofa_demo(prj_path):
    pom_path = os.path.join(prj_path, "pom.xml")
    
    # 检查 pom.xml 中是否包含 <serverless-depend>../sofa_demo</serverless-depend>
    if os.path.exists(pom_path):
        with open(pom_path, 'r') as pom_file:
            pom_content = pom_file.read()
            if "<serverless-depend>../sofa_demo</serverless-depend>" in pom_content:
                print("\nDetected serverless-depend on sofa_demo, compiling sofa_demo...")
                os_system_sure("cd ../sofa_demo && mvn clean package")
            else:
                print("\nNo serverless-depend on sofa_demo found.")
    else:
        print("\nNo pom.xml found.")
#################################################################################################


if len(sys.argv)!=2:
    print("Usage: python3 1.gen_waverless_prj.py <project_name>")
    exit(1)

prj = sys.argv[1]


# check jdk8
if not os.path.exists("/usr/lib/jvm/java-8-openjdk-amd64/"):
    print("!!! openjdk 8 not found")
    exit(1)

# JAVA_HOME
os.environ["JAVA_HOME"] = "/usr/lib/jvm/java-8-openjdk-amd64/"

temp_prj_dir="ow/"+prj
os_system_sure(f"rm -rf ow")
os_system_sure(f"mkdir -p ow/{prj}/tmp")

## copy prj
os_system_sure(f"cp -r ../{prj}/* ./ow/{prj}/tmp/")

# Check for sofa_demo dependency and compile if found
check_and_compile_sofa_demo(f"./ow/{prj}/tmp")

## construct app config file for waverless
#  find functions dir in prj
functions_dir = find_folders_recursively(f"./ow/{prj}/tmp","functions")[0]
functions=[]
#  for each XXX.java, construct app.yml
for fnfile in os.listdir(functions_dir):
    if fnfile.endswith(".java"):
        fnname = bigcamel_to_snake(fnfile.split(".")[0])
        # app_yml[prj][fnname]={
        #     "rpc": None
        # }
        functions.append(fnname)

## construct apps
for fn in functions:
    os_system_sure(f"cp -r ow/{prj}/tmp ow/{prj}/{fn}")
    target_functions_dir=find_folders_recursively(f"./ow/{prj}/{fn}","functions")[0]
    # remove others except fn
    for fnfile in os.listdir(target_functions_dir):
        if not fnfile.startswith(snake_to_big_camel(fn)):
            os_system_sure(f"rm -rf {target_functions_dir}/{fnfile}")
        
    ## add entry point
    #  Application.java
    functions_parent_dir = os.path.dirname(target_functions_dir)
    package_name=functions_parent_dir.split("main/java/")[-1].replace("/",".")
    application_java=f"""
package {package_name};



import com.google.gson.JsonObject;
import com.google.gson.JsonParser;
import {package_name}.functions.{snake_to_big_camel(fn)};


class NullOutputStream extends java.io.OutputStream {{
    @Override
    public void write(int b) {{
        // 不做任何处理
    }}
}}

public class Application {{
    
    public static JsonObject main(JsonObject args) {{  
        long fnStartTime = System.currentTimeMillis();

        JsonObject res= new {snake_to_big_camel(fn)}().call(args);
        
        long fnEndTime=System.currentTimeMillis();
        res.addProperty("fn_start_time",fnStartTime);
        res.addProperty("fn_end_time",fnEndTime);
        return res;
    }}

    // for simple call
    public static void main(String[] args){{
        java.io.PrintStream out=System.out;

        // 禁用System.out
        System.setOut(new java.io.PrintStream(new NullOutputStream()));
        // 禁用System.err
        System.setErr(new java.io.PrintStream(new NullOutputStream()));
        
        JsonParser parser = new JsonParser();
        // 将JSON字符串解析为JsonObject
        JsonObject req = parser.parse(args[0]).getAsJsonObject();
        JsonObject resp=new {snake_to_big_camel(fn)}().call(req);
        
        out.println(resp.toString());
    }}
}}

"""
    with open(f"{functions_parent_dir}/Application.java","w") as f:
        f.write(application_java)

    os.chdir(f"./ow/{prj}/{fn}")
    os_system_sure(f"mvn clean package")
    os.chdir("../../..")
os_system_sure("rm -rf ow/*/tmp")
add_cant_change_comment("ow","// ！！！请勿修改此文件，此文件由脚本生成")

