# ### chdir
import yaml
import os
import importlib.util
import inspect
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
    functions = {name: obj for name, obj in inspect.getmembers(module)}
    # 将函数添加到对象中
    for func_name, func in functions.items():
        setattr(obj, func_name, func)
# 使用示例
file_path = '../pylib.py'
pylib = FunctionContainer()
load_functions_into_object(file_path, pylib)
#################################################################################################

with open("cluster_config.yml","r") as f:
    clusteryml=yaml.safe_load(f)

for n in clusteryml:
    ip=clusteryml[n]["ip"]
    pylib.Cmd(["systemctl","start","waverless"]).remote("root",ip).run()

pylib.os_system_sure("python3 print_cluster_status.py")