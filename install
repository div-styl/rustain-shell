#!/usr/bin/bash

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
MAGENTA='\033[0;35m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# print color
cprint() {
    local color="$1"
    local message="$2"
    printf "${color}${message}${NC}\n"
}

# Install Rust if not in your machine
cprint "$RED" "[*] Checking if system have required packages and commands\n"
if [ -x "$(command -v rustc)" ] && [ -x "$(command -v cargo)" ]; then
    cprint "$YELLOW" "[*] Rust is already installed. Cloning repo..."
    git clone --depth=1 https://github.com/div-styl/rustain-shell > /dev/null 2>&1
    cprint "$GREEN" "[*] Rustain-shell is cloned successfully. Use 'cargo run' to run it."
else
    cprint "$MAGENTA" "[!] Installing Rust"
    if curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; then
        cprint "$YELLOW" "[*] Rust is installed. Cloning repo..."
        git clone --depth=1 https://github.com/div-styl/rustain-shell > /dev/null 2>&1
        cprint "$GREEN" "[*] Rustain-shell cloned successfully. Use 'cargo run' to run it."
    else
        cprint "$RED" "[!] Rust is not installed"
        exit 1
    fi
fi