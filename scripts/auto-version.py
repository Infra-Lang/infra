#!/usr/bin/env python3
"""
Automatic Version Increment Script for Infra
This script increments the version and updates all necessary files.
"""

import re
import sys
import subprocess
from datetime import datetime

def get_current_version():
    """Get current version from Cargo.toml"""
    try:
        with open('Cargo.toml', 'r') as f:
            content = f.read()
            match = re.search(r'version = "([^"]+)"', content)
            if match:
                return match.group(1)
    except FileNotFoundError:
        pass
    return "0.1.0"

def increment_version(version, bump_type='patch'):
    """Increment version based on bump type"""
    parts = version.split('.')
    major, minor, patch = int(parts[0]), int(parts[1]), int(parts[2])

    if bump_type == 'major':
        major += 1
        minor = 0
        patch = 0
    elif bump_type == 'minor':
        minor += 1
        patch = 0
    else:  # patch
        patch += 1

    return f"{major}.{minor}.{patch}"

def update_cargo_toml(new_version):
    """Update version in Cargo.toml"""
    with open('Cargo.toml', 'r') as f:
        content = f.read()

    # Update package version
    content = re.sub(
        r'version = "[^"]+"',
        f'version = "{new_version}"',
        content
    )

    # Update WIX version if it exists
    content = re.sub(
        r'version = "[^"]+"\s*\n(\s*url)',
        f'version = "{new_version}"\n\\1',
        content
    )

    with open('Cargo.toml', 'w') as f:
        f.write(content)

    print(f"✅ Updated Cargo.toml to version {new_version}")

def update_wix_files(new_version):
    """Update version in WIX files"""
    wix_files = ['wix/main.wxs']

    for wix_file in wix_files:
        try:
            with open(wix_file, 'r') as f:
                content = f.read()

            # Update version attributes
            content = re.sub(
                r'Version="\$\(var\.Version\)"',
                f'Version="{new_version}"',
                content
            )

            # Update version references
            content = re.sub(
                r'\$\(var\.Version\)',
                new_version,
                content
            )

            with open(wix_file, 'w') as f:
                f.write(content)

            print(f"✅ Updated {wix_file} to version {new_version}")
        except FileNotFoundError:
            print(f"⚠️  {wix_file} not found, skipping")

def create_version_commit(new_version):
    """Create a git commit with version changes"""
    try:
        # Add all changes
        subprocess.run(['git', 'add', '.'], check=True)

        # Create commit
        commit_message = f"chore: bump version to {new_version}"
        subprocess.run(['git', 'commit', '-m', commit_message], check=True)

        print(f"✅ Created version commit: {commit_message}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Failed to create version commit: {e}")
        return False

def create_version_tag(new_version):
    """Create and push a git tag"""
    try:
        # Create tag
        tag_name = f"v{new_version}"
        tag_message = f"Infra {tag_name} - Automated Release\n\nAutomatic version bump and release.\n\nDate: {datetime.now().strftime('%Y-%m-%d %H:%M:%S UTC')}"

        subprocess.run(['git', 'tag', '-a', tag_name, '-m', tag_message], check=True)
        print(f"✅ Created tag: {tag_name}")

        # Push commit and tag
        subprocess.run(['git', 'push', 'origin', 'main'], check=True)
        subprocess.run(['git', 'push', 'origin', tag_name], check=True)
        print(f"✅ Pushed tag: {tag_name}")

        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Failed to create/push tag: {e}")
        return False

def main():
    """Main function"""
    if len(sys.argv) > 1:
        bump_type = sys.argv[1]
        if bump_type not in ['major', 'minor', 'patch']:
            print("❌ Invalid bump type. Use: major, minor, or patch")
            sys.exit(1)
    else:
        bump_type = 'patch'  # Default to patch

    # Get current version
    current_version = get_current_version()
    print(f"📦 Current version: {current_version}")

    # Increment version
    new_version = increment_version(current_version, bump_type)
    print(f"🔢 New version: {new_version} ({bump_type} bump)")

    # Update files
    update_cargo_toml(new_version)
    update_wix_files(new_version)

    # Create commit and tag
    if create_version_commit(new_version):
        if create_version_tag(new_version):
            print(f"🎉 Successfully bumped version to {new_version} and pushed tag!")
            print(f"🚀 GitHub Actions will now build and create the release automatically.")
        else:
            print("❌ Failed to create tag")
            sys.exit(1)
    else:
        print("❌ Failed to create commit")
        sys.exit(1)

if __name__ == "__main__":
    main()
