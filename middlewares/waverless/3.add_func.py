# ### chdir
import os
import importlib.util
import inspect
import yaml
import requests
CUR_FPATH = os.path.abspath(__file__)
CUR_FDIR = os.path.dirname(CUR_FPATH)
# chdir to the directory of this script
os.chdir(CUR_FDIR)
class FunctionContainer:
    pass
def load_functions_into_object(file_path, obj):
    # 从指定文件路径导入模块
    spec = importlib.util.spec_from_file_location("module.name", file_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    # 获取模块中定义的所有函数
    functions = {name: obj for name, obj in inspect.getmembers(module) if inspect.isfunction(obj)}
    # 将函数添加到对象中
    for func_name, func in functions.items():
        setattr(obj, func_name, func)
# 使用示例
file_path = '../../pylib.py'
pylib = FunctionContainer()
load_functions_into_object(file_path, pylib)
#################################################################################################



def run_cmd_with_res(cmd):
    print(f"执行命令：{cmd}")
    result = os.popen(cmd)
    # print(f"执行结果：{result.read()}")
    return result.read()


import sys
if len(sys.argv) !=3:
    print("Usage: python 3.add_func.py <demo_app> <rename_sub>")
    exit(1)
demo_app=sys.argv[1]
rename_sub=sys.argv[2]

# targetname=sys.argv[2]
srcyaml=pylib.read_yaml("../cluster_config.yml")
first_worker_ip=""
for n in srcyaml:
    if "is_master" not in srcyaml[n]:
        first_worker_ip=srcyaml[n]["ip"]
        break


os.chdir(f"../../demos")
# pylib.os_system_sure(f"python3 scripts/1.gen_waverless_app.py {demo_app}")
def upload_app(appname,rename):
    appdir=f"scripts/waverless/{appname}/pack"
    # 复制 JAR 文件到应用包
    os.system(f"cp ../demos/{appname}/target/{appname}-1.0-SNAPSHOT-jar-with-dependencies.jar {appdir}/app.jar")

    os.chdir(appdir)
    
    entries=os.listdir(f"./")
    entries_concat=" ".join(entries)
    print(f"{appdir} contains {entries_concat}")

    os.system(f"zip -r {rename}.zip {entries_concat}")
    os.system(f"mv {rename}.zip {CUR_FDIR}")
    os.chdir(CUR_FDIR)
    
    filepath=f"{rename}.zip"
    files=[]
    f= open(filepath, 'rb')
    files.append((rename, (filepath.split('/')[-1], f, 'application/zip')))
    
    try:
        response = requests.post(f'http://{first_worker_ip}:2501/appmgmt/upload_app', files=files)
        print(response.status_code, response.text)
    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")

upload_app(demo_app,demo_app+rename_sub)