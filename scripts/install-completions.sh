#!/bin/bash
# Installation script for kitty-launcher shell completions
# This script automates the setup of bash and zsh shell completions
# 
# Usage: ./scripts/install-completions.sh [bash|zsh|both]
# Default: both

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Check if kitty-launcher binary exists
if ! command -v kitty-launcher &> /dev/null; then
    # Try to find it in the project
    if [[ -f "$PROJECT_DIR/target/release/kitty-launcher" ]]; then
        KITTY_LAUNCHER="$PROJECT_DIR/target/release/kitty-launcher"
    else
        echo -e "${RED}Error: kitty-launcher not found in PATH or project${NC}"
        echo "Please build the project first: cargo build --release"
        exit 1
    fi
else
    KITTY_LAUNCHER="kitty-launcher"
fi

shell_type="${1:-both}"

install_bash_completion() {
    echo -e "${YELLOW}Installing bash completion...${NC}"
    
    local bashrc="$HOME/.bashrc"
    local zshrc="$HOME/.zshrc"
    
    # Check if already installed
    if grep -q "kitty-launcher.*generate-completions" "$bashrc" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Bash completion already installed"
        return 0
    fi
    
    # Create backup
    if [[ -f "$bashrc" ]]; then
        cp "$bashrc" "$bashrc.bak"
        echo -e "${YELLOW}  Created backup: ${bashrc}.bak${NC}"
    fi
    
    # Add completion to bashrc
    if [[ ! -f "$bashrc" ]]; then
        touch "$bashrc"
    fi
    
    echo "" >> "$bashrc"
    echo "# kitty-launcher bash completion" >> "$bashrc"
    echo "eval \"\$($KITTY_LAUNCHER --generate-completions bash)\"" >> "$bashrc"
    
    echo -e "${GREEN}✓${NC} Bash completion installed"
    echo -e "  ${YELLOW}Action required:${NC} Run 'source ~/.bashrc' or restart your terminal"
}

install_zsh_completion() {
    echo -e "${YELLOW}Installing zsh completion...${NC}"
    
    local zshrc="$HOME/.zshrc"
    
    # Check if already installed
    if grep -q "kitty-launcher.*generate-completions" "$zshrc" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Zsh completion already installed"
        return 0
    fi
    
    # Create backup
    if [[ -f "$zshrc" ]]; then
        cp "$zshrc" "$zshrc.bak"
        echo -e "${YELLOW}  Created backup: ${zshrc}.bak${NC}"
    fi
    
    # Add completion to zshrc
    if [[ ! -f "$zshrc" ]]; then
        touch "$zshrc"
    fi
    
    echo "" >> "$zshrc"
    echo "# kitty-launcher zsh completion" >> "$zshrc"
    echo "eval \"\$($KITTY_LAUNCHER --generate-completions zsh)\"" >> "$zshrc"
    
    echo -e "${GREEN}✓${NC} Zsh completion installed"
    echo -e "  ${YELLOW}Action required:${NC} Run 'source ~/.zshrc' or restart your terminal"
}

print_usage() {
    echo "Usage: $0 [bash|zsh|both]"
    echo ""
    echo "Install shell completions for kitty-launcher"
    echo ""
    echo "Options:"
    echo "  bash    Install bash completion only"
    echo "  zsh     Install zsh completion only"
    echo "  both    Install both bash and zsh completions (default)"
    echo ""
    echo "Examples:"
    echo "  $0              # Install both completions"
    echo "  $0 bash         # Install bash only"
    echo "  $0 zsh          # Install zsh only"
}

case "$shell_type" in
    bash)
        install_bash_completion
        ;;
    zsh)
        install_zsh_completion
        ;;
    both)
        install_bash_completion
        echo ""
        install_zsh_completion
        ;;
    --help|-h)
        print_usage
        exit 0
        ;;
    *)
        echo -e "${RED}Error: Unknown shell type '$shell_type'${NC}"
        echo ""
        print_usage
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Restart your terminal or run 'source ~/.bashrc' (bash) / 'source ~/.zshrc' (zsh)"
echo "  2. Type 'kitty-launcher ', 'z-kitty ', or 'zk ' and press TAB to test completions"
echo ""
echo "For uninstallation:"
echo "  - Restore from backup: .bashrc.bak / .zshrc.bak"
echo "  - Or manually remove the kitty-launcher completion lines"
