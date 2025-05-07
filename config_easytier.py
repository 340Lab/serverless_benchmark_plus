#!/usr/bin/env python3

###########################################
# EasyTier 配置参数
###########################################
# 网络名称，确保唯一性
NETWORK_NAME = "cuit_cloud"
# 网络密钥，用于加密通信
NETWORK_SECRET = "827385543"
# 是否使用官方公共节点
USE_PUBLIC_NODE = True
# 官方公共节点地址
PUBLIC_NODE = "tcp://115.29.224.208:11010"
# 本地监听地址（多个进程需要避免端口冲突）
LISTEN_PORT = "11010"
# ipv4
IPV4="10.126.126.78"
# 是否启用子网代理
ENABLE_SUBNET_PROXY = False
# 要代理的子网（如果启用子网代理）
SUBNET_TO_PROXY = "192.168.1.0/24"
###########################################



import os
import sys

# 切换到脚本所在目录
script_dir = os.path.dirname(os.path.abspath(__file__))
os.chdir(script_dir)

###########################################
# 路径配置
###########################################
# 下载目录
DOWNLOAD_DIR = '/teledeploy_secret/bin_easytier'
# 安装目录
INSTALL_DIR = os.path.join(DOWNLOAD_DIR, 'install')
# 可执行文件名
EXEC_NAMES = ['easytier-cli', 'easytier-core', 'easytier-web']
MAIN_EXEC = 'easytier-core'
# 可执行文件的完整路径
EXEC_PATHS = {name: os.path.join(INSTALL_DIR, 'easytier', name) for name in EXEC_NAMES}
# 软链接路径
LINK_PATHS = {name: os.path.join('/usr/bin', name) for name in EXEC_NAMES}
# 主软链接路径
MAIN_LINK = os.path.join('/usr/bin', 'easytier')


import subprocess
import platform
import shutil
import time
import urllib.request
import zipfile
import os
from pathlib import Path

def run_with_root(cmd):
    """使用root权限运行命令，如果不是root用户则添加sudo"""
    if isinstance(cmd, list):
        cmd = ' '.join(cmd)
    if os.geteuid() != 0:
        cmd = 'sudo ' + cmd
    return os.system(cmd)

def check_system():
    """检查系统环境"""
    if platform.system() != "Linux":
        print("目前只支持 Linux 系统")
        sys.exit(1)
    
    # 检查必要的命令是否存在
    required_commands = ['curl', 'tar', 'systemctl']
    for cmd in required_commands:
        if shutil.which(cmd) is None:
            print(f"缺少必要的命令: {cmd}")
            sys.exit(1)

def verify_zip_file(file_path):
    """验证zip文件是否有效"""
    try:
        # 检查文件是否存在且非空
        if not os.path.exists(file_path) or os.path.getsize(file_path) == 0:
            return False
            
        # 尝试打开zip文件
        with zipfile.ZipFile(file_path, 'r') as zip_ref:
            # 检查zip文件完整性
            if zip_ref.testzip() is not None:
                return False
            # 检查是否包含 easytier 文件
            file_list = zip_ref.namelist()
            return any('easytier' in f.lower() for f in file_list)
            
    except zipfile.BadZipFile:
        return False
    except Exception:
        return False

def show_progress(count, block_size, total_size):
    """显示下载进度"""
    if total_size > 0:
        percent = min(100, count * block_size * 100 / total_size)
        # 计算进度条长度，总长50字符
        bar_len = 50
        filled_len = int(bar_len * count * block_size / total_size)
        bar = '=' * filled_len + '-' * (bar_len - filled_len)
        # 计算下载速度和已下载大小
        downloaded = count * block_size
        downloaded_mb = downloaded / (1024 * 1024)
        total_mb = total_size / (1024 * 1024)
        # 输出进度信息
        print(f'\r[下载进度] [{bar}] {percent:.1f}% ({downloaded_mb:.1f}MB/{total_mb:.1f}MB)', end='')
        if count * block_size >= total_size:
            print()  # 下载完成后换行

def download_easytier():
    """下载最新版本的 EasyTier到指定目录"""
    easytier_zip = os.path.join(DOWNLOAD_DIR, 'easytier.zip')
    
    try:
        need_download = True
        
        # 检查已存在的文件
        if os.path.exists(easytier_zip):
            print("检查已下载的文件...")
            if verify_zip_file(easytier_zip):
                print("使用已下载的 EasyTier 安装包")
                need_download = False
            else:
                print("已下载的文件无效，将重新下载")
                os.remove(easytier_zip)
        
        # 下载文件
        if need_download:
            print("正在下载 EasyTier...")
            os.makedirs(DOWNLOAD_DIR, exist_ok=True)
            download_url = "https://github.com/EasyTier/EasyTier/releases/download/v2.2.0/easytier-linux-x86_64-v2.2.0.zip"
            
            # 使用 urllib 下载文件，显示进度
            try:
                urllib.request.urlretrieve(download_url, easytier_zip, show_progress)
            except urllib.error.URLError as e:
                raise Exception(f"下载失败: {str(e)}")
            
            # 验证新下载的文件
            if not verify_zip_file(easytier_zip):
                raise Exception("下载的文件无效，请检查网络连接")
        
        # 解压文件
        print("正在解压文件...")
        # 解压之前要rm INSTALL_DIR
        os.system("rm -rf " + INSTALL_DIR)
        os.makedirs(INSTALL_DIR, exist_ok=True)
        
        with zipfile.ZipFile(easytier_zip, 'r') as zip_ref:
            # 查看解压后的文件结构
            file_list = zip_ref.namelist()
            print("文件列表:", file_list)
            
            # 解压文件
            zip_ref.extractall(INSTALL_DIR)
        
        # 检查解压后的目录结构
        print("安装目录内容:")
        os.system(f"ls -la {INSTALL_DIR}")
        
        # 如果目录不存在，创建它
        extracted_dir = os.path.join(INSTALL_DIR, 'easytier')
        if not os.path.exists(extracted_dir):
            os.makedirs(extracted_dir)
            # 移动所有可执行文件到这个目录
            for exec_name in EXEC_NAMES:
                src = os.path.join(INSTALL_DIR, f'easytier-linux-x86_64/{exec_name}')
                dst = os.path.join(extracted_dir, exec_name)
                if os.path.exists(src):
                    os.rename(src, dst)
                else:
                    raise Exception(f"未找到可执行文件: {exec_name}")
        
        # 检查所有可执行文件是否存在
        for exec_name in EXEC_NAMES:
            if not os.path.exists(EXEC_PATHS[exec_name]):
                raise Exception(f"解压失败，未找到 {exec_name} 可执行文件")
            # 设置可执行权限
            os.chmod(EXEC_PATHS[exec_name], 0o755)

        # 创建软链接
        for exec_name in EXEC_NAMES:
            if os.path.exists(LINK_PATHS[exec_name]):
                run_with_root(f'rm -rf {LINK_PATHS[exec_name]}')
            run_with_root(f'ln -s {EXEC_PATHS[exec_name]} {LINK_PATHS[exec_name]}')
        
        # 创建主软链接 (easytier -> easytier-core)
        if os.path.exists(MAIN_LINK):
            run_with_root(f'rm -rf {MAIN_LINK}')
        run_with_root(f'ln -s {EXEC_PATHS[MAIN_EXEC]} {MAIN_LINK}')
        
            
    except Exception as e:
        if os.path.exists(easytier_zip):
            os.remove(easytier_zip)
        raise e

def create_systemd_service():
    """创建并启用 systemd 服务"""
    print("\n开始配置 EasyTier 服务...")
    
    # 准备配置命令
    cmd = [
        LINK_PATHS[MAIN_EXEC],
        '--network-name', NETWORK_NAME,
        '--network-secret', NETWORK_SECRET,
        '-i',  # 使用DHCP自动分配IP
        IPV4
    ]
    
    if USE_PUBLIC_NODE:
        # 使用公共节点
        cmd.extend(['-p', PUBLIC_NODE])

    # 如果不使用公共节点，则监听本地地址
    cmd.extend(['--listeners', LISTEN_PORT])
    
    # 生成systemd服务文件
    service_content = f"""[Unit]
Description=EasyTier Service
After=network.target

[Service]
Type=simple
ExecStart={' '.join(cmd)}
Restart=always
User=root

[Install]
WantedBy=multi-user.target
"""
    
    # 写入服务文件
    service_path = '/etc/systemd/system/easytier.service'
    with open('/tmp/easytier.service', 'w') as f:
        f.write(service_content)
    run_with_root(f'mv /tmp/easytier.service {service_path}')
    
    # 启用并启动服务
    run_with_root('systemctl daemon-reload')
    run_with_root('systemctl enable easytier')
    run_with_root('systemctl restart easytier')
    
    print("EasyTier 服务配置完成！")



def check_installation():
    """检查 EasyTier 是否已安装"""
    return os.path.exists('/opt/easytier/easytier')

def main():
    try:
        check_system()
        download_easytier()
        create_systemd_service()
        
        # 等待服务启动
        time.sleep(2)
        
        # 显示状态
        print("\n当前配置信息：")
        print(f"网络名称: {NETWORK_NAME}")
        if USE_PUBLIC_NODE:
            print(f"使用公共节点: {PUBLIC_NODE}")
        else:
            print(f"监听地址: {LISTEN_PORT}")
        
        print("\n使用以下命令查看服务状态：")
        print("systemctl status easytier")
        
    except Exception as e:
        print(f"错误: {str(e)}")
        sys.exit(1)

if __name__ == "__main__":
    main()