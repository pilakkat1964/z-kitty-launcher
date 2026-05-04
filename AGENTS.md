# AGENTS.md - Kitty Launcher Project Status

## Project Overview

**Project**: Kitty Launcher - Terminal Session Manager for Kitty Emulator  
**Status**: ✅ **PRODUCTION READY** (v0.4.0)  
**Location**: `/home/sysadmin/workspace/Opencode-workspaces/z-tools/kitty-launcher/`  
**Language**: Rust (100% pure, zero external dependencies)  
**License**: MIT  
**Repository**: https://github.com/pilakkat1964/kitty-launcher  

---

## Current Status

### Version: 0.5.2 (Latest)
- **Release**: Session File Directive Parsing
- **Build**: ✅ Clean (0 warnings, 0 errors)
- **Tests**: ✅ 19/19 passing
- **Git**: ✅ SSH+Git fully operational with pilakkat1964 account
- **Debian**: ✅ Package builds successfully

### Quick Facts
- **Source Code**: ~1,150 lines (src/main.rs)
- **Documentation**: 1,600+ lines (README, man page, info page, guides)
- **Binary Size**: ~509 KB (release, stripped)
- **Build Time**: ~1.5 seconds
- **Test Suite**: 19 unit tests (validation, completions, desktop integration, directive parsing)

---

## Version History

### v0.5.2 - Session File Directive Parsing
- Embedded launcher directives in `.session` files via `#%[ key = value ]%#` syntax
- `currentWorkingDir` directive (and `cwd` shorthand) passes `-d <path>` to kitty
- Tilde (`~`) expansion for directive values
- Directive output printed to stdout on launch for transparency
- Graceful handling of unknown/malformed directives (warnings, no failure)
- Updated default session template with directive usage example
- Updated help output with SESSION DIRECTIVES section
- Fixed `build-deb.sh`: removed `local` keyword used outside functions
- Added `~/.local/share/kitty/sessions` as search path #4
- Added `.kitty-session` as accepted extension alongside `.session` (parity with kitty)
- 9 new unit tests; 19 total

### v0.5.1 - Build System Improvements
- Add intelligent version detection to build scripts
- Add --tip flag to build latest Cargo.toml version
- Add --git-version TAG flag for building specific git tags
- Add --version VERSION flag for forced version override
- Enhance build-deb.sh with automatic debian/changelog synchronization

### v0.4.0 - Shell Completions
- Bash completion script with session suggestions
- Zsh completion script with descriptions
- `--generate-completions` flag for dynamic generation
- `scripts/install-completions.sh` automated installer
- Debian package integration for completions
- README documentation updates

### v0.3.0 - Desktop Integration
- .desktop launcher file creation (`--create-launcher`, `-l`)
- System launcher installation (`--install-launcher`, `-i`)
- KDE Plasma folder view support
- Desktop environment integration

### v0.2.0 - Documentation & Sessions
- Session file creation (`--create`, `-c`)
- Comprehensive help system (`-h`, `--help`)
- Man page (kitty-launcher.1)
- Info page (kitty-launcher.info)
- Version flag (`-V`, `--version`)

### v0.1.0 - Core
- Session launch functionality
- Input validation and security
- Multiple configuration search paths
- Error handling with exit codes

---

## Key Features

✅ **Session Management**
- Launch terminal sessions with `kitty-launcher <session>`
- Create sessions: `kitty-launcher -c <name>`
- Session files stored in `~/.local/etc/kitty/`
- Automatic `.session` extension fallback

✅ **Desktop Integration**
- Create .desktop launchers: `kitty-launcher -l <name> [session]`
- System installation: `kitty-launcher -i`
- KDE Plasma cascading menus support

✅ **Shell Completions**
- Bash: `kitty-launcher --generate-completions bash`
- Zsh: `kitty-launcher --generate-completions zsh`
- Auto-discovers available sessions
- Automated setup via `./scripts/install-completions.sh`

✅ **Security & Validation**
- Path traversal attack prevention
- Input sanitization (alphanumeric, hyphens, underscores, dots only)
- Proper exit codes (0=success, 1=runtime error, 2=config error)
- Shell injection protection

✅ **Documentation**
- In-app help: `kitty-launcher -h`
- Man page: `man kitty-launcher`
- Info page: `info kitty-launcher`
- README with power-user workflows
- Learning guide for Rust beginners

---

## Project Structure

```
kitty-launcher/
├── src/main.rs                    # 957 lines - Full implementation
├── Cargo.toml                     # Project manifest (v0.4.0)
├── README.md                      # 614 lines - Main documentation
├── kitty-launcher.1               # Man page
├── kitty-launcher.info            # Info page
├── LEARNING_GUIDE.md             # Rust learning guide
├── INSTALL.md                     # Installation guide
├── scripts/
│   ├── build.sh                  # Build wrapper
│   ├── build-deb.sh              # Debian build wrapper
│   ├── install-completions.sh    # Completion installer (v0.4.0)
│   └── README.md                 # Build documentation
├── debian/                        # Debian package config
│   ├── control
│   ├── rules                     # Updated for completions
│   ├── changelog                 # v0.4.0 entry
│   ├── copyright
│   ├── compat
│   └── source/format
├── .github/workflows/            # GitHub Actions CI/CD
├── kitty-launcher.desktop        # System launcher template
├── kitty-launcher.png            # Icon (256x256)
├── kitty-launcher-icon.svg       # Icon (vector)
└── .git/                         # Version control (synced)
```

---

## Git & SSH Access

### SSH+Git Status: ✅ FULLY OPERATIONAL
- **Protocol**: SSH with ED25519 key
- **Key**: `~/.ssh/id_ed25519_pilakkat`
- **Config**: `~/.ssh/config` (auto-created)
- **Remote**: `git@github.com:pilakkat1964/kitty-launcher.git`
- **Account**: pilakkat1964 (pilakkat1964@gmail.com)
- **Access**: Read ✓ Write ✓ Push ✓ Pull ✓ Tags ✓

### Git Operations Verified
```bash
git status              # ✓ Shows clean working tree
git fetch origin        # ✓ Works via SSH
git pull origin master  # ✓ Works via SSH
git push origin master  # ✓ Write access confirmed
git tag -l             # ✓ v0.1.0, v0.2.0, v0.3.0, v0.4.0
```

### GitHub Repository
- URL: https://github.com/pilakkat1964/kitty-launcher
- All commits synchronized with remote
- All tags pushed and accessible
- CI/CD workflows configured

---

## Build & Test

### Build & Test

### Build Release Binary
```bash
cd /home/sysadmin/workspace/Opencode-workspaces/z-tools/kitty-launcher
cargo build --release
# Output: target/release/kitty-launcher (~509 KB)
```

### Run Tests
```bash
cargo test
# Output: test result: ok. 18 passed; 0 failed
```

### Test Coverage
- `test_validate_session_name_valid` ✓
- `test_validate_session_name_invalid` ✓
- `test_validate_session_name_with_extensions` ✓
- `test_session_name_variants` ✓
- `test_create_session_validation` ✓
- `test_create_launcher_validation` ✓
- `test_desktop_file_content` ✓
- `test_parse_directives_basic` ✓
- `test_parse_directives_tilde_expansion` ✓
- `test_parse_directives_bare_tilde` ✓
- `test_parse_directives_empty` ✓
- `test_parse_directives_unknown_key` ✓
- `test_parse_directives_malformed_no_equals` ✓
- `test_parse_directives_malformed_no_close_token` ✓
- `test_parse_directives_case_insensitive_key` ✓
- `test_parse_directives_cwd_shorthand` ✓
- `test_expand_tilde_absolute_path` ✓
- `test_expand_tilde_no_tilde` ✓
- `test_validate_session_name_kitty_session_extension` ✓

### Build Debian Package (AMD64 only)
```bash
./scripts/build-deb.sh --clean
# Output: kitty-launcher_0.4.0-1_amd64.deb
```

### GitHub Actions - Multi-Architecture Builds
- **Workflow**: `.github/workflows/release.yml`
- **Trigger**: Tag push (v*)
- **Build Matrix**:
  - AMD64 (x86_64-unknown-linux-gnu) - Native build with cargo
  - ARM64 (aarch64-unknown-linux-gnu) - Cross-compilation with `cross` crate
- **Outputs**:
  - Precompiled binary for AMD64
  - Debian packages for AMD64 and ARM64
  - Source archive (tar.gz)
- **Permissions**: Write access to GitHub repository (for release creation)
- **Features**:
  - Automatic .deb package generation with shell completions
  - Binary stripping for optimized size
  - Release notes with installation instructions
  - Artifact retention: 1 day

---

## Usage Examples

### Session Launch
```bash
kitty-launcher dev                      # Launch existing session
kitty-launcher -h                       # Show help
kitty-launcher --version                # Show version
```

### Session Creation
```bash
kitty-launcher -c my-project            # Create new session
$EDITOR ~/.local/etc/kitty/my-project.session
kitty-launcher my-project               # Launch created session
```

### Desktop Integration
```bash
kitty-launcher -l "Development" dev     # Create desktop launcher
kitty-launcher -i                       # Install main system launcher
```

### Shell Completions
```bash
./scripts/install-completions.sh bash   # Install bash
./scripts/install-completions.sh zsh    # Install zsh
./scripts/install-completions.sh both   # Install both
```

### Generate Completions Manually
```bash
kitty-launcher --generate-completions bash >> ~/.bashrc
kitty-launcher --generate-completions zsh >> ~/.zshrc
```

---

## Implementation Details

### Core Functions (src/main.rs)
- `print_help()` - Comprehensive help system
- `print_version()` - Version information
- `validate_session_name()` - Input validation
- `find_config_file()` - Configuration discovery
- `create_session_file()` - Session creation
- `create_launcher_file()` - .desktop file creation
- `create_system_launcher()` - System registration
- `parse_session_directives()` - Session file directive parser
- `expand_tilde()` - Tilde expansion helper
- `launch_kitty()` - Session launching (applies directives)
- `generate_bash_completion()` - Bash script generation
- `generate_zsh_completion()` - Zsh script generation

### Configuration Search Paths (Priority Order)
1. `./etc/kitty/` (current directory)
2. `~/.local/etc/kitty/` (user directory)
3. `/opt/etc/kitty/` (system-wide)
4. `~/.local/share/kitty/` (XDG data directory)
5. `~/.config/kitty/` (kitty standard location)

### Session File Discovery
- Tries exact name first: `<name>`
- If not found and name doesn't end with `.session`, tries `<name>.session`
- Searches all paths for both variants

---

## Next Steps (For Future Development)

### Enhancement Ideas
- [ ] Add `--list-sessions` command to show available sessions
- [ ] Add `--edit-session <name>` to open in $EDITOR
- [ ] Add `--remove-session <name>` to delete sessions
- [ ] Create interactive setup wizard (`--init`)
- [ ] Add fish shell completions
- [ ] Document team collaboration workflows
- [ ] Add environment variable injection per-session
- [ ] Set up GitHub releases page
- [ ] Add shell completion to package repositories
- [ ] Create video tutorials

### Known Limitations
- Currently requires kitty to be in PATH
- Absolute path specification not yet implemented
- No built-in session editor (use $EDITOR manually)

---

## Deployment

### Installation Methods

#### From Debian Package (Recommended)
**AMD64 Systems:**
```bash
sudo dpkg -i kitty-launcher_0.4.1-1_amd64.deb
```

**ARM64 Systems (Raspberry Pi, etc.):**
```bash
sudo dpkg -i kitty-launcher_0.4.1-1_arm64.deb
```

#### From Precompiled Binary
```bash
wget https://github.com/pilakkat1964/kitty-launcher/releases/download/v0.4.1/kitty-launcher-v0.4.1-linux-amd64
chmod +x kitty-launcher-v0.4.1-linux-amd64
sudo cp kitty-launcher-v0.4.1-linux-amd64 /usr/local/bin/kitty-launcher
```

#### From Source
```bash
git clone git@github.com:pilakkat1964/kitty-launcher.git
cd kitty-launcher
cargo build --release
sudo cp target/release/kitty-launcher /usr/local/bin/
```

#### Using Build Scripts
```bash
./scripts/build.sh --release --test
./scripts/build-deb.sh --clean
```

### Release Assets (GitHub)
Each version release includes:
- **Precompiled Binary**: `kitty-launcher-v*.tar.gz` - Source archive
- **Debian Package (AMD64)**: `kitty-launcher_*-1_amd64.deb` - Ready to install
- **Debian Package (ARM64)**: `kitty-launcher_*-1_arm64.deb` - For ARM systems
- **Source Archive**: `kitty-launcher-v*.tar.gz` - Full source code

**Repository**: https://github.com/pilakkat1964/kitty-launcher/releases

---

## Quality Metrics

### Code Quality
- ✅ 0 compiler warnings
- ✅ 0 compiler errors
- ✅ 957 lines of well-documented Rust code
- ✅ 7/7 unit tests passing
- ✅ Proper error handling with Result types
- ✅ Security-focused input validation

### Performance
- Binary compilation: ~1.5 seconds
- Test suite execution: ~0.03 seconds
- Session launch: Immediate (subprocess spawn)
- Shell completion generation: Instant

### Documentation
- README: 614 lines
- Man page: 339 lines
- Info page: 583 lines
- Learning guide: 439 lines
- Installation guide: 245 lines
- **Total: 1,500+ lines**

---

## Checkpoint for Restart

### Current State
- **All features implemented** and working
- **All tests passing** (7/7)
- **All code pushed** to GitHub via SSH
- **All tags created** (v0.1.0 through v0.4.0, v0.4.1 test release)
- **Clean working directory** with no uncommitted changes
- **Repository synchronized** with remote
- **GitHub Actions Workflow**: Multi-architecture builds fully operational
  - ✅ AMD64 builds working (native compilation)
  - ✅ ARM64 builds working (cross-compilation)
  - ✅ Debian package generation for both architectures
  - ✅ Binary release assets included
  - ✅ Release notes with installation instructions

### To Resume Later
1. Navigate to: `/home/sysadmin/workspace/Opencode-workspaces/z-tools/kitty-launcher`
2. Verify status: `git status` (should show "nothing to commit")
3. Check current version: `cargo build --release && ./target/release/kitty-launcher --version`
4. Run tests: `cargo test`
5. SSH+Git is ready: `git push origin master` works without auth prompts

### Files to Review First
- `README.md` - Overview and power-user workflows
- `src/main.rs` - Implementation details
- `AGENTS.md` (this file) - Project status

---

## Recent Session Updates (Priority 2: Build System Unification)

### ✅ Completed: Cargo-Audit Security Scanning (April 16, 2026)

**Changes Applied:**
1. **.github/workflows/build-and-test.yml**: 
   - Added new `security_audit` job
   - Installs `cargo-audit` and runs security vulnerability check
   - Configured to deny warnings (`--deny warnings`)
   - Runs in parallel with build/test jobs
   - Uses `dtolnay/rust-toolchain@stable`

**Impact:**
- Automatic security vulnerability scanning in CI/CD
- Detects known vulnerabilities in Rust dependencies
- Prevents release of packages with known security issues
- Consistent with z-rclone-mount-applete's security setup

**Verification:**
```bash
cargo audit --deny warnings
```

**Related Documentation:**
- See `/z-tools/CI_CD_STANDARDIZATION_GUIDE.md` for standardized security patterns
- See `/z-tools/z-rclone-mount-applete/.github/workflows/ci.yml` for reference implementation

---

## Recent Session Updates (Priority 3: GitHub Pages Deployment)

### ✅ Completed: GitHub Pages Setup & Cross-Project Navigation (April 16, 2026)

**Changes Applied:**

1. **GitHub Pages Infrastructure Setup**
   - Created `docs/_config.yml` with Jekyll configuration and Slate theme
   - Created `docs/_layouts/default.html` custom layout template
   - Added YAML front matter to all documentation files (6 markdown files)
   - Created `.github/workflows/pages.yml` GitHub Actions workflow
   - Created `Gemfile` for Ruby dependencies (github-pages, jekyll-relative-links)

2. **GitHub Actions Workflow Fixes**
   - Fixed deprecated actions:
     - Updated `actions/configure-pages@v3` → `v4`
     - Updated `actions/upload-pages-artifact@v2` → `v3`
     - Updated `actions/deploy-pages@v2` → `v4`
     - Updated `actions/upload-artifact@v3` → `v4` in build-and-test workflow

3. **Cross-Project Navigation**
   - Added "🔗 Related Z-Tools Projects" section to docs/index.md
   - Links to Z-Edit, Z-Open, and RClone Mount Applete
   - Master Index link with absolute URLs for cross-repository compatibility

4. **Testing & Deployment**
   - All GitHub Actions workflows run successfully ✅
   - Pages workflow builds and deploys in ~40 seconds
   - Live site: http://pilakkat.mywire.org/z-kitty-launcher/
   - All documentation pages rendering with Slate theme

**Impact:**
- Professional, auto-deployed documentation site
- Seamless navigation between all z-tools projects
- HTTPS certificate valid until June 23, 2026
- Updated GitHub Actions to latest versions (no deprecation warnings)

**Related Files Modified:**
- `docs/index.md` - Added cross-project navigation, standardized URLs
- `.github/workflows/build-and-test.yml` - Updated artifact action to v4
- `.github/workflows/pages.yml` - NEW, updated action versions
- `docs/_config.yml` - NEW, Jekyll configuration
- `docs/_layouts/default.html` - Already existed, verified working
- `Gemfile` - NEW, GitHub Pages dependencies
- `docs/*.md` - Added YAML front matter to 5 files

**Verification Commands:**
```bash
# Check Pages configuration
gh api repos/pilakkat1964/z-kitty-launcher/pages

# Check workflow status
gh run list --repo pilakkat1964/z-kitty-launcher --limit 5

# Verify site is live
curl -I http://pilakkat.mywire.org/z-kitty-launcher/
```

---

## Recent Session Updates (Priority 5: Crates.io Publishing)

### ✅ Completed: Crates.io Publishing Setup (April 16, 2026)

**Changes Applied:**

1. **Cargo.toml Metadata Standardization**
   - Fixed version duplication: Changed hardcoded `const VERSION: "0.5.1"` to use `env!("CARGO_PKG_VERSION")` macro
   - Updated documentation URL: `https://github.com/kitty-launcher` → `https://docs.rs/kitty-launcher` for automated Rust documentation
   - Added homepage field: `https://github.com/pilakkat1964/z-kitty-launcher`
   - Added keywords: `["terminal", "kitty", "session-manager", "launcher", "tui"]` (5 keywords for discoverability)
   - Added categories: `["command-line-utilities", "development-tools"]` (2 categories for proper classification)

2. **Crates.io Publishing Workflow**
   - Created `.github/workflows/publish-crates.yml` (45 lines)
   - Automatically triggered on git tags matching `v*` pattern (e.g., `v0.5.1`)
   - Uses `dtolnay/rust-toolchain@stable` for reliable Rust setup
   - Runs `cargo package --allow-dirty` for pre-flight verification
   - Uses `cargo publish` with token authentication
   - Includes 15-second delay for Crates.io indexing
   - Verifies publication with confirmation output

3. **Security & Configuration**
   - Requires `CARGO_REGISTRY_TOKEN` GitHub secret (user must configure)
   - Minimal permissions (contents: read only)
   - Uses continue-on-error for graceful handling of duplicate publishes

**Files Modified:**
- `Cargo.toml` - Metadata additions (keywords, categories, homepage, docs.rs URL)
- `src/main.rs` - Version duplication fix (line 33: use env! macro)
- `.github/workflows/publish-crates.yml` (NEW)

**Commits Created:**
- `ae1aab2`: "feat: add Crates.io publishing workflow and standardize metadata"

**Impact:**
- kitty-launcher can now be published to Crates.io automatically on release
- Eliminated version duplication (single source of truth from Cargo.toml)
- Automatic Rust documentation generation on docs.rs
- Professional package metadata with correct classification
- Users can install via: `cargo install kitty-launcher` (once published)

**Next Steps for Crates.io Deployment:**
1. Generate Crates.io API token at https://crates.io/me
2. Add token as `CARGO_REGISTRY_TOKEN` repository secret in GitHub
3. Tag and push release: `git tag v0.5.1 && git push origin v0.5.1`
4. Workflow runs automatically and publishes to Crates.io
5. Verify package appears at https://crates.io/crates/kitty-launcher/

---

## Future Development Priorities

### Priority 6: Enhanced Contribution Guidelines
- Create CONTRIBUTING.md for all projects
- Standardize code review process
- Document development workflow
- Create contributor's guide

### Priority 6: Enhanced Contribution Guidelines
- Create CONTRIBUTING.md for all projects
- Standardize code review process
- Document development workflow
- Create contributor's guide

### Priority 7: Shared Testing Utilities
- Create cross-project test framework
- Implement integration tests
- Set up performance benchmarking
- Create CI/CD testing matrix

### Priority 8: Performance Dashboards
- Track build times across all projects
- Monitor dependency updates
- Display security audit results
- Create GitHub-based metrics dashboard

---
## Developments v +0.1

Some usability improvements:

**Extensibility by parsing the *.sessions files**
- Currently the *.sessions files are entirely processed by the kitty program.
  However it will be useful to have the ability for embedding instructions to
  the launcher itself in the *.sessions file!
- An example scenario is specifying the startup folder for the kitty session.
  As of now there is no way to specify the startup folder in session file;
  the 'cd' command is restricted to use inside a tab only, needing the start up
  folder to be specified multiple times!
- However "kitty" supports command line switch "-d" to change the working 
  directory when launched! This means, if the working directory can be
  passed to the laucher in a way that is transparent to "kitty", the launcher
  could use this to modify the launch command by adding "-d <cwd>" so that
  the session will be launched wiith correct working directory.
- Suggested method : Embedd the commands to launcher in a specially 
  formatted header comment block. The session file uses "#" as the comment 
  delimiter. So commands to be interpreted by the launcher could be identified
  by start token "#%[" and terminated by token "]%#". This will allow the
  session file to remain valid kitty input.
- Please implement the necessary logic to parse the .session files while lauding.
- Initially implement the feature to pass the current working directory. That is
  some thing of the sort #%[ currentWorkingDir = <target directory> ]%# 
- Add the capability to (1) print out the detected working directory to console
  for debugging (2) Invoke the kitty with the option " -d <target directory> "
  so that the kitty gets lauched with the specified folder as the startup 
  directory
- Make sure the parser is flexible enough so hat more functionality can be added.

---
## Contact & Repository

- **GitHub**: https://github.com/pilakkat1964/z-kitty-launcher
- **Owner**: pilakkat1964 (pilakkat1964@gmail.com)
- **SSH Key**: `~/.ssh/id_ed25519_pilakkat` (fingerprint: SHA256:4iiBPkBDBtXoILLYqWTnShh9crw7vxnDhrwX1n7H1hY)
- **Build**: `cargo build --release` in project directory
- **Test**: `cargo test` to verify all tests pass
- **Pages**: http://pilakkat.mywire.org/z-kitty-launcher/

---

**Status Summary**: ✅ Production-ready. GitHub Pages deployed and live. Multi-architecture CI/CD operational. Cargo-audit security scanning enabled. Crates.io publishing configured. Cross-project navigation working. SSH+Git fully operational. Ready for contribution guidelines phase.

**Last Updated**: May 1, 2026 (v0.5.2: session file directive parsing, `cwd` shorthand, `.kitty-session` extension, XDG data dir search path, build-deb.sh fix)

## graphify

This project has a graphify knowledge graph at graphify-out/.

Rules:
- Before answering architecture or codebase questions, read graphify-out/GRAPH_REPORT.md for god nodes and community structure
- If graphify-out/wiki/index.md exists, navigate it instead of reading raw files
- After modifying code files in this session, run `graphify update .` to keep the graph current (AST-only, no API cost)
