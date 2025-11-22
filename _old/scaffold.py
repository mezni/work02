import os

def create_project_structure(project_name, microservice_name):
    structure = {
        project_name: [
            '__init__.py',
            'application',
            'domain',
            'infrastructure',
            'presentation',
            'tests',
            'services',
            'docs',
            'deployment'
        ],
        f'{project_name}/application': [
            '__init__.py',
            'commands',
            'queries',
            'handlers'
        ],
        f'{project_name}/domain': [
            '__init__.py',
            'entities',
            'value_objects',
            'repositories'
        ],
        f'{project_name}/infrastructure': [
            '__init__.py',
            'database',
            'messaging'
        ],
        f'{project_name}/presentation': [
            '__init__.py',
            'api',
            'web'
        ],
        f'{project_name}/tests': [
            '__init__.py',
            'unit',
            'integration'
        ],
        f'{project_name}/services/{microservice_name}': [
            '__init__.py'
        ],
        f'{project_name}/deployment': [
            'docker/docker-compose.yml'
        ]
    }

    for directory, files in structure.items():
        os.makedirs(directory, exist_ok=True)
        for file in files:
            if '/' in file:
                sub_dir = os.path.dirname(file)
                os.makedirs(os.path.join(directory, sub_dir), exist_ok=True)
            open(os.path.join(directory, file), 'w').close()

if __name__ == '__main__':
    project_name = input("Enter project name: ")
    microservice_name = input("Enter microservice name: ")
    create_project_structure(project_name, microservice_name)
    print(f"Project {project_name} with microservice {microservice_name} created successfully! ")