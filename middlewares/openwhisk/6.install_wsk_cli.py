import os
import sys
import subprocess
import requests
import tarfile
import shutil

def check_wsk_installed():
    try:
        subprocess.run(['wsk', '--version'], capture_output=True, check=True)
        return True
    except (subprocess.SubprocessError, FileNotFoundError):
        return False

def download_and_install_wsk():
    wsk_url = "https://github.com/apache/openwhisk-cli/releases/download/1.2.0/OpenWhisk_CLI-1.2.0-linux-amd64.tgz"
    temp_dir = "/tmp/wsk_install"
    os.makedirs(temp_dir, exist_ok=True)
    
    # Download the CLI
    print("Downloading OpenWhisk CLI...")
    response = requests.get(wsk_url)
    if response.status_code != 200:
        print("Failed to download OpenWhisk CLI")
        sys.exit(1)
    
    # Save the downloaded file
    tar_path = os.path.join(temp_dir, "wsk.tgz")
    with open(tar_path, 'wb') as f:
        f.write(response.content)
    
    # Extract the archive
    print("Extracting OpenWhisk CLI...")
    with tarfile.open(tar_path, 'r:gz') as tar:
        tar.extractall(path=temp_dir)
    
    # Move wsk binary to /usr/local/bin
    wsk_binary = os.path.join(temp_dir, "wsk")
    if os.path.exists(wsk_binary):
        shutil.move(wsk_binary, "/usr/local/bin/wsk")
        os.chmod("/usr/local/bin/wsk", 0o755)
        print("OpenWhisk CLI installed successfully")
    else:
        print("Failed to find wsk binary in the archive")
        sys.exit(1)
    
    # Cleanup
    shutil.rmtree(temp_dir)

def main():
    if not check_wsk_installed():
        print("OpenWhisk CLI not found. Installing...")
        download_and_install_wsk()
    else:
        print("OpenWhisk CLI is already installed")
    
    # Set OpenWhisk properties
    subprocess.run([
        'wsk', 'property', 'set',
        '--apihost', 'http://192.168.31.162:32062',
        '--auth', '23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP'
    ], check=True)

if __name__ == "__main__":
    main()