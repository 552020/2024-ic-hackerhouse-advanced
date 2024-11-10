FROM ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    g++ \
    git \
    pkg-config \
    sudo \
    nodejs \
    libc6 \
    libc6-dev

# Install Rust
RUN mkdir -p /root/.cargo && \
    chmod 777 /root/.cargo && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path && \
    chmod -R 777 /root/.cargo && \
    . /root/.cargo/env

# Install DFX using dfxvm
RUN DFXVM_INIT_NONINTERACTIVE=1 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"

# Set working directory
WORKDIR /app

# Command to run when starting the container
CMD ["/bin/bash"]