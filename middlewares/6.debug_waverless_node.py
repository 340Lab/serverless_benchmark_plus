import os,sys
CUR_FDIR = os.path.dirname(os.path.abspath(__file__)); cur_scan=CUR_FDIR; scan=[["pylib.py" in os.listdir(cur_scan),cur_scan,exec('global cur_scan;cur_scan=os.path.join(cur_scan, "..")')] for _ in range(10)]; found_pylib=[x[0] for x in scan]; pylib_dir_idx=found_pylib.index(True); assert pylib_dir_idx>=0, "pylib.py not found"; print(scan[pylib_dir_idx][1]); ROOT_DIR=os.path.abspath(os.path.join(CUR_FDIR, scan[pylib_dir_idx][1])); sys.path.append(ROOT_DIR)
import pylib

if len(sys.argv)!=2:
    print("Usage: python3 6.debug_waverless_node.py <node_id>")
    print("  // node_id is the number of lab machine")
    print("  // for example, node_id of lab4 is 4")
    exit(1)

looking_for=sys.argv[1]
find=None
cluster_conf=pylib.load_cluster_config("cluster_config.yml")
for n in cluster_conf.nodes:
    if n.node_name==f"lab{looking_for}":
        find=n
        break

if find is None:
    print("Please specify the correct node_id")
    print("Current nodes:")
    cluster_conf.print()
    exit(1)
    
print(f"Starting waverless on node {find.node_name} {find.node_ip}")

pylib.os_system_sure(f"ssh root@{find.node_ip} 'systemctl stop waverless && python3 /waverless_deploy/waverless_backend/run_node.py {looking_for}'")