use clap::Parser;
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "clean-history")]
#[command(about = "Clean multiline entries from shell history files", long_about = None)]
struct Args {
    /// Number of lines threshold - commands with this many lines or more will be removed
    #[arg(short, long, default_value_t = 3)]
    threshold: usize,
}

#[derive(Debug)]
struct ShellConfig {
    name: &'static str,
    history_file: &'static str,
    backup_dir: &'static str,
}

impl ShellConfig {
    const ZSH: Self = Self {
        name: "zsh",
        history_file: ".zsh_history",
        backup_dir: ".zsh_history_backups",
    };

    const BASH: Self = Self {
        name: "bash",
        history_file: ".bash_history",
        backup_dir: ".bash_history_backups",
    };

    const FISH: Self = Self {
        name: "fish",
        history_file: ".local/share/fish/fish_history",
        backup_dir: ".fish_history_backups",
    };

    // Add all supported shells here
    const SUPPORTED_SHELLS: &'static [&'static Self] = &[&Self::ZSH, &Self::BASH, &Self::FISH];

    fn detect() -> Option<&'static Self> {
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
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let shell = ShellConfig::detect()
        .expect("Could not detect shell. Make sure SHELL environment variable is set.");
    println!("Detected shell: {}", shell.name);

    let histfile = get_histfile(shell);
    let backup_dir = get_backup_dir(shell);

    // Create backup directory if it doesn't exist
    fs::create_dir_all(&backup_dir)?;

    // Create backup
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let backup_file = backup_dir.join(format!("{}_history.{}", shell.name, timestamp));
    fs::copy(&histfile, &backup_file)?;
    println!("Backup created: {}", backup_file.display());

    // Read history file
    let file = File::open(&histfile)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    // Clean the history
    let mut output = Vec::new();
    let mut i = 0;
    let mut removed_count = 0;

    while i < lines.len() {
        let line = &lines[i];

        // If line ends with backslash, it's a continuation
        if line.trim_end().ends_with('\\') {
            // Count how many continuation lines follow
            let mut j = i;
            while j < lines.len() && lines[j].trim_end().ends_with('\\') {
                j += 1;
            }
            // Add one more line if it doesn't end with backslash (the final line)
            if j < lines.len() && !lines[j].trim_end().ends_with('\\') {
                j += 1;
            }

            let total_lines = j - i;

            // Keep if below threshold, skip if at or above threshold
            if total_lines < args.threshold {
                for line in lines.iter().take(j).skip(i) {
                    output.push(line.clone());
                }
            } else {
                removed_count += 1;
            }

            i = j;
        } else {
            output.push(line.clone());
            i += 1;
        }
    }

    // Write cleaned history
    let mut file = File::create(&histfile)?;
    for line in &output {
        writeln!(file, "{line}")?;
    }

    println!(
        "Removed {} multiline entries ({}+ lines)",
        removed_count, args.threshold
    );
    println!(
        "Kept {} lines from {} original lines",
        output.len(),
        lines.len()
    );
    println!("History cleaned successfully!");

    Ok(())
}

fn get_histfile(shell: &ShellConfig) -> PathBuf {
    env::var("HOME").map_or_else(
        |_| PathBuf::from(format!("~/{}", shell.history_file)),
        |h| PathBuf::from(h).join(shell.history_file),
    )
}

fn get_backup_dir(shell: &ShellConfig) -> PathBuf {
    env::var("HOME").map_or_else(
        |_| PathBuf::from(format!("~/{}", shell.backup_dir)),
        |h| PathBuf::from(h).join(shell.backup_dir),
    )
}
