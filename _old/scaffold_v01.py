import json
import os

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

def snake_to_pascal(snake_str: str) -> str:
    return ''.join(word.capitalize() for word in snake_str.split('_'))


def create_enum(project_root, service_name, enum_name, variants):
    dir_path = f"{project_root}/services/{service_name}/domain/enums"

    file_path = f"{dir_path}/{enum_name}.rs"
    mod_path = os.path.join(dir_path, "mod.rs")
    
    # Create the directory if it doesn't exist
    os.makedirs(dir_path, exist_ok=True)

    # Check if mod.rs exists, create if it doesn't
    if not os.path.exists(mod_path):
        with open(mod_path, 'w') as f:
            f.write("")  # Create empty file
    
    # Read existing content to avoid duplicates
    existing_content = ""
    if os.path.exists(mod_path):
        with open(mod_path, 'r') as f:
            existing_content = f.read()
    
    # Check if module is already declared
    module_declaration = f"pub mod {enum_name};"
    if not module_declaration in existing_content:
        # Append module declaration
        with open(mod_path, 'a') as f:
            # Add newline if file is not empty
            if existing_content and not existing_content.endswith('\n'):
                f.write('\n')
            f.write(f"pub mod {enum_name};\n")

    pascal_name = snake_to_pascal(enum_name)
    
    # Build the variants section
    variants_lines = []
    for variant in variants:
        variants_lines.append(f"    {variant},")
    variants_section = "\n".join(variants_lines)
    
    # Build the FromStr match arms
    match_arms = []
    for variant in variants:
        match_arms.append(f'            "{variant}" => Ok({pascal_name}::{variant}),')
    match_section = "\n".join(match_arms)
    
    content = f"""use serde::{{Deserialize, Serialize}};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum {pascal_name} {{
{variants_section}
}}

impl fmt::Display for {pascal_name} {{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{
        write!(f, "{{:?}}", self).map(|_| ())
    }}
}}

impl FromStr for {pascal_name} {{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s.to_uppercase().as_str() {{
{match_section}
            _ => Err("VariantNotFound"),
        }}
    }}
}}"""


    with open(file_path, 'w') as f:
        f.write(content)

    print(f"Enum {enum_name} created in {file_path}")

def main():
    config = read_config('config.json')
    if config:      
        project_root = config['project_root']
        for service in config['services']:
            service_name = service['name']
            for enum in service['enums']:
                enum_name = enum['name']
                variants = enum['variants']
                print(f"  - {enum_name}: {variants}")
                create_enum(project_root, service_name, enum_name, variants)

if __name__ == "__main__":
    main()