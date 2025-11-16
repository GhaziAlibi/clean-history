use chrono::Local;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Creates a timestamped backup of the history file
pub fn create_backup(histfile: &Path, backup_dir: &Path, shell_name: &str) -> Result<PathBuf> {
    // Create backup directory if it doesn't exist
    fs::create_dir_all(backup_dir)?;

    // Create backup with timestamp
    let timestamp = Local::now().format("%Y-%m-%d-%H%M%S");
    let backup_file = backup_dir.join(format!("{}_history.{}", shell_name, timestamp));
    fs::copy(histfile, &backup_file)?;

    Ok(backup_file)
}

/// Cleans multiline entries from history based on the threshold
pub fn clean_history(histfile: &Path, threshold: usize) -> Result<CleaningStats> {
    // Read history file
    let file = File::open(histfile)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<std::result::Result<Vec<_>, _>>()?;

    // Clean the history
    let (output, removed_count) = filter_multiline_entries(&lines, threshold);

    // Write cleaned history
    let mut file = File::create(histfile)?;
    for line in &output {
        writeln!(file, "{}", line)?;
    }

    Ok(CleaningStats {
        original_lines: lines.len(),
        kept_lines: output.len(),
        removed_entries: removed_count,
    })
}

/// Filter out multiline entries that exceed the threshold
fn filter_multiline_entries(lines: &[String], threshold: usize) -> (Vec<String>, usize) {
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
            if total_lines < threshold {
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

    (output, removed_count)
}

/// Statistics about the cleaning operation
#[derive(Debug)]
pub struct CleaningStats {
    pub original_lines: usize,
    pub kept_lines: usize,
    pub removed_entries: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_single_line_commands() {
        let lines = vec![
            "echo 'hello'".to_string(),
            "ls -la".to_string(),
            "pwd".to_string(),
        ];
        let (output, removed) = filter_multiline_entries(&lines, 2);
        assert_eq!(output.len(), 3);
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_filter_multiline_below_threshold() {
        let lines = vec![
            "echo 'line 1' \\".to_string(),
            "  'line 2'".to_string(),
            "single line".to_string(),
        ];
        let (output, removed) = filter_multiline_entries(&lines, 3);
        assert_eq!(output.len(), 3);
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_filter_multiline_at_threshold() {
        let lines = vec![
            "echo 'line 1' \\".to_string(),
            "  'line 2'".to_string(),
            "single line".to_string(),
        ];
        let (output, removed) = filter_multiline_entries(&lines, 2);
        assert_eq!(output.len(), 1);
        assert_eq!(removed, 1);
    }

    #[test]
    fn test_filter_multiple_multiline_commands() {
        let lines = vec![
            "echo 'first' \\".to_string(),
            "  'multiline'".to_string(),
            "ls".to_string(),
            "cat file \\".to_string(),
            "  | grep pattern \\".to_string(),
            "  | sort".to_string(),
        ];
        let (output, removed) = filter_multiline_entries(&lines, 2);
        assert_eq!(output.len(), 1); // Only "ls" remains
        assert_eq!(removed, 2);
    }
}
