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
    functions = {name: obj for name, obj in inspect.getmembers(module) if inspect.isfunction(obj)}
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


# 打开并读取YAML文件
with open('proxy.yaml', 'r') as file:
    # 加载YAML数据
    data = yaml.safe_load(file)
HOST=data['host']
PORT=data['port']
NO_PROXY='localhost,127.0.0.1,::1,192.168.0.0/16'
new_exports = [
    f'export http_proxy=http://{HOST}:{PORT}',
    f'export https_proxy=http://{HOST}:{PORT}',
    f"export NO_PROXY='{NO_PROXY}'"
]

def config_remote_proxy(ip):
    def update_bashrc(bashrc_lines:list):
        # 检查和更新 .bashrc 文件
        new_http=False
        new_https=False
        new_no=False
        updated_bashrc_lines = []
        for line in bashrc_lines:
            if line.startswith('export http_proxy='):
                new_http=True
                updated_bashrc_lines.append(new_exports[0] + '\n')
            elif line.startswith('export https_proxy='):
                updated_bashrc_lines.append(new_exports[1] + '\n')
                new_https=True
            elif line.startswith('export NO_PROXY='):
                updated_bashrc_lines.append(new_exports[2] + '\n')
                new_no = True
            else:
                updated_bashrc_lines.append(line)

        # # 如果没有找到对应的 export 语句，则追加到文件末尾
        # if new_exports[0] not in updated_bashrc_lines:
        if not new_http:
            updated_bashrc_lines.append(new_exports[0] + '\n')
        # if new_exports[1] not in updated_bashrc_lines:
        if not new_https:
            updated_bashrc_lines.append(new_exports[1] + '\n')
        if not new_no:
            updated_bashrc_lines.append(new_exports[2] + '\n')

        return updated_bashrc_lines
    # read remote bashrc
    pylib.cmd_get_remote_file(ip,"root","/root/.bashrc").run()
    with open(".bashrc","r") as f:
        bashrc_lines=f.readlines()
    newlines=update_bashrc(bashrc_lines)
    with open(".bashrc","w") as f:
        f.writelines(newlines)
    # write remote bashrc
    pylib.cmd_send_remote_file(ip,"root","/root/.bashrc",".bashrc")

for n in clusteryml:
    nconf=clusteryml[n]
    config_remote_proxy(nconf["ip"])
    print(">>> configured proxy",n,nconf["ip"])






