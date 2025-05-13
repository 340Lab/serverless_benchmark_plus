
### chdir
import os
import yaml
CUR_FPATH = os.path.abspath(__file__)
CUR_FDIR = os.path.dirname(CUR_FPATH)
# chdir to the directory of this script
os.chdir(CUR_FDIR)


### utils
def os_system_sure(command):
    print(f"Run：{command}\n")
    result = os.system(command)
    if result != 0:
        print(f"\n  >>> Fail：{command}\n\n")
        exit(1)
    print(f"\n  >>> Succ：{command}\n\n")


# result.returncode
# result.stdout
def run_cmd_return(cmd):
    print(f"Run：{cmd}\n\n")
    result = subprocess.run(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    print(f"Stdout：{result.stdout}")
    
    return result


with open("config.yaml") as f:
    conf=yaml.safe_load(f)
waverless_rel_path=conf["waverless_rel_path"]
proto_src_dir=os.path.join(waverless_rel_path,"src/main/src/general/app/app_shared/")
proto_src=os.path.join(proto_src_dir,'process_rpc_proto.proto')


print("Proto Src Dir:")
os_system_sure(f"ls {proto_src_dir}")
print("\n\n")

print("Proto target Dir:")
os_system_sure("ls ../core/src/main/java/io/serverless_lib/")
print("\n\n"  )

os_system_sure(f"protoc --proto_path={proto_src_dir} \
--java_out=../core/src/main/java/io/serverless_lib {proto_src}")
