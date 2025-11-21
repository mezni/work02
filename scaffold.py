import argparse
import os
import json

def load_config(config_file):
    with open(config_file, 'r') as f:
        return json.load(f)

def create_base_directories(project_root):
    dirs = ["services", "deploy", "docs"]
    deploy_dirs = ["docker", "k8s"]
    docker_files = ["docker-compose.dev.yml", "docker-compose.prod.yml", "docker-compose.test.yml"]
    files = [".gitignore", "README.md"]

    for dir in dirs:
        os.makedirs(os.path.join(project_root, dir), exist_ok=True)

    for dir in deploy_dirs:
        os.makedirs(os.path.join(project_root, "deploy", dir), exist_ok=True)

    for file in docker_files:
        open(os.path.join(project_root, "deploy", "docker", file), 'w').close()

    for file in files:
        open(os.path.join(project_root, file), 'w').close()

def add_service(project_root, service_name):
    service_dir = os.path.join(project_root, "services", service_name)
    dirs = ["src", "tests", "docs", "scripts"]

    os.makedirs(service_dir, exist_ok=True)
    for dir in dirs:
        os.makedirs(os.path.join(service_dir, dir), exist_ok=True)

def main():
    parser = argparse.ArgumentParser(description='Scaffold project directories')
    parser.add_argument('--create_base', action='store_true', help='Create base directories')
    parser.add_argument('--create_service', type=str, help='Create a new service')
    args = parser.parse_args()

    config = load_config('config.json')
    project_root = config['project_root']

    if args.create_base:
        create_base_directories(project_root)
    elif args.create_service:
        add_service(project_root, args.create_service)

if __name__ == "__main__":
    main()