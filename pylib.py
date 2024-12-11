# # ### chdir
# import os
# import importlib.util
# import inspect
# CUR_FPATH = os.path.abspath(__file__)
# CUR_FDIR = os.path.dirname(CUR_FPATH)
# # chdir to the directory of this script
# os.chdir(CUR_FDIR)
# class FunctionContainer:
#     pass
# def load_functions_into_object(file_path, obj):
#     # 从指定文件路径导入模块
#     spec = importlib.util.spec_from_file_location("module.name", file_path)
#     module = importlib.util.module_from_spec(spec)
#     spec.loader.exec_module(module)
#     # 获取模块中定义的所有函数
#     functions = {name: obj for name, obj in inspect.getmembers(module)}
#     # 将函数添加到对象中
#     for func_name, func in functions.items():
#         setattr(obj, func_name, func)
# # 使用示例
# file_path = '../pylib.py'
# pylib = FunctionContainer()
# load_functions_into_object(file_path, pylib)
#################################################################################################


import sys
import os
import yaml
import subprocess

### utils
def os_system_sure(command):
    print(f"执行命令：{command}")
    result = os.system(command)
    if result != 0:
        print(f"命令执行失败：{command}")
        exit(1)
    print(f"命令执行成功：{command}\n\n")

def os_system(command):
    print(f"执行命令：{command}")
    result = os.system(command)
    print("\n\n")


def read_yaml(file):
    with open(file, 'r') as f:
        return yaml.safe_load(f)

def key_step(step):
    # if step is str
    if type(step)==str:
        print("\n")
        print("*"*40)
        print(">>>",sys.argv[0],step)
        print("\n")
        return
    # if step is fn
    if callable(step):
        print("\n")
        print("*"*40)
        print(">>>",sys.argv[0],step.__name__)
        print("\n")
        return step()

class Cmd:
    cmd=[]
    def __init__(self,cmd):
        self.cmd=cmd
    def remote(self,usr,host):
        self.cmd.insert(0,f"ssh")
        self.cmd.insert(1,f"{usr}@{host}")
        return self
    def run(self):
        try:
            result = subprocess.run(self.cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
            return True  # 如果服务处于活动状态，则返回True
        except subprocess.CalledProcessError:
            return False  # 如果服务未处于活动状态，则返回False


def cmd_check_service_status(service_name):
    return Cmd(['systemctl', 'is-active', '--quiet', service_name])

def cmd_get_remote_file(ip,user,file):
    return Cmd(['scp',f"{user}@{ip}:{file}","."])

def cmd_send_remote_file(ip,user,file,remotedir):
    return Cmd(['scp',file,f"{user}@{ip}:{remotedir}"])


class ClusterConfigNode:
    def __init__(self,node_name,node_ip,is_master) -> None:
        self.node_name=node_name
        self.node_ip=node_ip
        self.is_master=is_master
class ClusterConfig:
    nodes=[]
    
    def print(self):
        for n in self.nodes:
            print(f"Node: {n.node_name}, IP: {n.node_ip}, is_master: {n.is_master}")

def load_cluster_config(cluster_conf_path)->ClusterConfig:
    conf=ClusterConfig()
    with open(cluster_conf_path) as f:
        conf_raw=yaml.safe_load(f)
        for node_name in conf_raw:
            is_master=False
            if "is_master" in conf_raw[node_name]:
                is_master=True
            conf.nodes.append(ClusterConfigNode(
                node_name,
                conf_raw[node_name]["ip"],
                is_master))
    return conf