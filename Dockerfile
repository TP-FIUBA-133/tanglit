# Stage 1: Builder
# This stage installs all the dependencies needed to build the Tauri application.
FROM rust:1.77 AS builder

# Set environment variables to non-interactive to avoid prompts during installation
ENV DEBIAN_FRONTEND=noninteractive

# Install sudo and create a non-root user for security
RUN apt-get update && apt-get install -y sudo && \
    useradd -m -s /bin/bash builder && \
    echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# Switch to the non-root user
USER builder
WORKDIR /home/builder

# Install Node.js using nvm (Node Version Manager) and pnpm
# The nvm environment is sourced here but only applies to this RUN command.
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash && \
    export NVM_DIR="$HOME/.nvm" && \
    [ -s "$NVM_DIR/nvm.sh" ] && \
    \. "$NVM_DIR/nvm.sh"  && \
    nvm install 20 && \
    npm install -g pnpm

# Install Tauri dependencies
RUN sudo apt-get update && sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    clang \
    lld \
    llvm \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libssl-dev \
    xdg-utils

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]