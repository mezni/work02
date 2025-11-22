import json
import os
import argparse

def read_config(file_path):
    try:
        with open(file_path, 'r') as f:
            config = json.load(f)
            return config
    except FileNotFoundError:
        print(f"File {file_path} not found.")
        return None
    except json.JSONDecodeError as e:
        print(f"Invalid JSON: {e}")
        return None
    
def main():
    parser = argparse.ArgumentParser(description='DDD Microservice Scaffolding Tool')

    config = read_config('config.json')
    if config:      
        project_root = config['project_root']

if __name__ == "__main__":
    main()