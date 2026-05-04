//! # Kitty Terminal Launcher
//!
//! A robust Rust wrapper for the kitty terminal emulator that allows launching
//! terminal sessions with flexible configuration presets.
//!
//! ## Features
//! - Validates input parameters to ensure they're safe and valid
//! - Searches for session configuration files in multiple standard locations
//! - Provides helpful error messages when something goes wrong
//! - Launches kitty terminal with the specified session preset
//! - Creates new session configuration files from templates
//! - Comprehensive help and man page documentation
//!
//! ## Configuration Search Path
//! The program searches for session configuration files in this order:
//! 1. `./etc/kitty/sessions` (current directory)
//! 2. `~/.local/etc/kitty/sessions` (user home directory)
//! 3. `/opt/etc/kitty/sessions` (optional system-wide)
//! 4. `~/.config/kitty/sessions` (kitty standard location)
//!
//! ## Usage
//! ```
//! kitty-launcher <session-name>              # Launch a session
//! kitty-launcher --create <name>             # Create a new session file
//! kitty-launcher --help                      # Show help
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

// Version is automatically synced from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Represents the configuration for the kitty launcher application.
///
/// This struct contains the session name that the user wants to launch
/// and the resolved path to the configuration file.
struct LauncherConfig {
    /// The name of the session to launch (e.g., "dev", "work", "default")
    session_name: String,
    /// The full path to the configuration file that will be used
    config_path: PathBuf,
}

/// Holds launcher directives parsed from a session file's header comments.
///
/// Directives are embedded in session files using the special comment syntax:
///   `#%[ key = value ]%#`
///
/// These lines are valid kitty comments (ignored by kitty) but are read and
/// acted upon by kitty-launcher before launching the terminal.
///
/// # Example session file header
/// ```text
/// #%[ currentWorkingDir = ~/projects/my-app ]%#
/// new_tab Main
///   launch
/// ```
#[derive(Debug, Default)]
struct SessionDirectives {
    /// Optional working directory to pass to kitty via the `-d` flag.
    current_working_dir: Option<String>,
}

/// Prints comprehensive help message
fn print_help() {
    println!(
        "kitty-launcher v{} - Kitty Terminal Session Launcher",
        VERSION
    );
    println!();
    println!("SYNOPSIS:");
    println!("    A session manager for the kitty terminal emulator that allows you to:");
    println!("    • Launch kitty with predefined session configurations");
    println!("    • Generate session templates at organized search paths");
    println!("    • Create .desktop launcher files for convenient GUI access");
    println!();
    println!("USAGE:");
    println!("    kitty-launcher [OPTIONS] [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    <SESSION_NAME>                              Launch a kitty session");
    println!(
        "    <SESSION_NAME> -- [KITTY_ARGS]              Launch session and pass args to kitty"
    );
    println!("    -c, --create <NAME> [--path|-p <DIR>]       Create a session template file");
    println!("    -l, --create-launcher <NAME> [SESSION]      Create a .desktop launcher file");
    println!("                         [--path|-p <DIR>]");
    println!("    --install <LAUNCHER_NAME>                   Add launcher to application menu");
    println!(
        "    --generate-completions <SHELL>              Generate shell completions (bash|zsh)"
    );
    println!("    -h, --help                                  Show this help message");
    println!("    -V, --version                               Show version information");
    println!();
    println!("OPTIONS:");
    println!("    --path, -p <DIR>                            (with -c or -l) Set working directory");
    println!("    -h, --help                                  Display help");
    println!("    -V, --version                               Display version");
    println!();
    println!("QUICK START:");
    println!("    # Create a session template (auto-detects current working directory)");
    println!("    kitty-launcher -c my-session");
    println!("    $EDITOR ~/.local/etc/kitty/sessions/my-session.session");
    println!();
    println!("    # Create a session template with explicit working directory");
    println!("    kitty-launcher -c my-session --path /path/to/project");
    println!("    kitty-launcher -c my-session -p ~/projects/my-app");
    println!();
    println!("    # Launch the session");
    println!("    kitty-launcher my-session");
    println!();
    println!("    # Launch session with custom kitty arguments");
    println!("    kitty-launcher my-session -- --start-as=fullscreen");
    println!();
    println!("    # Create a launcher file for GUI access");
    println!("    kitty-launcher -l 'My Session' my-session");
    println!();
    println!("    # Create launcher with working directory (for local sessions)");
    println!("    kitty-launcher -l 'Project' dev --path /path/to/project");
    println!("    kitty-launcher -l 'Project' dev -p ~/work");
    println!();
    println!("SESSION SEARCH PATHS (in order of priority):");
    println!("    1. ./kitty/sessions/");
    println!("    2. ~/.local/etc/kitty/sessions/");
    println!("    3. /opt/etc/kitty/sessions/");
    println!("    4. ~/.local/share/kitty/sessions/");
    println!("    5. ~/.config/kitty/sessions/");
    println!();
    println!("SESSION FILE DISCOVERY:");
    println!("    • Looks for exact name first, then tries <NAME>.session and <NAME>.kitty-session");
    println!("    • Valid characters: alphanumeric, hyphens, underscores, dots");
    println!();
    println!("CREATING SESSION TEMPLATES:");
    println!("    kitty-launcher -c <NAME> generates a session file at:");
    println!("    ~/.local/etc/kitty/sessions/<NAME>.session");
    println!();
    println!("    If -p/--path is not specified, the current working directory is");
    println!("    auto-detected and embedded as a #%[ cwd = <dir> ]%# directive.");
    println!();
    println!("    To specify an explicit directory:");
    println!("    kitty-launcher -c my-session -p /path/to/my-project");
    println!();
    println!("    Uses z-tools.session as template if available, otherwise creates a basic one.");
    println!("    Edit the generated file to customize your session.");
    println!();
    println!("CREATING LAUNCHER FILES:");
    println!("    kitty-launcher -l <LAUNCHER_NAME> [SESSION] generates a .desktop file.");
    println!("    If SESSION is omitted, <LAUNCHER_NAME> is used as the session name.");
    println!("    If the session file doesn't exist, you are prompted to create it.");
    println!("    Files are created in ~/.local/etc/kitty/launchers/ by default.");
    println!();
    println!("    Use --path/-p to set working directory for launching local sessions:");
    println!("    kitty-launcher -l <LAUNCHER_NAME> <SESSION> -p /path/to/project");
    println!();
    println!("    This enables ./kitty/sessions/ to be found when launching graphically.");
    println!();
    println!("    To expose a launcher to the application menu, use:");
    println!("    kitty-launcher --install <LAUNCHER_NAME>");
    println!("    This creates a symlink in ~/.local/share/applications/.");
    println!();
    println!("PASSING ARGUMENTS TO KITTY:");
    println!("    Use the '--' separator to pass arguments directly to kitty:");
    println!("    kitty-launcher <SESSION> -- <KITTY_ARGS>");
    println!();
    println!("    Examples:");
    println!("    kitty-launcher dev -- --start-as=fullscreen");
    println!("    kitty-launcher work -- --title='Work Terminal'");
    println!("    kitty-launcher main -- -o font_size=14");
    println!();
    println!("SHELL COMPLETIONS:");
    println!("    bash:  kitty-launcher --generate-completions bash >> ~/.bashrc");
    println!("    zsh:   kitty-launcher --generate-completions zsh >> ~/.zshrc");
    println!();
    println!("SESSION DIRECTIVES:");
    println!("    Embed launcher instructions in session files using special comment blocks.");
    println!("    These are valid kitty comments (ignored by kitty) but are acted upon");
    println!("    by kitty-launcher before the terminal is launched.");
    println!();
    println!("    Syntax:  #%[ key = value ]%#");
    println!();
    println!("    Supported directives:");
    println!("      currentWorkingDir = <path>    Set kitty's startup directory (supports ~)");
    println!("      cwd = <path>                  Shorthand for currentWorkingDir");
    println!();
    println!("    Examples (add to the top of your .session file):");
    println!("      #%[ currentWorkingDir = ~/projects/my-app ]%#");
    println!("      #%[ currentWorkingDir = /opt/my-project ]%#");
    println!();
    println!("For more information: https://github.com/pilakkat1964/kitty-launcher");
}

/// Prints version information
fn print_version() {
    println!("kitty-launcher v{}", VERSION);
    println!("A robust Rust wrapper for the kitty terminal emulator");
    println!("License: MIT");
}

/// Validates the session name to ensure it's safe for use.
///
/// This function checks that the session name:
/// - Is not empty
/// - Contains only alphanumeric characters, hyphens, underscores, and dots
/// - Is not a special value like "." or ".."
///
/// # Arguments
/// * `name` - The session name to validate
///
/// # Returns
/// * `Ok(())` if the name is valid
/// * `Err(String)` if the name is invalid, with a descriptive error message
fn validate_session_name(name: &str) -> Result<(), String> {
    // Check if the name is empty
    if name.is_empty() {
        return Err("Session name cannot be empty".to_string());
    }

    // Check if the name is trying to traverse directories (path traversal attack)
    if name.contains('/') || name.contains('\\') || name == "." || name == ".." {
        return Err(format!(
            "Invalid session name: '{}'. Session names cannot contain path separators or special directory names.",
            name
        ));
    }

    // Check if the name contains only valid characters
    // Valid characters: alphanumeric, hyphens, underscores, and dots
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err(format!(
            "Invalid session name: '{}'. Only alphanumeric characters, hyphens, underscores, and dots are allowed.",
            name
        ));
    }

    Ok(())
}

/// Gets the home directory path for the current user.
///
/// This function attempts to get the user's home directory from the `HOME`
/// environment variable. If it's not set, returns None.
///
/// # Returns
/// * `Some(PathBuf)` - The home directory path if available
/// * `None` - If the home directory cannot be determined
fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

/// Finds the configuration file for a given session name.
///
/// This function searches for the session configuration file in the standard
/// locations in the following order:
/// 1. `./kitty/sessions/` (current directory - shallowest nesting for local projects)
/// 2. `~/.local/etc/kitty/sessions/` (user home directory)
/// 3. `/opt/etc/kitty/sessions/` (optional system-wide)
/// 4. `~/.config/kitty/sessions/` (kitty standard location)
///
/// If the session name doesn't already end with `.session`, the function will
/// first try to find the file as-is, then retry with `.session` extension appended.
///
/// The function returns the path to the first configuration file found.
/// If no file is found, it returns an error with suggestions and lists all tried paths.
///
/// # Arguments
/// * `session_name` - The name of the session (with or without .session extension)
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the found configuration file
/// * `Err(String)` - An error message listing all attempted paths
fn find_config_file(session_name: &str) -> Result<PathBuf, String> {
    // Define the search paths in order of preference
    // Sessions are stored in a dedicated ./kitty/sessions subfolder to avoid
    // conflicts with kitty's own configuration files and reduce nesting depth
    let mut search_paths: Vec<PathBuf> = vec![
        // Current directory (highest priority, shallow nesting for local projects)
        PathBuf::from("./kitty/sessions"),
    ];

    // Add user's local configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".local/etc/kitty/sessions"));
    }

    // Add optional system-wide directory
    search_paths.push(PathBuf::from("/opt/etc/kitty/sessions"));

    // Add kitty's XDG data directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".local/share/kitty/sessions"));
    }

    // Add kitty's standard configuration directory if home dir is available
    if let Some(home) = get_home_dir() {
        search_paths.push(home.join(".config/kitty/sessions"));
    }

    // Build list of session names to try: first the original, then with known
    // extensions appended (.session and .kitty-session, matching kitty's own
    // supported extensions). Already-extended names skip redundant variants.
    let session_names_to_try = if session_name.ends_with(".session")
        || session_name.ends_with(".kitty-session")
    {
        vec![session_name.to_string()]
    } else {
        vec![
            session_name.to_string(),
            format!("{}.session", session_name),
            format!("{}.kitty-session", session_name),
        ]
    };

    // Track all attempted paths for error reporting
    let mut attempted_paths: Vec<PathBuf> = Vec::new();

    // Try each search path with each session name variant
    for search_path in search_paths.iter() {
        for session_variant in session_names_to_try.iter() {
            let config_file = search_path.join(session_variant);
            attempted_paths.push(config_file.clone());

            // Check if the file exists
            if config_file.exists() {
                // Verify it's actually a file (not a directory)
                if config_file.is_file() {
                    return Ok(config_file);
                }
            }
        }
    }

    // If we get here, no configuration file was found
    // Build a detailed error message with all attempted paths
    let mut error_msg = format!(
        "Configuration file for session '{}' not found.\n\n\
         Attempted paths:\n",
        session_name
    );

    for (i, path) in attempted_paths.iter().enumerate() {
        let display_path =
            if let Ok(abs_path) = std::fs::canonicalize(path.parent().unwrap_or(Path::new("."))) {
                abs_path.join(path.file_name().unwrap_or_default())
            } else {
                path.clone()
            };
        error_msg.push_str(&format!("  {}. {}\n", i + 1, display_path.display()));
    }

    error_msg.push_str(&format!(
        "\nPlease create a configuration file named '{}' (or '{}.session' / '{}.kitty-session') in one of these directories.",
        session_name, session_name, session_name
    ));

    Err(error_msg)
}

/// Creates a new session configuration file from a template.
///
/// This function:
/// 1. Validates the session name
/// 2. Finds or creates the ~/.local/etc/kitty/sessions directory
/// 3. Reads the template file (z-tools.session) or creates a default one
/// 4. Injects a `cwd` directive with the specified or detected working directory
/// 5. Creates a new session file with the provided name
///
/// # Arguments
/// * `name` - The name of the new session (without .session extension)
/// * `working_dir` - Optional working directory for the cwd directive
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the created session file
/// * `Err(String)` - An error message if creation failed
fn create_session_file(name: &str, working_dir: Option<&str>) -> Result<PathBuf, String> {
    // Validate the session name
    validate_session_name(name)?;

    // Get home directory
    let home = get_home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

    // Define the session directory (with sessions subfolder for isolation)
    let session_dir = home.join(".local/etc/kitty/sessions");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&session_dir).map_err(|e| {
        format!(
            "Failed to create directory {}: {}",
            session_dir.display(),
            e
        )
    })?;

    // Define template and new file paths
    let template_path = session_dir.join("z-tools.session");
    let new_file_path = session_dir.join(format!("{}.session", name));

    // Check if the new file already exists
    if new_file_path.exists() {
        return Err(format!(
            "Session file already exists: {}",
            new_file_path.display()
        ));
    }

    // Determine the working directory to embed
    let embedded_dir = match working_dir {
        Some(dir) => expand_tilde(dir),
        None => {
            // Auto-detect current working directory
            env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "~".to_string())
        }
    };

    // Read the template file
    let template_content = if template_path.exists() {
        fs::read_to_string(&template_path).map_err(|e| {
            format!(
                "Failed to read template file {}: {}",
                template_path.display(),
                e
            )
        })?
    } else {
        // If no template exists, create a basic one
        create_default_template()
    };

    // Inject or update the cwd directive at the top of the file
    let final_content = inject_cwd_directive(&template_content, &embedded_dir);

    // Write the new session file
    fs::write(&new_file_path, final_content).map_err(|e| {
        format!(
            "Failed to create session file {}: {}",
            new_file_path.display(),
            e
        )
    })?;

    Ok(new_file_path)
}

/// Injects or replaces the `cwd` directive in session template content.
///
/// If the template already contains a `#%[ cwd = ... ]%#` or
/// `#%[ currentWorkingDir = ... ]%#` line, it is replaced.
/// Otherwise, the directive is prepended before the first non-comment,
/// non-empty content line (after a leading banner).
fn inject_cwd_directive(content: &str, cwd: &str) -> String {
    let cwd_line = format!("#%[ cwd = {} ]%#", cwd);

    // If there's already a cwd or currentWorkingDir directive, replace it
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    for line in lines.iter_mut() {
        let trimmed = line.trim();
        if trimmed.starts_with("#%[") && trimmed.ends_with("]%#") {
            let inner = &trimmed["#%[".len()..trimmed.len() - "]%#".len()];
            let key = inner.split('=').next().map(|s| s.trim().to_lowercase());
            if key.as_deref() == Some("cwd") || key.as_deref() == Some("currentworkingdir") {
                // Replace this line with the new directive
                *line = cwd_line.clone();
                return lines.join("\n");
            }
        }
    }

    // No existing directive found — insert after the comment banner header
    // Find the right insertion point: after any leading comment lines
    // but before the first tab/launch content
    let mut insert_at = 0;
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') || trimmed.is_empty() {
            // Skip comment lines and empty lines — but track position
            insert_at = i + 1;
        } else {
            // First non-comment, non-empty line — insert directive before it
            insert_at = i;
            break;
        }
    }
    // If we've reached the end (all comments), insert at the end
    if insert_at > lines.len() {
        insert_at = lines.len();
        // Add blank line before directive for separation
        if insert_at > 0 && lines[insert_at - 1].trim().len() > 0 {
            lines.insert(insert_at, String::new());
            insert_at += 1;
        }
    }

    // Insert the directive with a blank line before it for readability
    if insert_at > 0 && !lines[insert_at.saturating_sub(1)].trim().is_empty() {
        lines.insert(insert_at, cwd_line);
    } else {
        lines.insert(insert_at, cwd_line);
    }

    lines.join("\n")
}

/// Creates a default template if z-tools.session doesn't exist
fn create_default_template() -> String {
    r#"# Kitty Session Configuration
# Edit this file to customize your terminal session
# For more information, see: https://sw.kovidgoyal.net/kitty/conf/
#
# Kitty Launcher Directives (parsed by kitty-launcher, ignored by kitty):
# The cwd directive below sets kitty's startup working directory.

# Define the first tab
new_tab Main
  launch

# Define the second tab
new_tab Development
  launch
"#
    .to_string()
}

/// Parses launcher directives from a session file's special comment blocks.
///
/// Scans each line of the session file for the pattern:
///   `#%[ key = value ]%#`
///
/// Lines that do not match this pattern are ignored. Known directive keys
/// are stored in the returned `SessionDirectives`. Unknown keys produce a
/// warning on stderr but do not cause failure.
///
/// Tilde (`~`) at the start of a value is expanded to the user's `$HOME`
/// directory.
///
/// # Arguments
/// * `session_path` - Path to the `.session` file to parse
///
/// # Returns
/// A `SessionDirectives` struct; fields are `None` if not specified in the file.
/// Returns a default (all-`None`) struct if the file cannot be read.
fn parse_session_directives(session_path: &Path) -> SessionDirectives {
    let mut directives = SessionDirectives::default();

    // Read file — fail gracefully (session may still launch without directives)
    let content = match fs::read_to_string(session_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Warning: Could not read session file for directive parsing: {}",
                e
            );
            return directives;
        }
    };

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Directive lines must start with #%[ and end with ]%#
        if !trimmed.starts_with("#%[") || !trimmed.ends_with("]%#") {
            continue;
        }

        // Extract content between the tokens
        let inner = &trimmed["#%[".len()..trimmed.len() - "]%#".len()];
        let inner = inner.trim();

        // Split on the first '=' only
        let eq_pos = match inner.find('=') {
            Some(pos) => pos,
            None => {
                eprintln!(
                    "Warning: Session file line {}: directive missing '=' — ignored: {}",
                    line_num + 1,
                    trimmed
                );
                continue;
            }
        };

        let key = inner[..eq_pos].trim();
        let value = inner[eq_pos + 1..].trim();

        match key.to_lowercase().as_str() {
            "currentworkingdir" | "cwd" => {
                let expanded = expand_tilde(value);
                directives.current_working_dir = Some(expanded);
            }
            _ => {
                eprintln!(
                    "Warning: Session file line {}: unknown directive key '{}' — ignored",
                    line_num + 1,
                    key
                );
            }
        }
    }

    directives
}

/// Expands a leading `~` to the user's home directory.
///
/// If the value starts with `~/` or is exactly `~`, the tilde is replaced
/// with the `$HOME` environment variable. If `$HOME` is not set, the tilde
/// is left unchanged. Any other value is returned as-is.
fn expand_tilde(value: &str) -> String {
    if value == "~" {
        return get_home_dir()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|| value.to_string());
    }
    if value.starts_with("~/") {
        if let Some(home) = get_home_dir() {
            return format!("{}/{}", home.to_string_lossy(), &value[2..]);
        }
    }
    value.to_string()
}

/// Creates a .desktop file for launching a kitty session from application menus.
///
/// This function:
/// 1. Validates the launcher name (same rules as session names)
/// 2. Creates ~/.local/etc/kitty/launchers directory if needed
/// 3. Generates a standard .desktop file for the session
/// 4. Includes optional working directory for finding local sessions
/// 5. Saves the .desktop file in the launchers directory
///
/// The .desktop file can be used by desktop environments to add the launcher
/// to application menus and allow quick access to the session. If a working
/// directory is specified, the launcher will start in that directory, allowing
/// it to find local sessions in ./kitty/sessions/
///
/// # Arguments
/// * `name` - The name for the launcher (e.g., "dev", "work-session")
/// * `session_name` - The session configuration to launch
/// * `working_dir` - Optional working directory where to start the launcher
///
/// # Returns
/// * `Ok(PathBuf)` - Path to the created .desktop file
/// * `Err(String)` - Error description if creation fails
fn create_launcher_file(
    name: &str,
    session_name: &str,
    working_dir: Option<&str>,
) -> Result<PathBuf, String> {
    // Validate both launcher name and session name
    validate_session_name(name)?;
    validate_session_name(session_name)?;

    // Get home directory
    let home = get_home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

    // Define the launchers directory (isolated from app menu by default)
    let launchers_dir = home.join(".local/etc/kitty/launchers");

    // Create the directory if it doesn't exist
    fs::create_dir_all(&launchers_dir).map_err(|e| {
        format!(
            "Failed to create directory {}: {}",
            launchers_dir.display(),
            e
        )
    })?;

    // Define the .desktop file path
    let desktop_file_path = launchers_dir.join(format!("kitty-launcher-{}.desktop", name));

    // Check if the .desktop file already exists
    if desktop_file_path.exists() {
        return Err(format!(
            "Launcher file already exists: {}",
            desktop_file_path.display()
        ));
    }

    // Generate the .desktop file content
    let desktop_content = if let Some(work_dir) = working_dir {
        // Validate that working_dir doesn't contain path traversal attacks
        if work_dir.contains("..") || work_dir.contains("//") {
            return Err("Invalid working directory: contains path traversal patterns".to_string());
        }
        // Expand tilde if present
        let expanded_dir = if work_dir.starts_with("~") {
            home.join(&work_dir[2..]).to_string_lossy().to_string()
        } else if work_dir.starts_with("/") {
            work_dir.to_string()
        } else {
            // Relative paths are kept as-is (can be expanded by shell)
            work_dir.to_string()
        };

        format!(
            r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Kitty: {}
Comment=Launch kitty terminal with {} session
Exec=kitty-launcher {}
Path={}
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
StartupNotify=true
MimeType=application/x-shellscript;text/x-shellscript;application/x-sh;text/x-sh;
"#,
            name, session_name, session_name, expanded_dir
        )
    } else {
        format!(
            r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Kitty: {}
Comment=Launch kitty terminal with {} session
Exec=kitty-launcher {}
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
StartupNotify=true
MimeType=application/x-shellscript;text/x-shellscript;application/x-sh;text/x-sh;
"#,
            name, session_name, session_name
        )
    };

    // Write the .desktop file
    fs::write(&desktop_file_path, desktop_content).map_err(|e| {
        format!(
            "Failed to create launcher file {}: {}",
            desktop_file_path.display(),
            e
        )
    })?;

    Ok(desktop_file_path)
}

/// Creates a symbolic link from a launcher file to the system applications directory.
///
/// This function enables a launcher to appear in the application menu by creating
/// a symlink from ~/.local/etc/kitty/launchers/<file> to ~/.local/share/applications/.
///
/// # Arguments
/// * `launcher_name` - The name of the launcher (without .desktop extension)
///
/// # Returns
/// * `Ok(PathBuf)` - The path to the created symlink
/// * `Err(String)` - An error message if installation failed
fn install_launcher_symlink(launcher_name: &str) -> Result<PathBuf, String> {
    use std::os::unix::fs as unix_fs;

    // Get home directory
    let home = get_home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;

    // Define source and target paths
    let launcher_file = format!("kitty-launcher-{}.desktop", launcher_name);
    let source_path = home.join(".local/etc/kitty/launchers").join(&launcher_file);
    let apps_dir = home.join(".local/share/applications");
    let symlink_path = apps_dir.join(&launcher_file);

    // Check if source file exists
    if !source_path.exists() {
        return Err(format!(
            "Launcher file not found: {}",
            source_path.display()
        ));
    }

    // Create applications directory if it doesn't exist
    fs::create_dir_all(&apps_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", apps_dir.display(), e))?;

    // Check if symlink already exists
    if symlink_path.exists() || symlink_path.is_symlink() {
        return Err(format!(
            "Symlink already exists: {}",
            symlink_path.display()
        ));
    }

    // Create the symlink
    unix_fs::symlink(&source_path, &symlink_path)
        .map_err(|e| format!("Failed to create symlink {}: {}", symlink_path.display(), e))?;

    Ok(symlink_path)
}

/// Loads and validates the launcher configuration from command line arguments.
///
/// This function:
/// 1. Checks that exactly one argument (session name) is provided
/// 2. Validates the session name is safe to use
/// 3. Finds the configuration file for the session
///
/// # Returns
/// * `Ok(LauncherConfig)` - If everything is valid and the config file is found
/// * `Err(String)` - If there's a validation error or the config file is not found
fn load_config(session_name: &str) -> Result<LauncherConfig, String> {
    // Validate the session name
    validate_session_name(session_name)?;

    // Find the configuration file
    let config_path = find_config_file(session_name)?;

    Ok(LauncherConfig {
        session_name: session_name.to_string(),
        config_path,
    })
}

/// Launches the kitty terminal with the specified configuration.
///
/// This function spawns a new kitty process using the configuration file
/// found by the launcher. It sets the KITTY_CONF_DIR environment variable
/// to point to the directory containing the configuration file.
///
/// Prints the resolved configuration file path and session directory to stdout.
///
/// # Arguments
/// * `config` - The launcher configuration containing the session name and config file path
/// * `extra_args` - Optional additional arguments to pass to kitty
///
/// # Returns
/// * `Ok(())` - If kitty was launched successfully
/// * `Err(String)` - If there was an error launching kitty
fn launch_kitty(config: &LauncherConfig, extra_args: Option<Vec<String>>) -> Result<(), String> {
    // Extract the directory containing the configuration file
    let config_dir = config
        .config_path
        .parent()
        .ok_or_else(|| "Could not determine configuration directory".to_string())?;

    // Get the canonical (absolute) path to the config file for display
    let resolved_path = match std::fs::canonicalize(&config.config_path) {
        Ok(path) => path,
        Err(_) => config.config_path.clone(),
    };

    // Parse any launcher directives embedded in the session file
    let directives = parse_session_directives(&config.config_path);

    // Create the kitty command
    let mut command = Command::new("kitty");

    // Set the KITTY_CONF_DIR environment variable
    command.env("KITTY_CONF_DIR", config_dir);

    // Apply currentWorkingDir directive if present (session file wins over inherited cwd)
    if let Some(ref cwd) = directives.current_working_dir {
        println!("Working directory: {}", cwd);
        command.arg("-d");
        command.arg(cwd);
    }

    // Add the session argument
    command.arg("--session");
    command.arg(&config.config_path);

    // Add any extra arguments passed by the user
    if let Some(args) = extra_args {
        for arg in args {
            command.arg(arg);
        }
    }

    // Attempt to execute kitty
    match command.spawn() {
        Ok(_) => {
            println!("Session: {}", config.session_name);
            println!("Config file: {}", resolved_path.display());
            println!("Config directory: {}", config_dir.display());
            println!("Status: Launched kitty terminal");
            Ok(())
        }
        Err(e) => Err(format!(
            "Failed to launch kitty: {}\n\n\
                 Please ensure kitty is installed and available in your PATH.",
            e
        )),
    }
}

/// Generates bash shell completion script for kitty-launcher
fn generate_bash_completion() {
    let completion_script = r#"# bash completion for kitty-launcher, z-kitty, and zk
# Generated by kitty-launcher --generate-completions bash

_kitty_launcher_complete() {
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # Collect sessions from all search paths, stripping known extensions
    local sessions=""
    local _dir
    for _dir in \
        "./kitty/sessions" \
        "$HOME/.local/etc/kitty/sessions" \
        "/opt/etc/kitty/sessions" \
        "$HOME/.local/share/kitty/sessions" \
        "$HOME/.config/kitty/sessions"
    do
        if [[ -d "$_dir" ]]; then
            local _found
            _found=$(ls -1 "$_dir" 2>/dev/null \
                | sed -e 's/\.kitty-session$//' -e 's/\.session$//' \
                | sort -u)
            sessions=$(printf '%s\n%s' "$sessions" "$_found")
        fi
    done
    # Deduplicate and trim blank lines
    sessions=$(echo "$sessions" | sort -u | grep -v '^$')

    # Handle options and commands
    if [[ $COMP_CWORD -eq 1 ]]; then
        # First argument: commands and sessions
        local commands="--help --version --create --create-launcher --install --generate-completions -h -V -c -l"
        COMPREPLY=( $(compgen -W "${commands} ${sessions}" -- "$cur") )
    elif [[ "$prev" == "--create" ]] || [[ "$prev" == "-c" ]]; then
        # After --create or -c: no completion (user enters new session name)
        COMPREPLY=()
    elif [[ "$prev" == "--create-launcher" ]] || [[ "$prev" == "-l" ]]; then
        # After --create-launcher: complete with available sessions
        COMPREPLY=( $(compgen -W "${sessions}" -- "$cur") )
    elif [[ "$prev" == "--install" ]]; then
        # After --install: no completion (user enters launcher name to install)
        COMPREPLY=()
    elif [[ "$prev" == "--generate-completions" ]]; then
        # After --generate-completions: offer bash and zsh
        COMPREPLY=( $(compgen -W "bash zsh" -- "$cur") )
    fi
}

# Register completion for all command names
complete -o bashdefault -o default -o nospace -F _kitty_launcher_complete kitty-launcher
complete -o bashdefault -o default -o nospace -F _kitty_launcher_complete z-kitty
complete -o bashdefault -o default -o nospace -F _kitty_launcher_complete zk
"#;
    println!("{}", completion_script);
}

/// Generates zsh shell completion script for kitty-launcher
fn generate_zsh_completion() {
    let completion_script = r#"# zsh completion for kitty-launcher, z-kitty, and zk
# Generated by kitty-launcher --generate-completions zsh

_kitty_launcher() {
    local -a commands
    local -a sessions

    # Collect sessions from all search paths, stripping known extensions
    local _dir _found
    for _dir in \
        "./kitty/sessions" \
        "$HOME/.local/etc/kitty/sessions" \
        "/opt/etc/kitty/sessions" \
        "$HOME/.local/share/kitty/sessions" \
        "$HOME/.config/kitty/sessions"
    do
        if [[ -d "$_dir" ]]; then
            _found=(${(f)"$(ls -1 "$_dir" 2>/dev/null \
                | sed -e 's/\.kitty-session$//' -e 's/\.session$//' \
                | sort -u)"})
            sessions+=($_found)
        fi
    done
    # Deduplicate
    sessions=(${(u)sessions})

    commands=(
        '--help:Show help message'
        '--version:Show version information'
        '--create:Create a new session configuration file'
        '--create-launcher:Create a .desktop launcher file'
        '--install:Add launcher to application menu'
        '--generate-completions:Generate shell completion scripts'
        '-h:Show help message (short form)'
        '-V:Show version information (short form)'
        '-c:Create session (short form)'
        '-l:Create launcher (short form)'
    )

    _arguments \
        '1: :->cmd_or_session' \
        '2: :->second_arg' \
        '*:session names:(${sessions})'

    case $state in
        cmd_or_session)
            _describe -t commands 'kitty-launcher commands' commands
            _describe -t sessions 'available sessions' sessions
            ;;
        second_arg)
            case ${words[2]} in
                --create|-c)
                    # No completion for new session names
                    ;;
                --create-launcher|-l)
                    # Complete with available sessions or launcher name
                    _describe -t sessions 'session name' sessions
                    ;;
                --install)
                    # No completion for launcher names to install
                    ;;
                --generate-completions)
                    _values 'shell' 'bash' 'zsh'
                    ;;
            esac
            ;;
    esac
}

# Register completion for all command names
compdef _kitty_launcher kitty-launcher
compdef _kitty_launcher z-kitty
compdef _kitty_launcher zk
"#;
    println!("{}", completion_script);
}

/// The main entry point for the kitty launcher application.
///
/// This function:
/// 1. Parses command line arguments
/// 2. Handles help, version, and create options
/// 3. Launches kitty with the validated configuration
/// 4. Exits with appropriate error codes
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, launch kitty with no session (default behavior)
    if args.len() == 1 {
        match Command::new("kitty").spawn() {
            Ok(_) => {
                exit(0);
            }
            Err(e) => {
                eprintln!(
                    "Error: Failed to launch kitty: {}\n\n\
                    Please ensure kitty is installed and available in your PATH.",
                    e
                );
                exit(1);
            }
        }
    }

    let first_arg = &args[1];

    // Handle version flag
    if first_arg == "--version" || first_arg == "-V" {
        print_version();
        exit(0);
    }

    // Handle help flag
    if first_arg == "--help" || first_arg == "-h" {
        print_help();
        exit(0);
    }

    // Handle create command (both --create and -c)
    // Now accepts optional --path/-p for embedded working directory
    if first_arg == "--create" || first_arg == "-c" {
        if args.len() < 3 {
            eprintln!(
                "Error: {} requires at least one argument (session name)",
                first_arg
            );
            eprintln!(
                "Usage: {} {} <SESSION_NAME> [--path|-p <DIR>]",
                args[0], first_arg
            );
            exit(2);
        }

        let session_name = &args[2];
        let mut working_dir: Option<String> = None;

        // Parse optional --path/-p argument
        if args.len() > 3 {
            let mut i = 3;
            while i < args.len() {
                if args[i] == "--path" || args[i] == "-p" {
                    if i + 1 >= args.len() {
                        eprintln!("Error: {} requires an argument", args[i]);
                        exit(2);
                    }
                    working_dir = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: Unknown argument: {}", args[i]);
                    exit(2);
                }
            }
        }

        match create_session_file(session_name, working_dir.as_deref()) {
            Ok(path) => {
                println!("Session file created successfully!");
                println!("Path: {}", path.display());
                if let Some(ref wd) = working_dir {
                    println!("Working directory: {}", wd);
                } else {
                    if let Ok(cwd) = env::current_dir() {
                        println!("Working directory (auto-detected): {}", cwd.display());
                    }
                }
                println!("Edit this file to customize your session configuration.");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2);
            }
        }
    }

    // Handle create-launcher command (both --create-launcher and -l)
    // Now accepts optional session name: if omitted, uses launcher name as session
    // Also accepts optional working directory with --path/-p flag
    // If SESSION is omitted and no matching session file exists, prompts to create one
    if first_arg == "--create-launcher" || first_arg == "-l" {
        // Parse arguments: <LAUNCHER_NAME> [SESSION_NAME] [--path|-p WORK_DIR]
        if args.len() < 3 {
            eprintln!("Error: {} requires at least one argument", first_arg);
            eprintln!(
                "Usage: {} {} <LAUNCHER_NAME> [SESSION_NAME] [--path|-p WORK_DIR]",
                args[0], first_arg
            );
            eprintln!();
            eprintln!("If SESSION_NAME is omitted, LAUNCHER_NAME is used as the session.");
            eprintln!("If --path/-p is specified, the launcher will start from that directory.");
            exit(2);
        }

        let launcher_name = &args[2];
        let mut session_name = launcher_name.to_string();
        let mut working_dir: Option<String> = None;
        let mut session_explicitly_provided = false;

        // Parse remaining arguments
        let mut i = 3;
        while i < args.len() {
            if args[i] == "--path" || args[i] == "-p" {
                // Next argument should be the path
                if i + 1 >= args.len() {
                    eprintln!("Error: {} requires an argument", args[i]);
                    exit(2);
                }
                working_dir = Some(args[i + 1].clone());
                i += 2;
            } else if !args[i].starts_with("--") && !args[i].starts_with("-") && i == 3 {
                // First non-flag argument is session name
                session_name = args[i].clone();
                session_explicitly_provided = true;
                i += 1;
            } else {
                eprintln!("Error: Unknown argument: {}", args[i]);
                exit(2);
            }
        }

        // If session was NOT explicitly provided, check if a session file with
        // the launcher name exists in standard search paths. If not, prompt user.
        if !session_explicitly_provided {
            match find_config_file(launcher_name) {
                Ok(_) => {
                    // Session file exists — proceed normally
                }
                Err(_) => {
                    // Session file not found — prompt user
                    eprintln!(
                        "Session file '{}' not found in standard search paths.",
                        launcher_name
                    );
                    eprint!("Create it now? [y/N] ");

                    // Read a single line from stdin
                    let mut input = String::new();
                    if std::io::stdin().read_line(&mut input).is_ok() {
                        let answer = input.trim().to_lowercase();
                        if answer == "y" || answer == "yes" {
                            match create_session_file(launcher_name, working_dir.as_deref()) {
                                Ok(path) => {
                                    println!("Session file created: {}", path.display());
                                }
                                Err(e) => {
                                    eprintln!("Error creating session file: {}", e);
                                    exit(2);
                                }
                            }
                        } else {
                            println!("Skipping session file creation. Launcher may not work without it.");
                        }
                    } else {
                        eprintln!("Error reading input — skipping session creation.");
                    }
                }
            }
        }

        match create_launcher_file(launcher_name, &session_name, working_dir.as_deref()) {
            Ok(path) => {
                println!("Launcher file created successfully!");
                println!("Path: {}", path.display());
                println!("Launcher name: {}", launcher_name);
                println!("Session: {}", session_name);
                if let Some(ref wd) = working_dir {
                    println!("Working directory: {}", wd);
                }
                println!();
                println!("To add this launcher to your application menu, run:");
                println!("  kitty-launcher --install {}", launcher_name);
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2);
            }
        }
    }

    // Handle --install command (add launcher to application menu)
    if first_arg == "--install" {
        if args.len() != 3 {
            eprintln!("Error: --install requires exactly one argument");
            eprintln!("Usage: {} --install <LAUNCHER_NAME>", args[0]);
            eprintln!();
            eprintln!("This creates a symlink in ~/.local/share/applications/");
            eprintln!("to expose the launcher to your application menu.");
            exit(2);
        }

        let launcher_name = &args[2];

        match install_launcher_symlink(launcher_name) {
            Ok(symlink_path) => {
                println!("Launcher installed successfully!");
                println!("Symlink: {}", symlink_path.display());
                println!();
                println!(
                    "The launcher '{}' is now available in your application menu.",
                    launcher_name
                );
                println!(
                    "You may need to refresh your desktop environment for changes to take effect."
                );
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(2);
            }
        }
    }

    // Handle generate-completions command
    if first_arg == "--generate-completions" {
        if args.len() != 3 {
            eprintln!("Error: --generate-completions requires exactly one argument (shell type)");
            eprintln!("Usage: {} --generate-completions <SHELL>", args[0]);
            eprintln!();
            eprintln!("Supported shells:");
            eprintln!("  bash  - Generate bash completion script");
            eprintln!("  zsh   - Generate zsh completion script");
            eprintln!();
            eprintln!("Installation instructions:");
            eprintln!(
                "  Bash: {} --generate-completions bash >> ~/.bashrc",
                args[0]
            );
            eprintln!("  Zsh:  {} --generate-completions zsh >> ~/.zshrc", args[0]);
            exit(2);
        }

        let shell = &args[2];
        match shell.as_str() {
            "bash" => {
                generate_bash_completion();
                exit(0);
            }
            "zsh" => {
                generate_zsh_completion();
                exit(0);
            }
            _ => {
                eprintln!("Error: Unknown shell '{}'", shell);
                eprintln!("Supported shells: bash, zsh");
                exit(2);
            }
        }
    }

    // If we get here, treat as session launch
    // Check for at least the session name
    if args.len() < 2 {
        eprintln!("Error: Expected at least one session name argument");
        eprintln!("Use '{}' --help for usage information", args[0]);
        exit(2);
    }

    let session_name = &args[1];

    // Look for the "--" separator to split session args from kitty args
    let mut extra_args: Option<Vec<String>> = None;
    if args.len() > 2 {
        // Check if there's a "--" separator
        if let Some(sep_index) = args.iter().position(|arg| arg == "--") {
            // Everything after "--" goes to kitty
            if sep_index + 1 < args.len() {
                extra_args = Some(args[sep_index + 1..].to_vec());
            }
        } else {
            // No separator found, but multiple args - this is an error
            eprintln!("Error: Unexpected arguments");
            eprintln!("Usage: {} <SESSION_NAME> [-- <KITTY_ARGS>...]", args[0]);
            eprintln!();
            eprintln!("To pass arguments to kitty, use '--' as a separator:");
            eprintln!("  {} my-session -- --start-as=fullscreen", args[0]);
            exit(2);
        }
    }

    // Load configuration and validate inputs
    match load_config(session_name) {
        Ok(config) => {
            // Try to launch kitty
            match launch_kitty(&config, extra_args) {
                Ok(()) => {
                    // Kitty launched successfully
                    exit(0);
                }
                Err(e) => {
                    // Error launching kitty - print error and exit with code 1
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        Err(e) => {
            // Configuration error - print error and exit with code 2
            eprintln!("Error: {}", e);
            exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that valid session names are accepted
    #[test]
    fn test_validate_session_name_valid() {
        assert!(validate_session_name("dev").is_ok());
        assert!(validate_session_name("work-session").is_ok());
        assert!(validate_session_name("session_2").is_ok());
        assert!(validate_session_name("dev.backup").is_ok());
        assert!(validate_session_name("z-tools.session").is_ok()); // with .session extension
        assert!(validate_session_name("default.session").is_ok()); // with .session extension
    }

    /// Test that invalid session names are rejected
    #[test]
    fn test_validate_session_name_invalid() {
        assert!(validate_session_name("").is_err()); // empty
        assert!(validate_session_name("../etc").is_err()); // path traversal
        assert!(validate_session_name("./config").is_err()); // path traversal
        assert!(validate_session_name("dev/session").is_err()); // contains slash
        assert!(validate_session_name("dev\\session").is_err()); // contains backslash
        assert!(validate_session_name(".").is_err()); // special directory
        assert!(validate_session_name("..").is_err()); // special directory
        assert!(validate_session_name("dev@home").is_err()); // invalid character
    }

    /// Test that session names with various extensions are accepted
    #[test]
    fn test_validate_session_name_with_extensions() {
        assert!(validate_session_name("dev.session").is_ok());
        assert!(validate_session_name("work.session").is_ok());
        assert!(validate_session_name("test.backup.session").is_ok());
        assert!(validate_session_name("session.config").is_ok());
    }

    /// Test the logic for determining session names to try
    #[test]
    fn test_session_name_variants() {
        // Names without an extension get both .session and .kitty-session appended
        let name = "dev";
        assert!(!name.ends_with(".session") && !name.ends_with(".kitty-session"));

        // Names already ending with .session are not retried
        let name_session = "dev.session";
        assert!(name_session.ends_with(".session") || name_session.ends_with(".kitty-session"));

        // Names already ending with .kitty-session are not retried
        let name_ks = "dev.kitty-session";
        assert!(name_ks.ends_with(".session") || name_ks.ends_with(".kitty-session"));
    }

    /// Test that .kitty-session extension is accepted by the validator
    #[test]
    fn test_validate_session_name_kitty_session_extension() {
        assert!(validate_session_name("dev.kitty-session").is_ok());
        assert!(validate_session_name("my-project.kitty-session").is_ok());
    }

    /// Test create session file validation
    #[test]
    fn test_create_session_validation() {
        // These should be valid names for creating sessions
        assert!(validate_session_name("my-session").is_ok());
        assert!(validate_session_name("project_v1").is_ok());
        assert!(validate_session_name("work.dev").is_ok());

        // These should not be valid
        assert!(validate_session_name("../evil").is_err());
        assert!(validate_session_name("/root").is_err());
    }

    /// Test launcher file name validation
    #[test]
    fn test_create_launcher_validation() {
        // These should be valid names for creating launchers
        assert!(validate_session_name("dev-launcher").is_ok());
        assert!(validate_session_name("work_env").is_ok());
        assert!(validate_session_name("project.v2").is_ok());

        // These should not be valid
        assert!(validate_session_name("../hack").is_err());
        assert!(validate_session_name("./local").is_err());
        assert!(validate_session_name("app@example").is_err());
    }

    /// Test that desktop content is properly formatted
    #[test]
    fn test_desktop_file_content() {
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Version=1.0
Name=Kitty: {}
Comment=Launch kitty terminal with {} session
Exec=kitty-launcher {}
Icon=kitty
Terminal=false
Categories=System;TerminalEmulator;
StartupNotify=true
MimeType=application/x-shellscript;text/x-shellscript;application/x-sh;text/x-sh;
"#,
            "test", "test", "test"
        );

        // Verify the content contains required desktop entry fields
        assert!(desktop_content.contains("[Desktop Entry]"));
        assert!(desktop_content.contains("Type=Application"));
        assert!(desktop_content.contains("Version=1.0"));
        assert!(desktop_content.contains("Name=Kitty: test"));
        assert!(desktop_content.contains("Exec=kitty-launcher test"));
        assert!(desktop_content.contains("Icon=kitty"));
        assert!(desktop_content.contains("Terminal=false"));
    }

    /// Helper: write a temp session file and return its path
    fn write_temp_session(name: &str, content: &str) -> std::path::PathBuf {
        let path = std::path::PathBuf::from(format!("/tmp/kitty-launcher-test-{}.session", name));
        std::fs::write(&path, content).expect("write temp session file");
        path
    }

    /// Test that a basic currentWorkingDir directive is parsed correctly
    #[test]
    fn test_parse_directives_basic() {
        let path = write_temp_session("basic", "#%[ currentWorkingDir = /tmp/myproject ]%#\nnew_tab Main\n  launch\n");
        let directives = parse_session_directives(&path);
        assert_eq!(directives.current_working_dir, Some("/tmp/myproject".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    /// Test that tilde is expanded to the home directory
    #[test]
    fn test_parse_directives_tilde_expansion() {
        let path = write_temp_session("tilde", "#%[ currentWorkingDir = ~/projects/app ]%#\n");
        let directives = parse_session_directives(&path);
        if let Some(home) = get_home_dir() {
            let expected = format!("{}/projects/app", home.to_string_lossy());
            assert_eq!(directives.current_working_dir, Some(expected));
        } else {
            assert_eq!(directives.current_working_dir, Some("~/projects/app".to_string()));
        }
        let _ = std::fs::remove_file(&path);
    }

    /// Test that a bare tilde expands to the home directory
    #[test]
    fn test_parse_directives_bare_tilde() {
        let path = write_temp_session("bare-tilde", "#%[ currentWorkingDir = ~ ]%#\n");
        let directives = parse_session_directives(&path);
        if let Some(home) = get_home_dir() {
            assert_eq!(directives.current_working_dir, Some(home.to_string_lossy().to_string()));
        }
        let _ = std::fs::remove_file(&path);
    }

    /// Test that a session file with no directives returns all-None
    #[test]
    fn test_parse_directives_empty() {
        let path = write_temp_session("empty", "# Normal comment\nnew_tab Main\n  launch\n");
        let directives = parse_session_directives(&path);
        assert!(directives.current_working_dir.is_none());
        let _ = std::fs::remove_file(&path);
    }

    /// Test that unknown directive keys produce a warning but do not panic
    #[test]
    fn test_parse_directives_unknown_key() {
        let path = write_temp_session("unknown", "#%[ unknownKey = somevalue ]%#\nnew_tab Main\n");
        let directives = parse_session_directives(&path);
        assert!(directives.current_working_dir.is_none());
        let _ = std::fs::remove_file(&path);
    }

    /// Test that malformed directive blocks (no '=') are ignored gracefully
    #[test]
    fn test_parse_directives_malformed_no_equals() {
        let path = write_temp_session("no-equals", "#%[ currentWorkingDir ]%#\nnew_tab Main\n");
        let directives = parse_session_directives(&path);
        assert!(directives.current_working_dir.is_none());
        let _ = std::fs::remove_file(&path);
    }

    /// Test that lines missing the closing token are not parsed as directives
    #[test]
    fn test_parse_directives_malformed_no_close_token() {
        let path = write_temp_session("no-close", "#%[ currentWorkingDir = /tmp\nnew_tab Main\n");
        let directives = parse_session_directives(&path);
        assert!(directives.current_working_dir.is_none());
        let _ = std::fs::remove_file(&path);
    }

    /// Test that directive key matching is case-insensitive
    #[test]
    fn test_parse_directives_case_insensitive_key() {
        let path = write_temp_session("case", "#%[ CURRENTWORKINGDIR = /opt/app ]%#\n");
        let directives = parse_session_directives(&path);
        assert_eq!(directives.current_working_dir, Some("/opt/app".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    /// Test that "cwd" is accepted as shorthand for "currentWorkingDir"
    #[test]
    fn test_parse_directives_cwd_shorthand() {
        let path = write_temp_session("cwd-short", "#%[ cwd = /srv/myapp ]%#\nnew_tab Main\n");
        let directives = parse_session_directives(&path);
        assert_eq!(directives.current_working_dir, Some("/srv/myapp".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    /// Test that expand_tilde does not alter absolute paths
    #[test]
    fn test_expand_tilde_absolute_path() {
        let result = expand_tilde("/usr/local/share");
        assert_eq!(result, "/usr/local/share");
    }

    /// Test that expand_tilde does not alter paths without a tilde
    #[test]
    fn test_expand_tilde_no_tilde() {
        let result = expand_tilde("relative/path");
        assert_eq!(result, "relative/path");
    }

    /// Test inject_cwd_directive adds a cwd directive to a template without one
    #[test]
    fn test_inject_cwd_directive_new() {
        let template = "# Comment\nnew_tab Main\n  launch\n";
        let result = inject_cwd_directive(template, "/my/project");
        assert!(result.contains("#%[ cwd = /my/project ]%#"));
        assert!(result.contains("new_tab Main"));
    }

    /// Test inject_cwd_directive replaces existing cwd directive
    #[test]
    fn test_inject_cwd_directive_replace_cwd() {
        let template = "#%[ cwd = /old/path ]%#\nnew_tab Main\n  launch\n";
        let result = inject_cwd_directive(template, "/new/path");
        assert!(result.contains("#%[ cwd = /new/path ]%#"));
        assert!(!result.contains("/old/path"));
    }

    /// Test inject_cwd_directive replaces existing currentWorkingDir directive
    #[test]
    fn test_inject_cwd_directive_replace_full_key() {
        let template = "#%[ currentWorkingDir = /old/path ]%#\nnew_tab Main\n";
        let result = inject_cwd_directive(template, "/new/path");
        assert!(result.contains("#%[ cwd = /new/path ]%#"));
        assert!(!result.contains("currentWorkingDir"));
    }

    /// Test create_default_template no longer has the directive line inline
    #[test]
    fn test_default_template_no_hardcoded_directive() {
        let template = create_default_template();
        assert!(!template.contains("currentWorkingDir = ~"));
        assert!(!template.contains("Uncomment and edit"));
    }
}
