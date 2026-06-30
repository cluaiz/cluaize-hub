#!/usr/bin/env python3
import os
import json
import yaml
from pathlib import Path
from datetime import datetime, timezone

def parse_frontmatter(file_path):
    """Extract and parse YAML frontmatter or Markdown Table from a markdown file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        data = {}
        
        # Check for YAML frontmatter
        if content.startswith('---'):
            parts = content.split('---')
            if len(parts) >= 3:
                frontmatter_text = parts[1]
                data = yaml.safe_load(frontmatter_text)
                if data:
                    return data
        
        # Check for Markdown Table at the top
        lines = content.strip().split('\n')
        for line in lines:
            if line.startswith('|') and 'Attribute' not in line and '---' not in line:
                cols = [c.strip() for c in line.split('|') if c.strip()]
                if len(cols) >= 2:
                    key = cols[0].replace('**', '').strip()
                    val = cols[1].replace('`', '').strip()
                    data[key] = val
            elif line.startswith('#'):
                break # Stop parsing table once heading starts
                
        return data if data else None
    except Exception as e:
        print(f"Error parsing {file_path}: {e}")
        return None

def scan_directory(base_path, expected_file="SKILL.md", item_type="skill"):
    """Scan a directory and return a lean dictionary mapping name to path and version."""
    items = {}
    base_dir = Path(base_path)
    
    if not base_dir.exists():
        return items
        
    for category_dir in base_dir.iterdir():
        if not category_dir.is_dir():
            continue
            
        # Treat top level souls differently for legacy compatibility if needed
        if item_type == "soul" and (category_dir / expected_file).exists():
            pkg_json = category_dir / "package.json"
            if pkg_json.exists():
                process_package_json(pkg_json, items)
            else:
                process_legacy_markdown(category_dir / expected_file, items)
            continue
            
        # Check if the package is directly under the base directory (1-level hierarchy like Extensions)
        pkg_json = category_dir / "package.json"
        if pkg_json.exists():
            process_package_json(pkg_json, items)
            continue
            
        md_file = category_dir / expected_file
        if md_file.exists():
            process_legacy_markdown(md_file, items)
            continue
            
        # Otherwise, check if it is a category containing packages (2-level hierarchy like Skills)
        for item_dir in category_dir.iterdir():
            if not item_dir.is_dir():
                continue
                
            pkg_json = item_dir / "package.json"
            if pkg_json.exists():
                process_package_json(pkg_json, items)
            else:
                md_file = item_dir / expected_file
                if md_file.exists():
                    process_legacy_markdown(md_file, items)
                    
    return items

def process_package_json(pkg_json, items):
    """Process a package.json file and extract registry data."""
    try:
        with open(pkg_json, 'r', encoding='utf-8') as f:
            data = json.load(f)
            
        name = data.get("id")
        if not name:
            return
            
        version = data.get("latest_version", "1.0.0")
        
        # Get the URL directly from package.json if it exists
        direct_url = None
        if "versions" in data and version in data["versions"]:
            v_data = data["versions"][version]
            if "files" in v_data and "file_directory" in v_data["files"]:
                direct_url = v_data["files"]["file_directory"]
                
        if not direct_url:
            # Fallback if package.json is missing URL
            prefix = "ext"
            category = data.get("hub_type", "skill")
            if category == "skill": prefix = "skill"
            elif category == "plugin": prefix = "plugin"
            elif category == "mcp": prefix = "mcp"
            elif category == "soul": prefix = "soul"
            
            tag_name = f"{prefix}-{name}-v{version}"
            direct_url = f"https://github.com/cluaiz/cluaiz-hub/releases/download/{tag_name}/{name}-files.zip"
            
        items[name] = {
            "latest": version,
            "versions": {
                version: direct_url
            }
        }
    except Exception as e:
        print(f"Error reading package.json at {pkg_json}: {e}")

def process_legacy_markdown(md_file, items):
    """Process a legacy markdown file and extract registry data."""
    frontmatter = parse_frontmatter(md_file)
    if frontmatter and 'name' in frontmatter:
        name = frontmatter['name']
        version = str(frontmatter.get('version', '1.0.0'))
        
        # Legacy fallback URL
        direct_url = f"https://github.com/cluaiz/skills/releases/download/{name}-latest/{name}-v{version}.zip"
        
        items[name] = {
            "latest": version,
            "versions": {
                version: direct_url
            }
        }

def build_registry():
    """Build the full lean registry.json index."""
    print("Building Lean Cluaiz Hub Registry...")
    
    root_dir = Path(__file__).parent.parent
    
    extensions = scan_directory(root_dir / "extensions", "package.json", "extension")
    
    skills = scan_directory(root_dir / "skills", "SKILL.md", "skill")
    plugins = scan_directory(root_dir / "plugins", "SKILL.md", "plugin")
    mcp = scan_directory(root_dir / "mcp", "SKILL.md", "mcp")
    souls = scan_directory(root_dir / "souls", "SOUL.md", "soul")
    
    registry = {
        "_meta": {
            "updated": datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z')
        },
        "extensions": extensions,
        "skills": skills,
        "plugins": plugins,
        "mcp": mcp,
        "souls": souls
    }
    
    # Write to registry.json
    out_file = root_dir / "registry.json"
    with open(out_file, 'w', encoding='utf-8') as f:
        json.dump(registry, f, indent=2)
        
    print(f"Registry built successfully at {out_file}")
    print(f"Found: {len(extensions)} extensions, {len(skills)} skills, {len(plugins)} plugins, {len(mcp)} mcp, {len(souls)} souls.")

if __name__ == "__main__":
    build_registry()
