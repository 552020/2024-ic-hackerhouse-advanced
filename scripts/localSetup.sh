#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting development environment setup...${NC}"

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check and install dependencies
check_dependencies() {
    echo -e "${GREEN}Checking dependencies...${NC}"
    
    # Check Docker
    if ! command_exists docker; then
        echo "Docker not found. Please install Docker first."
        exit 1
    fi

    # Check if dfx is installed
    if ! command_exists dfx; then
        echo "Installing dfx..."
        sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
    fi
}

# Build development environment
build_dev_env() {
    echo -e "${GREEN}Building development environment...${NC}"
    docker build -t ic-dev-env .
}

# Setup development identity
setup_dfx_identity() {
    echo -e "${GREEN}Checking dfx identity...${NC}"
    
    # List current identities
    if dfx identity list | grep -q 'current'; then
        echo "Identity already exists and is set"
        dfx identity get-principal
        return 0
    fi

    # If no identity exists, create a new one
    echo "No identity found. Creating new identity..."
    dfx identity new default_developer --storage-mode=plaintext
    dfx identity use default_developer
    dfx identity get-principal
}

# Initialize project
init_project() {
    echo -e "${GREEN}Initializing project...${NC}"
    npm install
    dfx start --background
    dfx deploy
    dfx stop
}

# Main setup process
main() {
    check_dependencies
    
    # If we're not in a container, build the dev environment
    if [ ! -f /.dockerenv ]; then
        build_dev_env
        echo -e "${GREEN}You can now run the development environment with:${NC}"
        echo "docker run -it --rm -v \$(pwd):/app ic-dev-env bash"
    else
        setup_dfx_identity
        init_project
    fi
}

main "$@"