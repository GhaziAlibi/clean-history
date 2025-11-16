# Clean History

A simple tool to clean up your command history by removing long multiline commands.

## What does it do?

When you work in the terminal, some commands can be very long and span multiple lines. These can clutter your history and make it hard to find the commands you actually need.

This tool:

- Removes commands that are 3 or more lines long (configurable)
- Keeps shorter commands (1-2 lines)
- Automatically creates a backup before making changes
- Works with any shell (zsh, bash, fish) - automatically detects which one you're using

## Installation

### Option 1: Install from crates.io (Recommended)

```bash
cargo install clean-history
```

### Option 2: Arch Linux (AUR)

```bash
# Using yay
yay -S clean-history

# Using paru
paru -S clean-history

# Or manually
git clone https://aur.archlinux.org/clean-history.git
cd clean-history
makepkg -si
```

### Option 3: Build from source

```bash
# Clone the repository
git clone https://github.com/GhaziAlibi/clean-history.git
cd clean-history

# Build and install
cargo install --path .
```

## Usage

### Basic usage

Simply run the command:

```bash
clean-history
```

### Options

```bash
# Set a custom threshold (remove commands with 5+ lines)
clean-history --threshold 5

# Enable verbose output
clean-history --verbose

# Combine options
clean-history -t 3 -v
```

### Help

```bash
clean-history --help
```

## Where are backups stored?

Backups are saved with a timestamp in a folder in your home directory:

- For zsh: `~/.zsh_history_backups/`
- For bash: `~/.bash_history_backups/`
- For fish: `~/.fish_history_backups/`

You can always restore your history if needed.

## Safety

Don't worry - your original history is always backed up before any changes are made. Each time you run the tool, a new backup is created with the current date and time.
