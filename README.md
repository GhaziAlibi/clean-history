# Clean History

A simple tool to clean up your command history by removing long multiline commands.

## What does it do?

When you work in the terminal, some commands can be very long and span multiple lines. These can clutter your history and make it hard to find the commands you actually need.

This tool:

- Removes commands that are 3 or more lines long
- Keeps shorter commands (1-2 lines)
- Automatically creates a backup before making changes
- Works with any shell (zsh, bash, fish) - automatically detects which one you're using

## How to use

1. Build the program:

   ```
   cargo build --release
   ```

2. Run it:
   ```
   ./target/release/clean-history
   ```

That's it! Your history will be cleaned and a backup will be saved.

## Where are backups stored?

Backups are saved with a timestamp in a folder in your home directory:

- For zsh: `~/.zsh_history_backups/`
- For bash: `~/.bash_history_backups/`
- For fish: `~/.fish_history_backups/`

You can always restore your history if needed.

## Safety

Don't worry - your original history is always backed up before any changes are made. Each time you run the tool, a new backup is created with the current date and time.
