import os,sys
CUR_FDIR = os.path.dirname(os.path.abspath(__file__)); cur_scan=CUR_FDIR; scan=[["pylib.py" in os.listdir(cur_scan),cur_scan,exec('global cur_scan;cur_scan=os.path.join(cur_scan, "..")')] for _ in range(10)]; found_pylib=[x[0] for x in scan]; pylib_dir_idx=found_pylib.index(True); assert pylib_dir_idx>=0, "pylib.py not found"; print(scan[pylib_dir_idx][1]); ROOT_DIR=os.path.abspath(os.path.join(CUR_FDIR, scan[pylib_dir_idx][1])); sys.path.append(ROOT_DIR)
import pylib

os.chdir("openwhisk")

pylib.os_system_sure(f"python3 1.prepare.py")
pylib.os_system_sure(f"python3 3.deploy_ow.py")

