# rust-microkernel Development Environment
# Provides all tools needed to build and run the microkernel

FROM rust:latest

# Install build dependencies
RUN apt-get update && apt-get install -y \
    qemu-system-x86 \
    qemu-system-aarch64 \
    build-essential \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Allow rustup to copy instead of rename across Docker layers
ENV RUSTUP_PERMIT_COPY_RENAME=yes

# Install Rust nightly and targets
RUN rustup default nightly && \
    rustup component add rust-src --toolchain nightly && \
    rustup target add x86_64-unknown-none && \
    rustup target add aarch64-unknown-none

# Create workspace
WORKDIR /workspace

# Copy repository (when building image)
COPY . /workspace/

# Set up git safe directory
RUN git config --global --add safe.directory /workspace

# Default command: drop into shell
CMD ["/bin/bash"]

# Usage:
#
# Build image:
#   docker build -t rust-microkernel .
#
# Run container:
#   docker run -it rust-microkernel
#
# Inside container:
#   ./scripts/build-x86.sh && ./scripts/run-x86.sh
#   ./scripts/build-aarch64-virt.sh && ./scripts/run-aarch64-virt.sh
