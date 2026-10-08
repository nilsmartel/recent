use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs, usize};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
#[structopt(name = "recent", about = "A tool to find most recently modified files")]
struct Options {
    /// Number of files to show
    #[structopt(short)]
    n: Option<usize>,

    #[structopt(short, long, default_value = ".")]
    dir: String,
    // Sort by creation time instead of modification time
    // #[structopt(short, long)]
    // created: bool,
}

/// Get all files recursively from a directory and sort by modification time
fn get_files_by_modification_time(
    dir: &str,
) -> Result<Vec<(SystemTime, PathBuf)>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    // Walk the directory recursively
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Only include files, not directories
        if path.is_file() {
            // Get the modification time
            let metadata = fs::metadata(path)?;
            let modified = metadata.modified()?;

            files.push((modified, path.to_path_buf()));
        }
    }

    // Sort by modification time (most recent first)
    files.sort_unstable();
    files.reverse();

    Ok(files)
}

fn main() {
    let Options { n, dir } = Options::from_args();
    let n = n.unwrap_or(usize::MAX);
    let files = get_files_by_modification_time(&dir).expect("get files");

    for (_t, f) in files.into_iter().take(n) {
        println!("{}", f.display());
    }
}
