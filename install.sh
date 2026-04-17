#!/bin/bash

# Omni-Kernel Unix Installer (Bash)
REPO_URL="https://github.com/isbatovk-web/Omni-Kernel.git"
INSTALL_DIR="$HOME/Omni-Kernel"

echo -e "\e[36m⚡ Omni-Kernel Installer starting...\e[0m"

# 1. Dependency Check
if ! command -v git &> /dev/null; then
    echo -e "\e[31m❌ Error: Git is not installed.\e[0m"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "\e[31m❌ Error: Rust/Cargo is not installed. Visit https://rustup.rs\e[0m"
    exit 1
fi

# 2. Clone or Update
if [ -d "$INSTALL_DIR" ]; then
    echo -e "\e[33m⚠️  Updating existing installation...\e[0m"
    cd "$INSTALL_DIR" && git pull
else
    echo -e "\e[37m📂 Cloning Omni-Kernel...\e[0m"
    git clone "$REPO_URL" "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# 3. Build
echo -e "\e[37m🏗️  Building binaries...\e[0m"
cargo build --release

echo -e "\n\e[32m✅ Omni-Kernel installed successfully!\e[0m"
echo -e "\e[33m👉 To start: cd ~/Omni-Kernel && cargo run\e[0m"
