import os,sys
CUR_FDIR = os.path.dirname(os.path.abspath(__file__)); cur_scan=CUR_FDIR; scan=[["pylib.py" in os.listdir(cur_scan),cur_scan,exec('global cur_scan;cur_scan=os.path.join(cur_scan, "..")')] for _ in range(10)]; found_pylib=[x[0] for x in scan]; pylib_dir_idx=found_pylib.index(True); assert pylib_dir_idx>=0, "pylib.py not found"; print(scan[pylib_dir_idx][1]); ROOT_DIR=os.path.abspath(os.path.join(CUR_FDIR, scan[pylib_dir_idx][1])); sys.path.append(ROOT_DIR)
import pylib

os.chdir(CUR_FDIR)

pylib.os_system(f"rm -rf waverless/bin_waverless2/prepare_cache/waverless_amd64")
pylib.os_system(f"rm -rf waverless/bin_waverless2/prepare_cache/waverless_entry_amd64")
pylib.os_system_sure(f"telego cmd --cmd deploy/bin_waverless2/prepare")
pylib.os_system_sure(f"telego cmd --cmd deploy/bin_waverless2/upload")


