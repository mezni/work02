import argparse
import json
from pathlib import Path
from typing import Dict, Any

def load_config(config_file: str) -> Dict[str, Any]:
    with open(config_file, 'r') as f:
        return json.load(f)

def create_dir(path: Path):
    try:
        path.mkdir(parents=True, exist_ok=True)
    except Exception as e:
        print(f"Error creating directory {path}: {e}")

def create_file(path: Path):
    try:
        path.touch()
    except Exception as e:
        print(f"Error creating file {path}: {e}")

def create_structure(project_root: Path):
    dirs = [
        "services",
        "deploy/docker",
        "deploy/k8s/infrastructure", 
        "deploy/k8s/services",
        "deploy/scripts",
        "docs/architecture",
        "docs/api",
        "docs/development",
        "docs/deployment",
        ".github/workflows"
    ]
    files = {
        "": [".gitignore", "README.md"],
        "deploy/docker": ["docker-compose.dev.yml", "docker-compose.prod.yml", "docker-compose.test.yml"],
        "deploy/k8s": ["namespace.yaml", "configmap.yaml", "secrets.yaml"],
        "deploy/scripts": ["deploy.sh", "health-check.sh", "migrate-db.sh"],
        "docs": ["architecture/microservices.md", "architecture/data-flow.md", "architecture/domain-model.md",
                 "api/gateway-swagger.yaml", "development/setup.md", "development/testing.md",
                 "development/coding-standards.md", "deployment/local.md", "deployment/production.md", 
                 "deployment/kubernetes.md", "README.md", "api-guidelines.md", "deployment-guide.md"]
    }

    for dir in dirs:
        create_dir(project_root / dir)

    for dir, file_list in files.items():
        for file in file_list:
            create_file(project_root / dir / file)

def add_service(project_root: Path, service_name: str):
    service_dir = project_root / "services" / service_name
    
    ddd_dirs = [
        "src/domain/entities",
        "src/domain/value_objects",
        "src/domain/events", 
        "src/domain/services",
        "src/domain/common",
        "src/domain/enums",
        "src/domain/models",
        "src/application/use_cases",
        "src/application/commands",
        "src/application/queries",
        "src/application/dtos", 
        "src/application/common",
        "src/infrastructure/database/repositories",
        "src/infrastructure/database/connections",
        "src/infrastructure/database/migrations",
        "src/infrastructure/web",
        "src/infrastructure/messaging",
        "src/infrastructure/config",
        "src/infrastructure/common",
        "src/interfaces/controllers", 
        "src/interfaces/api",
        "src/interfaces/common",
        "tests/unit",
        "tests/integration",
        "tests/fixtures",
        "docs",
        "scripts",
        "migrations"
    ]
    config_files = [
        "Cargo.toml",
        "Dockerfile",
        ".env.example", 
        "README.md"
    ]

    for dir in ddd_dirs:
        create_dir(service_dir / dir)

    for file in config_files:
        create_file(service_dir / file)

def main():
    parser = argparse.ArgumentParser(description='DDD Microservice Scaffolding Tool')
    parser.add_argument('--create-structure', action='store_true', 
                       help='Create complete DDD microservice structure')
    parser.add_argument('--create-service', type=str, 
                       help='Create a new service with DDD structure')
    parser.add_argument('--config', type=str, default='config.json',
                       help='Path to configuration file (default: config.json)')
    
    args = parser.parse_args()
    
    config = load_config(args.config)
    project_root = Path(config['project_root'])

    if args.create_structure:
        create_structure(project_root)
    elif args.create_service:
        if 'services' in config and args.create_service in [s['name'] for s in config['services']]:
            add_service(project_root, args.create_service)
        else:
            print(f"Service '{args.create_service}' not found in config.json")
    else:
        parser.print_help()

if __name__ == "__main__":
    main()