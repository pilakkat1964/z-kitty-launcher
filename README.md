# Kitty Launcher - Terminal Session Manager for Power Users

A robust, lightning-fast Rust-based terminal session manager for the [kitty terminal emulator](https://sw.kovidgoyal.net/kitty/). Perfect for power users who manage multiple terminal configurations, developers working on diverse projects, and system administrators handling complex workflows.

> **Now with KDE Plasma desktop integration!** Launch custom terminal environments directly from your desktop using folder view with cascading navigation.

## Table of Contents

- [Quick Start](#quick-start)
- [Power User Features](#power-user-features)
- [Use Cases & Workflows](#use-cases--workflows)
- [KDE Plasma Integration](#kde-plasma-integration)
- [Installation](#installation)
- [Shell Completions](#shell-completions)
- [Usage Guide](#usage-guide)
- [Session Directives](#session-directives)
- [Learning Resources](#learning-resources)

## Quick Start

### Prerequisites

- **Rust & Cargo**: Install from [rustup.rs](https://rustup.rs/)
- **Kitty Terminal**: Install from [sw.kovidgoyal.net/kitty](https://sw.kovidgoyal.net/kitty/)

### Build & Install

```bash
cd kitty-launcher
cargo build --release
sudo cp target/release/kitty-launcher /usr/local/bin/
```

### Create Your First Session

```bash
# Create a new terminal session configuration
kitty-launcher -c dev

# Edit it to customize
$EDITOR ~/.local/etc/kitty/sessions/dev.session

# Or launch from project directory to use local ./kitty/sessions/
# Create local session: mkdir -p ./kitty/sessions && kitty-launcher -c mylocal

# Launch it anytime
kitty-launcher dev
```

## Power User Features

### ⚡ Multi-Session Management
- **Instant switching**: Launch any terminal configuration with one command
- **Quick aliases**: Use short flags (`-c`, `-l`) for faster operation
- **Session templates**: Create new sessions from existing templates
- **Zero startup overhead**: Minimal latency compared to manual configuration

### 🎯 Desktop Integration
- **System menu registration**: Add sessions to your desktop application menu
- **Folder view launcher**: Create cascading session menus on your desktop (KDE Plasma)
- **Direct execution**: Launch sessions with a single click from anywhere

### 🛡️ Security & Reliability
- **Input validation**: Prevents path traversal and injection attacks
- **Error recovery**: Clear, actionable error messages
- **Atomic operations**: Session creation is safe and idempotent
- **Exit codes**: Proper status codes for scripting integration

### 📝 Session Configuration
- **Location flexibility**: Sessions stored in standard XDG directories
- **Template system**: Bootstrap new sessions from existing ones
- **Standard format**: Compatible with native kitty session files
- **Multiple search paths**: Find sessions in project, user, or system directories
- **Embedded directives**: Control launcher behaviour from within the session file

### 🔧 Developer-Friendly
- **Shell integration**: Works with bash, zsh, fish, and other shells
- **Scripting support**: Proper exit codes and error messages
- **Lean binary**: ~2.5MB compiled size, minimal dependencies
- **Pure Rust**: No external runtime dependencies

## Use Cases & Workflows

### 📌 Scenario 1: Web Developer with Multiple Environments

**Challenge**: Need quick access to different terminal setups for:
- Frontend development (with Node.js watch mode)
- Backend API server (Python/Django)
- Database/Redis services
- Testing and CI/CD
- Container management

**Solution with Kitty Launcher**:

```bash
# Create sessions for each environment
kitty-launcher -c frontend        # Node.js, npm watch
kitty-launcher -c backend         # Python, Django, DB
kitty-launcher -c devops          # Docker, Kubernetes, CI/CD
kitty-launcher -c testing         # Test runners, debuggers

# Create desktop launchers for quick access
kitty-launcher -l "🔵 Frontend" frontend
kitty-launcher -l "🟢 Backend" backend
kitty-launcher -l "🐳 DevOps" devops
kitty-launcher -l "🧪 Testing" testing

# Use short flags for rapid switching
kitty-launcher frontend    # Launch frontend environment
kitty-launcher backend     # Launch backend environment
```

**Desktop Integration**: Create folder view on KDE Plasma desktop with these launchers. Hover over folder → see cascading menu → click to launch.

### 📌 Scenario 2: System Administrator

**Challenge**: Manage multiple servers, each with specific tool configurations:
- SSH sessions to production/staging/development
- Container orchestration tools
- Log aggregation and monitoring
- Backup and disaster recovery workflows

**Solution with Kitty Launcher**:

```bash
# Create environment-specific sessions
kitty-launcher -c prod-access      # Production SSH, limited tools
kitty-launcher -c staging-debug    # Staging with debugging tools
kitty-launcher -c dev-env          # Development with full tooling
kitty-launcher -c monitoring       # ELK stack, Prometheus, logs
kitty-launcher -c backup-recovery  # Backup tools, recovery scripts

# Quick access for emergencies
kitty-launcher prod-access         # Urgent production fix
kitty-launcher staging-debug       # Staging issue investigation
```

**Workflow**: Pin launcher folder on taskbar → click on situations → access right environment instantly.

### 📌 Scenario 3: Researcher/Data Scientist

**Challenge**: Juggling multiple analysis environments:
- Data exploration with Jupyter/IPython
- Model training (GPU environment)
- Data visualization workspace
- Documentation and writing
- Paper submission and review

**Solution with Kitty Launcher**:

```bash
# Create specialized environments
kitty-launcher -c analysis         # Data exploration, Jupyter
kitty-launcher -c gpu-training     # CUDA environment, model training
kitty-launcher -c visualization    # Plotting libraries, dashboards
kitty-launcher -c documentation    # LaTeX, Markdown, citations
kitty-launcher -c paperwork        # arXiv tools, review comments

# Create themed launchers
kitty-launcher -l "📊 Analysis" analysis
kitty-launcher -l "🎓 Training" gpu-training
kitty-launcher -l "📈 Viz" visualization
```

**Integration**: Organize all on desktop in single folder view. Context-switching is instant.

### 📌 Scenario 4: DevOps/Infrastructure Engineer

**Challenge**: Managing infrastructure across multiple clouds:
- AWS CLI with different credential profiles
- Kubernetes clusters (multiple regions)
- Terraform/IaC environments
- Container registries and deployments
- Secrets and configuration management

**Solution with Kitty Launcher**:

```bash
# Create cloud-specific environments
kitty-launcher -c aws-prod         # AWS production account
kitty-launcher -c aws-staging      # AWS staging account
kitty-launcher -c gcp-prod         # GCP production
kitty-launcher -c k8s-us-east      # Kubernetes US region
kitty-launcher -c k8s-eu-west      # Kubernetes EU region
kitty-launcher -c terraform-live   # IaC deployments

# Create categorical launchers
kitty-launcher -l "☁️  AWS Prod" aws-prod
kitty-launcher -l "☁️  AWS Staging" aws-staging
kitty-launcher -l "🚀 K8s US" k8s-us-east
kitty-launcher -l "🚀 K8s EU" k8s-eu-west
```

**Benefit**: Each terminal opens with correct credentials, region, context pre-configured. Zero chance of deploying to wrong cluster.

### 📌 Scenario 5: Full-Stack Developer Team Lead

**Challenge**: Onboarding new developers with complex multi-repo setups:
- Microservices architecture (10+ services)
- Development database setup
- Cache servers (Redis)
- Message queues (RabbitMQ)
- Search engines (Elasticsearch)

**Solution with Kitty Launcher**:

```bash
# Create starter pack sessions
kitty-launcher -c service-auth     # Auth microservice + deps
kitty-launcher -c service-api      # API service + deps
kitty-launcher -c service-worker   # Background worker + deps
kitty-launcher -c databases        # PostgreSQL, MongoDB, Redis
kitty-launcher -c monitoring       # Logging, tracing, metrics

# Create portable launcher folder
kitty-launcher -l "🔐 Auth" service-auth
kitty-launcher -l "📡 API" service-api
kitty-launcher -l "⚙️  Workers" service-worker
kitty-launcher -l "💾 Data" databases
kitty-launcher -l "📊 Monitor" monitoring

# Add to version control for team
# .local/etc/kitty/launchers/kitty-launcher-*.desktop
# .local/etc/kitty/sessions/*.session
# Or ./kitty/sessions/*.session for project-local sessions
```

**Team Benefit**: New developers clone repo → run scripts to create launchers → have same environment as team lead. Zero setup friction.

## KDE Plasma Integration

### What is Folder View?

**Folder View** is a KDE Plasma feature that displays folder contents directly on your desktop. Combined with cascading menus on folder icons, it creates an intuitive hierarchical navigation system perfect for application launchers.

### Setting Up Desktop Launchers with Folder View

#### Step 1: Create Session Configurations

```bash
# Create your sessions
kitty-launcher -c dev
kitty-launcher -c frontend
kitty-launcher -c backend
kitty-launcher -c tools
```

#### Step 2: Create Desktop Launchers

```bash
# Create launchers for each session
kitty-launcher -l "Development" dev
kitty-launcher -l "Frontend" frontend
kitty-launcher -l "Backend" backend
kitty-launcher -l "Tools" tools
```

Launchers are created in: `~/.local/etc/kitty/launchers/kitty-launcher-*.desktop`

To expose launchers to your application menu, use:
```bash
kitty-launcher --install <LAUNCHER_NAME>
```
This creates a symlink in `~/.local/share/applications/` for system integration.

### Local Project Sessions with Working Directory

You can create project-local sessions by storing them in `./kitty/sessions/` and using the `--path` option to set the working directory for desktop launchers:

```bash
# In your project root, create a local session directory
mkdir -p ./kitty/sessions
echo "new_window
  launch
" > ./kitty/sessions/dev.session

# Create a desktop launcher that starts from the project directory
kitty-launcher -l "Project Dev" dev --path /path/to/project

# Or use relative path (expands with shell)
kitty-launcher -l "Project Dev" dev --path ~/path/to/project
```

When you click this launcher from your desktop:
1. The launcher starts in the specified working directory
2. Kitty-launcher searches for sessions in `./kitty/sessions/` first
3. Finds the local session and launches it
4. Your terminal starts with the project context

This enables graphical launching of project-specific terminal environments.

#### Step 3: Create Folder View on Desktop

1. Right-click on desktop → **Configure Desktop**
2. Click **Add Widget** → **Folder View**
3. Set the folder to: `~/.local/etc/kitty/launchers`
4. Position and resize to preference

#### Step 4: Access Your Terminals

**Desktop View**:
- See all your launcher icons on desktop
- Click any icon → terminal launches with that configuration

**Cascading Menu** (if configured):
- Right-click folder icon → see subfolder menu
- Hover over items → see actions
- Click launcher → instant terminal with your environment

### Creating Organization Structure

For better organization with cascading menus:

```bash
# Create categorical folders in your launchers directory
mkdir -p ~/.local/etc/kitty/launchers/Development
mkdir -p ~/.local/etc/kitty/launchers/DevOps
mkdir -p ~/.local/etc/kitty/launchers/Data-Science

# Create categorized launchers
kitty-launcher -l "Development/Python" python-dev
kitty-launcher -l "Development/Node.js" nodejs-dev
kitty-launcher -l "DevOps/Kubernetes" k8s-prod
kitty-launcher -l "DevOps/AWS" aws-prod
kitty-launcher -l "Data-Science/Analysis" jupyter-analysis
kitty-launcher -l "Data-Science/Training" gpu-training
```

### KDE Plasma Features Leveraged

- ✅ **Folder View Widget**: Desktop integration without installation
- ✅ **Desktop Plasma Folders**: Cascading navigation menus
- ✅ **Application Menu**: Integrated with system launcher
- ✅ **Activity Support**: Different launcher sets per activity
- ✅ **Desktop Searching**: Launchers discoverable via KDE search
- ✅ **Launch Feedback**: Visual indication of terminal launching

### Workflow Optimization

**Before Kitty Launcher**:
1. Open terminal manually
2. Navigate to project directory
3. Set up environment variables
4. Start development server
5. Open second terminal for other tasks
6. Repeat setup steps
7. ... context switching chaos

**With Kitty Launcher + Folder View**:
1. Glance at desktop
2. Click launcher icon
3. Terminal opens, fully configured
4. All environment variables set
5. Ready to work immediately

## Installation

### From Debian Package (Recommended)

**For AMD64 Systems (Intel/AMD 64-bit):**

```bash
wget https://github.com/pilakkat1964/kitty-launcher/releases/download/v0.4.0/kitty-launcher_0.4.0-1_amd64.deb
sudo dpkg -i kitty-launcher_0.4.0-1_amd64.deb
```

**For ARM64 Systems (Raspberry Pi, Apple Silicon, etc.):**

```bash
wget https://github.com/pilakkat1964/kitty-launcher/releases/download/v0.4.0/kitty-launcher_0.4.0-1_arm64.deb
sudo dpkg -i kitty-launcher_0.4.0-1_arm64.deb
```

Debian packages include shell completions (bash/zsh) pre-configured for instant use!

### From Source

```bash
git clone https://github.com/pilakkat1964/kitty-launcher
cd kitty-launcher
cargo build --release
sudo cp target/release/kitty-launcher /usr/local/bin/
```

### From Precompiled Binary

```bash
wget https://github.com/pilakkat1964/kitty-launcher/releases/download/v0.4.0/kitty-launcher-v0.4.0-linux-amd64
chmod +x kitty-launcher-v0.4.0-linux-amd64
sudo cp kitty-launcher-v0.4.0-linux-amd64 /usr/local/bin/kitty-launcher
```

### Verification

```bash
kitty-launcher --version
# kitty-launcher v0.4.0
# A robust Rust wrapper for the kitty terminal emulator
# License: MIT
```

### Shell Completions

Enable tab-completion in your shell to quickly complete session names and commands:

#### Bash Completion

```bash
# Add to ~/.bashrc
kitty-launcher --generate-completions bash >> ~/.bashrc
source ~/.bashrc
```

Or use the automated installer:

```bash
./scripts/install-completions.sh bash
```

#### Zsh Completion

```bash
# Add to ~/.zshrc
kitty-launcher --generate-completions zsh >> ~/.zshrc
source ~/.zshrc
```

Or use the automated installer:

```bash
./scripts/install-completions.sh zsh
```

#### Install All Completions

```bash
./scripts/install-completions.sh both
```

After installation, you can:
- Type `kitty-launcher <TAB>` to see available sessions
- Type `kitty-launcher -<TAB>` to see available flags
- Type `kitty-launcher --create-launcher <TAB>` to select from existing sessions

## Usage Guide

### Basic Commands

```bash
# Launch a session
kitty-launcher dev

# Create a new session
kitty-launcher -c my-project

# Create a desktop launcher
kitty-launcher -l "My Project" my-project

# Display help
kitty-launcher -h

# Show version
kitty-launcher -V
```

### Advanced Usage

```bash
# Create session with specific configuration
kitty-launcher -c work-server
$EDITOR ~/.local/etc/kitty/sessions/work-server.session

# Create multiple launchers from same session
kitty-launcher -l "Primary" work-server
kitty-launcher -l "Secondary" work-server

# Launch from scripts
if kitty-launcher dev; then
    echo "Dev environment launched"
else
    echo "Failed to launch dev environment"
fi
```

### Session Configuration

Sessions are searched in this order (first match wins):
1. `./kitty/sessions/` (project-local, shallowest for easy discovery)
2. `~/.local/etc/kitty/sessions/` (user-level)
3. `/opt/etc/kitty/sessions/` (system-wide optional)
4. `~/.local/share/kitty/sessions/` (XDG data directory)
5. `~/.config/kitty/sessions/` (kitty standard location)

This priority order enables flexible session organization from project-local to system-wide.

Example session file (`dev.session`):

```
# Development environment session
new_window
  cd ~/projects/current-project
  launch

new_window
  cd ~/projects/current-project
  launch npm start

new_window
  cd ~/projects/current-project
  launch git status
```

For full kitty session format, see: https://sw.kovidgoyal.net/kitty/conf/

## Session Directives

Session files can embed **launcher directives** — special comment blocks that are read and acted upon by `kitty-launcher` before the terminal is launched. Because they use the kitty comment character (`#`), they are completely ignored by kitty itself.

### Syntax

```
#%[ key = value ]%#
```

Place directives at the top of your `.session` file (though they are scanned from any line).

### Supported Directives

| Directive | Shorthand | Description |
|-----------|-----------|-------------|
| `currentWorkingDir` | `cwd` | Set kitty's startup working directory (supports `~`) |

### `currentWorkingDir` / `cwd`

Passes `-d <path>` to kitty so the terminal opens in the specified directory. This is especially useful when sessions are shared across projects or launched from `.desktop` files that don't have a `Path=` set.

```
#%[ currentWorkingDir = ~/projects/my-app ]%#
```

Or using the shorthand:

```
#%[ cwd = ~/projects/my-app ]%#
```

Both forms are equivalent and case-insensitive for the key.

**Tilde expansion**: `~` and `~/path` are expanded to `$HOME` automatically.

### Example Session File with Directive

```
#%[ cwd = ~/projects/my-app ]%#

# Development environment session
new_tab Editor
  launch nvim .

new_tab Server
  launch npm run dev

new_tab Git
  launch
```

When launched, kitty starts in `~/projects/my-app` regardless of the working directory of the calling process. The directive is printed to stdout for transparency:

```
Working directory: /home/user/projects/my-app
Session: my-app
Config file: /home/user/.local/etc/kitty/sessions/my-app.session
Status: Launched kitty terminal
```

### Adding More Directives in Future

The parser is designed for easy extension. Unknown directive keys produce a warning on stderr but never fail the launch:

```
Warning: Session file line 2: unknown directive key 'myFutureKey' — ignored
```

## Learning Resources

This project is an excellent Rust learning resource demonstrating real-world patterns while solving practical problems. It's a productive way to learn Rust by building something useful.

### Project Overview

This project demonstrates several important Rust concepts in a practical, real-world context:

- **Error Handling**: Using `Result<T, E>` types for robust error management
- **Input Validation**: Security best practices to validate user input
- **File System Operations**: Reading and validating file paths
- **Process Management**: Spawning external processes
- **Documentation**: Writing clear, beginner-friendly code comments
- **Testing**: Unit tests for critical functions
- **CLI Design**: Building flexible command-line interfaces

### Understanding the Code

#### File Structure

```
kitty-launcher/
├── Cargo.toml          # Project metadata and dependencies
├── src/
│   └── main.rs         # Main application code (well-documented)
├── kitty-launcher.1    # Man page documentation
└── kitty-launcher.info # Info page documentation
```

#### Core Concepts Explained

**1. Structs** (`LauncherConfig`)

```rust
struct LauncherConfig {
    session_name: String,
    config_path: PathBuf,
}
```

A `struct` groups related data together—in this case, the session name and its configuration file path. This is Rust's way of organizing information safely.

**2. Functions with Result Types**

```rust
fn validate_session_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Session name cannot be empty".to_string());
    }
    Ok(())
}
```

`Result<(), String>` means:
- **Success** (`Ok(())`): No error, nothing to return
- **Error** (`Err(String)`): Something failed, return an error message

Rust forces you to handle both cases explicitly—preventing bugs where errors are ignored.

**3. Input Validation & Security**

The `validate_session_name()` function demonstrates security best practices:
- Prevents path traversal attacks (no `../` sequences)
- Rejects special characters that could be misinterpreted
- Only allows safe characters: alphanumeric, hyphens, underscores, dots

This is how you write secure CLI tools in Rust.

**4. File System Operations**

```rust
fn find_config_file(session_name: &str) -> Result<PathBuf, String> {
    // Searches multiple paths, handles errors, returns first match
}
```

This demonstrates safe file handling:
- Checking file existence before use
- Handling missing files gracefully
- Providing helpful error messages

### Rust Learning Resources

#### Concepts Used in This Project

- **Ownership & Borrowing**: [The Rust Book Ch.4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- **Error Handling**: [The Rust Book Ch.9](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- **Structs**: [The Rust Book Ch.5](https://doc.rust-lang.org/book/ch05-00-using-structs.html)
- **Pattern Matching**: [The Rust Book Ch.6](https://doc.rust-lang.org/book/ch06-00-enums.html)
- **Testing**: [The Rust Book Ch.11](https://doc.rust-lang.org/book/ch11-00-testing.html)

#### Official Resources

- **The Rust Book**: https://doc.rust-lang.org/book/ - Start here!
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings - Interactive exercises

### Development & Testing

```bash
# Run tests
cargo test

# The test suite verifies:
# - Valid session names are accepted
# - Invalid session names are rejected
# - Security validation works correctly
# - Desktop file generation is correct
# - Session directive parsing (currentWorkingDir / cwd)
# - Tilde expansion in directive values

# Development commands
cargo build              # Debug build (fast compile, slow runtime)
cargo build --release   # Release build (slow compile, fast runtime)
cargo check             # Check without building
cargo fmt               # Format code
cargo clippy            # Lint for improvements
```

### Common Questions

**Q: Why use `Result` instead of null?**
A: `Result` forces explicit error handling. This prevents silent failures and catches bugs at compile time.

**Q: What's the difference between `String` and `&str`?**
A: `String` owns its data (heap allocated), `&str` is a borrowed reference. Use `&str` for function parameters unless you need to own the data.

**Q: Why does Rust require explicit error handling?**
A: It's a safety feature that prevents bugs. Ignoring errors in Rust is literally impossible—the compiler won't let you.

**Q: How does this demonstrate real-world patterns?**
A: Session management, desktop integration, and configuration management are real problems developers face daily. This project shows how to solve them safely and efficiently.

## Comparison with Similar Tools

| Feature | kitty-launcher | Manual Setup | Other Tools |
|---------|---|---|---|
| Quick session switching | ✅ Single command | ❌ 5+ manual steps | ⚠️ Complex config |
| Desktop integration | ✅ Built-in | ❌ Not available | ⚠️ Plugin-dependent |
| KDE Plasma folder view | ✅ Native | ❌ No | ⚠️ Limited support |
| Short flags for speed | ✅ Yes (-c, -l) | N/A | ⚠️ Long options |
| Session templates | ✅ Included | ❌ Manual | ⚠️ Complex setup |
| Written in Rust | ✅ Fast & safe | N/A | ⚠️ Other languages |
| Searchable in docs | ✅ Excellent | N/A | ⚠️ Poor discoverability |

## Contributing & Feedback

Want to contribute to z-kitty-launcher? We'd love your help! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for:
- How to report bugs and suggest features
- Development environment setup with Rust and Cargo
- Code style guidelines (fmt, clippy, doc comments)
- Testing requirements and best practices
- Pull request process and code review expectations
- Understanding the codebase architecture

This is both a learning project and a practical tool. Contributions welcome!

- **Issues**: Report bugs or request features
- **Pull Requests**: Share improvements
- **Feedback**: Help us document edge cases

## License

MIT License - Free for personal and commercial use

---

**For Power Users**: Kitty Launcher + KDE Plasma folder view = instant terminal context switching  
**For Learners**: Production Rust code demonstrating real patterns and best practices  
**For Teams**: Shareable session configurations for consistent development environments  

**Start managing your terminal sessions efficiently today!** 🚀
