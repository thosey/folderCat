use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} <folder>", args[0])?;
        std::process::exit(1);
    }

    let folder_path = &args[1];
    let path = Path::new(folder_path);
    if path.is_dir() {
        collate_files(&path)?;
    } else {
        writeln!(io::stderr(), "The provided path is not a directory")?;
        std::process::exit(1);
    }

    Ok(())
}

fn collate_files(dir: &Path) -> io::Result<()> {
    let mut stack = vec![dir.to_path_buf()];

    while let Some(current_path) = stack.pop() {
        if current_path.is_dir() {
            for entry in fs::read_dir(&current_path)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if !isHidden(file_name) {
                        stack.push(path);
                    }
                }
            }
        } else {
            output_file_contents(&current_path)?;
        }
    }
    Ok(())
}

fn isHidden(file_name: &OsStr) -> bool {
   file_name.to_string_lossy().starts_with('.')
}

fn output_file_contents(path: &Path) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    let binary_threshold = 0.01;
    let non_utf8_count = contents.iter().filter(|&b| b & 0b1000_0000 != 0).count();
    let is_binary = non_utf8_count as f64 / contents.len() as f64 > binary_threshold;

    let contents = if is_binary {
        String::from("[Binary]")
    } else {
        String::from_utf8_lossy(&contents).to_string()
    };

    println!("File: {}\nContents:\n{}", path.display(), contents);
    Ok(())
}

