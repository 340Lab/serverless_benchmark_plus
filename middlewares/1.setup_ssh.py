# 配置节点间免密

### chdir
import os
CUR_FPATH = os.path.abspath(__file__)
CUR_FDIR = os.path.dirname(CUR_FPATH)
# chdir to the directory of this script
os.chdir(CUR_FDIR)


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


### pack waverless_ui & waverless_backend to pack.tar.gz
import os
import yaml
import argparse
import sys
import pexpect

PASSWORD="aaaaa"

def run_cmd(cmd):
    print("> "+cmd)
    # if cmd.startswith("ssh") or cmd.startswith("scp"):
    #     # 创建spawn对象
    #     child = pexpect.spawn(cmd, encoding='utf-8',logfile=sys.stdout)

    #     # 匹配密码提示，然后发送密码
    #     child.expect('password:')
    #     child.sendline(PASSWORD)

    #     # 在这里可以继续与SSH会话进行交互
    #     # 例如，可以发送其他命令

    #     # 等待命令执行完成
    #     try:
    #         child.expect(pexpect.EOF)
    #     except:
    #         pass
    #     child.close()
    #     # 打印输出
    #     # print(child.before)
    # else:
    os.system(cmd)


def read_yaml(f):
    # parse
    import ruamel.yaml
    yaml = ruamel.yaml.YAML(typ='rt')
    parsed_data = yaml.load(f)

    return parsed_data

def entry():
    # read cluster-nodes.yml
    with open('cluster_config.yml', 'r') as f:
        # run_cmd("scripts/install/install_ansible.sh")

        # # gen ssh key if not exist
        if not os.path.exists("/root/.ssh/id_rsa"):
            run_cmd("ssh-keygen -t rsa -b 2048")

        cluster_nodes = read_yaml(f)
        appeared_node={}
        for nid in cluster_nodes:
            ip=cluster_nodes[nid]
            # id=node["id"]

        
            # run_cmd("ssh root@{} 'apt install python3'".format(ip))
            run_cmd("ssh-copy-id root@{}".format(ip))
        
        
        
entry()