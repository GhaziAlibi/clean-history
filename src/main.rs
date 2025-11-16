mod history;
mod shell;

use clap::Parser;
use shell::ShellConfig;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(name = "clean-history")]
#[command(about = "Clean multiline entries from shell history files", long_about = None)]
struct Args {
    /// Number of lines threshold - commands with this many lines or more will be removed
    #[arg(short, long, default_value_t = 2)]
    threshold: usize,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let shell = ShellConfig::detect()
        .ok_or("Could not detect shell. Make sure SHELL environment variable is set.")?;

    if args.verbose {
        println!("Detected shell: {}", shell.name);
    }

    let histfile = shell.get_histfile()?;
    let backup_dir = shell.get_backup_dir()?;

    // Ensure history file exists before processing
    if !histfile.exists() {
        eprintln!("History file not found: {}", histfile.display());
        std::process::exit(1);
    }

    // Create backup
    let backup_file = history::create_backup(&histfile, &backup_dir, shell.name)?;

    if args.verbose {
        println!("Backup created: {}", backup_file.display());
    }

    // Clean the history
    let stats = history::clean_history(&histfile, args.threshold)?;

    if args.verbose {
        println!(
            "Removed {} multiline entries ({}+ lines)",
            stats.removed_entries, args.threshold
        );
        println!(
            "Kept {} lines from {} original lines",
            stats.kept_lines, stats.original_lines
        );
    }

    println!("\n\x1b[32m✓\x1b[0m History cleaned successfully!");
    println!(
        "\n  To apply in current session, run:  \x1b[1;36m{}\x1b[0m",
        shell.reload_cmd
    );

    Ok(())
}
