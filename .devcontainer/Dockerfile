FROM mcr.microsoft.com/devcontainers/rust:stable

USER root

# Install Debian packaging tools and python tooling
RUN apt-get update && apt-get install -y --no-install-recommends \
    devscripts debhelper fakeroot dpkg-dev gnupg lintian python3-pip python3-venv curl git build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install pipx for user-local python apps
RUN python3 -m pip install --no-input --user pipx && python3 -m pipx ensurepath || true

# Put user-local binaries and cargo into PATH
ENV PATH=/root/.local/bin:/root/.cargo/bin:${PATH}

USER vscode
