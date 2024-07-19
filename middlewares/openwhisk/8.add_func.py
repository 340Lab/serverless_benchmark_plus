import glob
import os, yaml
CUR_FPATH = os.path.abspath(__file__)
CUR_FDIR = os.path.dirname(CUR_FPATH)
# chdir to the directory of this script
os.chdir(CUR_FDIR)


import os
def os_system_sure(command):
    print(f"执行命令：{command}")
    result = os.system(command)
    if result != 0:
        print(f"命令执行失败：{command}")
        exit(1)
    print(f"命令执行成功：{command}\n\n")


def run_cmd_with_res(cmd):
    print(f"执行命令：{cmd}")
    result = os.popen(cmd)
    # print(f"执行结果：{result.read()}")
    return result.read()


import sys
if len(sys.argv) !=2:
    print("Usage: python 8.add_func.py <demo_app>")
    exit(1)

demo_app=sys.argv[1]
# targetname=sys.argv[2]

os.chdir(f"../../demos")
os_system_sure(f"python3 scripts/1.gen_ow_app.py {demo_app}")

ow_app_dir=f"scripts/ow/{demo_app}"
# list funcs in ow_app_dir
file_pattern = os.path.join(ow_app_dir, '*with-dependencies*')
for file_path in glob.glob(file_pattern):
    fn = os.path.basename(file_path)  # 获取文件名
    os_system_sure(f"wsk -i action create {demo_app}_{fn} {file_path}/target/hello-1.0-SNAPSHOT-jar-with-dependencies.jar --main test.Application")

# list funcs
os_system_sure("wsk list")
