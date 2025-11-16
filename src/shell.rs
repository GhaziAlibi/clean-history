use std::env;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct ShellConfig {
    pub name: &'static str,
    pub history_file: &'static str,
    pub backup_dir: &'static str,
    pub reload_cmd: &'static str,
}

impl ShellConfig {
    const ZSH: Self = Self {
        name: "zsh",
        history_file: ".zsh_history",
        backup_dir: ".zsh_history_backups",
        reload_cmd: "fc -R",
    };

    const BASH: Self = Self {
        name: "bash",
        history_file: ".bash_history",
        backup_dir: ".bash_history_backups",
        reload_cmd: "history -r",
    };

    const FISH: Self = Self {
        name: "fish",
        history_file: ".local/share/fish/fish_history",
        backup_dir: ".fish_history_backups",
        reload_cmd: "history --merge",
    };

    // Add all supported shells here
    const SUPPORTED_SHELLS: &'static [&'static Self] = &[&Self::ZSH, &Self::BASH, &Self::FISH];

    /// Detects the current shell from parent process or SHELL environment variable
    pub fn detect() -> Option<&'static Self> {
        // Try to detect from parent process first (more accurate)
        if let Ok(output) = std::process::Command::new("ps")
            .args([
                "-p",
                &std::os::unix::process::parent_id().to_string(),
                "-o",
                "comm=",
            ])
            .output()
        {
            if let Ok(parent) = String::from_utf8(output.stdout) {
                let parent = parent.trim();
                for shell in Self::SUPPORTED_SHELLS {
                    if parent.contains(shell.name) {
                        return Some(shell);
                    }
                }
            }
        }

        // Fallback to SHELL environment variable
        if let Ok(shell_path) = env::var("SHELL") {
            for shell in Self::SUPPORTED_SHELLS {
                if shell_path.contains(shell.name) {
                    return Some(shell);
                }
            }
        }
        None
    }

    /// Gets the full path to the history file
    pub fn get_histfile(&self) -> Result<PathBuf> {
        let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(self.history_file))
    }

    /// Gets the full path to the backup directory
    pub fn get_backup_dir(&self) -> Result<PathBuf> {
        let home = env::var("HOME").map_err(|_| "HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(self.backup_dir))
    }
}
