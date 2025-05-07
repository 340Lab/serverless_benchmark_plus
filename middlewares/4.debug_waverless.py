import subprocess, sys, os
import json

def get_pods_on_node(node_name):
    try:
        # 执行 kubectl 命令获取节点上的 Pod 列表
        result = subprocess.run(
            ["kubectl", "get", "pods", "--all-namespaces", "-o", "json"],
            capture_output=True,
            text=True,
            check=True
        )

        # 解析 JSON 输出
        pods = json.loads(result.stdout)

        # 筛选出指定节点上的 Pod
        pods_on_node = [
            pod for pod in pods['items']
            if pod['spec']['nodeName'] == node_name
        ]

        return pods_on_node

    except subprocess.CalledProcessError as e:
        print(f"Error executing kubectl command: {e}")
        return []
    except json.JSONDecodeError as e:
        print(f"Error parsing JSON output: {e}")
        return []

# 示例调用
if len(sys.argv) < 2:
    print("Usage: python script.py <node_name>")
    sys.exit(1)

node_name = sys.argv[1]
pods = get_pods_on_node(node_name)
for pod in pods:
    # print(f"Pod Name: {pod['metadata']['name']} Namespace: {pod['metadata']['namespace']}")
    podname=pod['metadata']['name']
    if podname.find("waverless")>=0:
        print(f"Pod Name: {podname} Namespace: {pod['metadata']['namespace']}")
        os.system(f"kubectl logs -f -n {pod['metadata']['namespace']} {podname}")