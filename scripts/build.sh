#!/bin/bash
# Build script wrapper for kitty-launcher
# This script provides a unified build interface with intelligent version detection
# Intelligently detects version from git tags and Cargo.toml
# Usage: ./scripts/build.sh [OPTIONS]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Defaults
RELEASE_BUILD=false
RUN_TESTS=false
STRIP_BINARY=false
FORCE_TIP=false
INSTALL_BINARY=false
TARGET_VERSION=""
TARGET_REF=""

# Helper function to print usage
print_usage() {
    cat << 'EOF'
Usage: ./scripts/build.sh [OPTIONS]

Build Options:
  --release            Build release binary (optimized)
  --test               Run tests after build
  --strip              Strip binary symbols (requires --release)
  --install            Install binary to /usr/local/bin and create z-kitty symlink (requires --release)

Version Detection (default: current checked-out version):
  By default, this script builds the version matching the current git checkout.
  If Cargo.toml and debian/changelog differ, a warning is shown.

Version Options:
  --tip                Build tip of current branch (latest Cargo.toml version)
  --git-version TAG    Build specific git tag or ref (e.g., v0.4.0)
  --version VERSION    Force specific version (e.g., 0.5.1)
  -h, --help          Show this help message

Examples:
  ./scripts/build.sh                        # Build current debug
  ./scripts/build.sh --release --test       # Release build with tests
  ./scripts/build.sh --release --strip      # Release build, stripped
  ./scripts/build.sh --release --install    # Release build and install system-wide
  ./scripts/build.sh --tip --release        # Build latest Cargo.toml version
  ./scripts/build.sh --git-version v0.4.0   # Build specific git tag
  ./scripts/build.sh --version 0.5.1 --release
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            RELEASE_BUILD=true
            shift
            ;;
        --test)
            RUN_TESTS=true
            shift
            ;;
        --strip)
            STRIP_BINARY=true
            shift
            ;;
        --install)
            INSTALL_BINARY=true
            shift
            ;;
        --tip)
            FORCE_TIP=true
            shift
            ;;
        --git-version)
            if [[ -z "$2" ]]; then
                echo -e "${RED}Error: --git-version requires a tag/ref argument${NC}"
                exit 1
            fi
            TARGET_REF="$2"
            shift 2
            ;;
        --version)
            if [[ -z "$2" ]]; then
                echo -e "${RED}Error: --version requires a version argument${NC}"
                exit 1
            fi
            TARGET_VERSION="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
    esac
done

echo -e "${BLUE}=== Kitty Launcher Build ===${NC}"
echo "Project directory: $PROJECT_DIR"

# Change to project directory
cd "$PROJECT_DIR"

# Verify we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Helper function to get Cargo.toml version
get_cargo_version() {
    grep '^version' Cargo.toml | head -1 | sed 's/.*"\([^"]*\)".*/\1/'
}

# Helper function to get current debian changelog version
get_debian_version() {
    if [ -f debian/changelog ]; then
        head -1 debian/changelog | sed 's/.*(\([^)]*\)).*/\1/' | sed 's/-.*//g'
    else
        echo "unknown"
    fi
}

# Helper function to handle git checkout
checkout_version() {
    local ref="$1"
    
    echo -e "${YELLOW}Checking out: $ref${NC}"
    git checkout "$ref" > /dev/null 2>&1 || {
        echo -e "${RED}Error: Failed to checkout $ref${NC}"
        exit 1
    }
}

# Store current branch/ref to restore later if needed
ORIGINAL_REF=$(git rev-parse --abbrev-ref HEAD)
ORIGINAL_COMMIT=$(git rev-parse HEAD)

# Determine version to build
if [[ -n "$TARGET_REF" ]]; then
    echo -e "${BLUE}Building specific git ref: $TARGET_REF${NC}"
    checkout_version "$TARGET_REF"
elif [[ "$FORCE_TIP" == true ]]; then
    echo -e "${BLUE}Building tip of current branch${NC}"
    # Stay on current branch, just get the version
elif [[ -n "$TARGET_VERSION" ]]; then
    echo -e "${BLUE}Building version: $TARGET_VERSION${NC}"
else
    # Default: build current checkout
    echo -e "${BLUE}Building current checked-out version${NC}"
fi

# Get and display version information
CARGO_VER=$(get_cargo_version)
DEBIAN_VER=$(get_debian_version)

echo -e "${BLUE}Cargo.toml version: ${CARGO_VER}${NC}"
if [[ "$DEBIAN_VER" != "unknown" ]]; then
    echo -e "${BLUE}Debian/changelog version: ${DEBIAN_VER}${NC}"
    if [[ "$CARGO_VER" != "$DEBIAN_VER" ]]; then
        echo -e "${YELLOW}⚠ Warning: Version mismatch detected${NC}"
    fi
fi

# Determine build output label
if [[ -n "$TARGET_VERSION" ]]; then
    BUILD_LABEL="$TARGET_VERSION"
elif [[ "$FORCE_TIP" == true ]] || [[ -n "$TARGET_REF" ]]; then
    BUILD_LABEL="$CARGO_VER"
else
    BUILD_LABEL="$CARGO_VER"
fi

echo -e "${GREEN}Building version: $BUILD_LABEL${NC}"
echo ""

# Run cargo build
if [ "$RELEASE_BUILD" = true ]; then
    echo -e "${GREEN}Building release binary...${NC}"
    cargo build --release
    
    # Strip binary if requested
    if [ "$STRIP_BINARY" = true ]; then
        echo -e "${GREEN}Stripping binary...${NC}"
        strip target/release/kitty-launcher
    fi
    
    BINARY_PATH="target/release/kitty-launcher"
else
    echo -e "${GREEN}Building debug binary...${NC}"
    cargo build
    BINARY_PATH="target/debug/kitty-launcher"
fi

# Get binary size
BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
echo -e "${GREEN}Binary created: $BINARY_PATH ($BINARY_SIZE)${NC}"

# Run tests if requested
if [ "$RUN_TESTS" = true ]; then
    echo -e "${GREEN}Running tests...${NC}"
    if [ "$RELEASE_BUILD" = true ]; then
        cargo test --release
    else
        cargo test
    fi
fi

# Restore original checkout if we switched branches
if [[ "$ORIGINAL_REF" != "HEAD" ]] && [[ -n "$TARGET_REF" ]]; then
    echo -e "${YELLOW}Restoring original branch: $ORIGINAL_REF${NC}"
    git checkout "$ORIGINAL_REF" > /dev/null 2>&1 || true
    git reset --hard "$ORIGINAL_COMMIT" > /dev/null 2>&1 || true
fi

# Install system-wide if requested (requires --release)
if [ "$INSTALL_BINARY" = true ]; then
    if [ "$RELEASE_BUILD" != true ]; then
        echo -e "${RED}Error: --install requires --release${NC}"
        exit 1
    fi
    INSTALL_DIR="/usr/local/bin"
    echo -e "${GREEN}Installing kitty-launcher to ${INSTALL_DIR}...${NC}"
    sudo cp "$BINARY_PATH" "${INSTALL_DIR}/kitty-launcher"
    sudo chmod 755 "${INSTALL_DIR}/kitty-launcher"
    # Create z-kitty symlink (remove stale link first if present)
    echo -e "${GREEN}Creating z-kitty symlink in ${INSTALL_DIR}...${NC}"
    sudo ln -sf "${INSTALL_DIR}/kitty-launcher" "${INSTALL_DIR}/z-kitty"
    echo -e "${GREEN}✓ Installed:${NC}"
    echo -e "    ${INSTALL_DIR}/kitty-launcher"
    echo -e "    ${INSTALL_DIR}/z-kitty -> ${INSTALL_DIR}/kitty-launcher"
fi

echo ""
echo -e "${GREEN}✓ Build completed successfully!${NC}"
exit 0
