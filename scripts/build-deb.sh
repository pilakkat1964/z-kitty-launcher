#!/bin/bash
# Debian package build script wrapper for kitty-launcher
# This script provides a unified interface for building debian packages
# Intelligently detects version from git tags and Cargo.toml
# Usage: ./scripts/build-deb.sh [OPTIONS]

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
CLEAN_BUILD=false
SIGNED=false
FORCE_TIP=false
TARGET_VERSION=""
TARGET_REF=""

# Helper function to print usage
print_usage() {
    cat << 'EOF'
Usage: ./scripts/build-deb.sh [OPTIONS]

Version Detection (default: current checked-out version):
  By default, this script builds the version matching the current git checkout.
  If Cargo.toml and debian/changelog differ, a warning is shown.

Options:
  --clean              Clean build artifacts before building
  --signed             Sign the debian package (requires GPG key)
  --tip                Build tip of current branch (latest Cargo.toml version)
  --git-version TAG    Build specific git tag or ref (e.g., v0.4.0)
  --version VERSION    Force specific Debian version (e.g., 0.5.1-1)
  -h, --help          Show this help message

Examples:
  ./scripts/build-deb.sh                    # Build current checkout
  ./scripts/build-deb.sh --clean            # Clean and build current
  ./scripts/build-deb.sh --tip --clean      # Build latest Cargo.toml version
  ./scripts/build-deb.sh --git-version v0.4.0
  ./scripts/build-deb.sh --version 0.5.2-1  # Force specific version
  ./scripts/build-deb.sh --signed --clean   # Signed release build
EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        --signed)
            SIGNED=true
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

echo -e "${BLUE}=== Kitty Launcher Debian Package Builder ===${NC}"
echo "Project directory: $PROJECT_DIR"

# Change to project directory
cd "$PROJECT_DIR"

# Check for required tools
echo -e "${GREEN}Checking dependencies...${NC}"
for cmd in dpkg-buildpackage cargo rustc git; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "${RED}Error: $cmd not found. Please install build dependencies.${NC}"
        echo "On Debian/Ubuntu: sudo apt-get install build-essential debhelper dpkg-dev cargo rustc git"
        exit 1
    fi
done

# Check for debhelper
if [ ! -d "/usr/share/debhelper" ]; then
    echo -e "${RED}Error: debhelper not found. Please install build dependencies.${NC}"
    echo "On Debian/Ubuntu: sudo apt-get install debhelper"
    exit 1
fi

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
    head -1 debian/changelog | sed 's/.*(\([^)]*\)).*/\1/'
}

# Helper function to handle git checkout and version update
checkout_and_update_version() {
    local ref="$1"
    local target_ver="$2"
    
    echo -e "${YELLOW}Checking out: $ref${NC}"
    git checkout "$ref" > /dev/null 2>&1 || {
        echo -e "${RED}Error: Failed to checkout $ref${NC}"
        exit 1
    }
    
    local cargo_ver=$(get_cargo_version)
    echo -e "${BLUE}Cargo.toml version: ${cargo_ver}${NC}"
    
    if [[ -z "$target_ver" ]]; then
        target_ver="${cargo_ver}-1"
    fi
    
    # Update debian/changelog if needed
    local current_deb=$(get_debian_version)
    if [[ "$current_deb" != "$target_ver" ]]; then
        echo -e "${YELLOW}Updating debian/changelog from $current_deb to $target_ver${NC}"
        local temp_changelog=$(mktemp)
        cat > "$temp_changelog" << CHANGELOG_EOF
kitty-launcher ($target_ver) unstable; urgency=medium

  * Build for version $target_ver

 -- OpenCode Contributors <contributors@opencode.ai>  $(date -R)

CHANGELOG_EOF
        tail -n +2 debian/changelog >> "$temp_changelog"
        mv "$temp_changelog" debian/changelog
    fi
}

# Store current branch/ref to restore later if needed
ORIGINAL_REF=$(git rev-parse --abbrev-ref HEAD)
ORIGINAL_COMMIT=$(git rev-parse HEAD)

# Determine version to build
if [[ -n "$TARGET_REF" ]]; then
    echo -e "${BLUE}Building specific git ref: $TARGET_REF${NC}"
    checkout_and_update_version "$TARGET_REF" "$TARGET_VERSION"
elif [[ "$FORCE_TIP" == true ]]; then
    echo -e "${BLUE}Building tip of current branch${NC}"
    cargo_ver=$(get_cargo_version)
    deb_ver=$(get_debian_version)
    if [[ "$cargo_ver-1" != "$deb_ver" ]]; then
        echo -e "${YELLOW}Updating debian/changelog to match Cargo.toml ($cargo_ver)${NC}"
        target_ver="${cargo_ver}-1"
        temp_changelog=$(mktemp)
        cat > "$temp_changelog" << CHANGELOG_EOF
kitty-launcher ($target_ver) unstable; urgency=medium

  * Build for version $target_ver

 -- OpenCode Contributors <contributors@opencode.ai>  $(date -R)

CHANGELOG_EOF
        tail -n +2 debian/changelog >> "$temp_changelog"
        mv "$temp_changelog" debian/changelog
    fi
elif [[ -n "$TARGET_VERSION" ]]; then
    echo -e "${BLUE}Forcing version: $TARGET_VERSION${NC}"
    temp_changelog=$(mktemp)
    cat > "$temp_changelog" << CHANGELOG_EOF
kitty-launcher ($TARGET_VERSION) unstable; urgency=medium

  * Build for version $TARGET_VERSION

 -- OpenCode Contributors <contributors@opencode.ai>  $(date -R)

CHANGELOG_EOF
    tail -n +2 debian/changelog >> "$temp_changelog"
    mv "$temp_changelog" debian/changelog
else
    # Default: build current checkout
    echo -e "${BLUE}Building current checked-out version${NC}"
    cargo_ver=$(get_cargo_version)
    deb_ver=$(get_debian_version)
    echo -e "${BLUE}Current Cargo.toml version: ${cargo_ver}${NC}"
    echo -e "${BLUE}Current Debian version: ${deb_ver}${NC}"
    
    if [[ "$cargo_ver-1" != "$deb_ver" ]]; then
        echo -e "${YELLOW}⚠ Warning: Version mismatch detected${NC}"
        echo -e "${YELLOW}  Cargo.toml: $cargo_ver${NC}"
        echo -e "${YELLOW}  Debian: $deb_ver${NC}"
        echo -e "${YELLOW}  Using Debian version for package build${NC}"
    fi
fi

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    echo -e "${GREEN}Cleaning build artifacts...${NC}"
    cargo clean
    rm -f kitty-launcher_*.deb
    rm -f kitty-launcher_*.build
    rm -f kitty-launcher_*.changes
fi

# Build source first to verify everything compiles
echo -e "${GREEN}Building Rust binary...${NC}"
cargo build --release
strip target/release/kitty-launcher

# Build Debian package
echo -e "${GREEN}Building Debian package...${NC}"
BUILD_VERSION=$(get_debian_version)
echo -e "${BLUE}Building package version: ${BUILD_VERSION}${NC}"

if [ "$SIGNED" = true ]; then
    echo -e "${YELLOW}Building signed package (requires GPG key)...${NC}"
    dpkg-buildpackage -uc
else
    echo -e "${YELLOW}Building unsigned package...${NC}"
    dpkg-buildpackage -us -uc -d
fi

# Find and verify the built packages
echo -e "${GREEN}Locating build artifacts...${NC}"
DEB_FILE=$(ls -1 ../kitty-launcher_*.deb 2>/dev/null | tail -1)

if [ -z "$DEB_FILE" ]; then
    echo -e "${RED}Error: Debian package not found after build.${NC}"
    exit 1
fi

echo -e "${GREEN}Debian package created: $(basename $DEB_FILE)${NC}"
ls -lh "$DEB_FILE"

# Copy to project directory
cp "$DEB_FILE" .
echo -e "${GREEN}Package copied to project directory.${NC}"

# Restore original checkout if we switched branches
if [[ "$ORIGINAL_REF" != "HEAD" ]] && [[ -n "$TARGET_REF" ]]; then
    echo -e "${YELLOW}Restoring original branch: $ORIGINAL_REF${NC}"
    git checkout "$ORIGINAL_REF" > /dev/null 2>&1 || true
    git reset --hard "$ORIGINAL_COMMIT" > /dev/null 2>&1 || true
fi

echo ""
echo -e "${GREEN}✓ Debian package build completed successfully!${NC}"
echo -e "${YELLOW}Package file:${NC} $(basename $DEB_FILE)"
echo -e "${YELLOW}Install with:${NC} sudo dpkg -i $(basename $DEB_FILE)"
exit 0
