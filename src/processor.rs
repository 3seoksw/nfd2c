use std::fs;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

pub fn process_file(file_path: &Path, verbose: bool) {
    let path = file_path;
    if !file_path.is_file() {
        eprintln!("No such file found: {:?}", file_path);
        return;
    }

    if let Some(parent) = path.parent() {
        let file_name_str = path.file_name().unwrap();
        let file_name = file_name_str.to_string_lossy();

        let normalized_name: String = file_name.nfc().collect();

        if file_name != normalized_name {
            let new_path = parent.join(&normalized_name);

            if let Err(e) = fs::rename(&path, &new_path) {
                eprintln!("[ ] {:?}\n └─ Error: {}", path, e);
            } else {
                println!("[x] {:?}", new_path);
            }
        } else if verbose {
            println!("[ ] {:?}\n └─ NFC format already satisfied", file_path)
        }
    }
}

pub fn process_directory(dir: &str, recursive: bool, verbose: bool) {
    if recursive {
        for entry in WalkDir::new(dir) {
            match entry {
                Ok(entry) => {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        process_file(&file_path, verbose);
                    } else if verbose {
                        println!("Entering non-file entry: {:?}", file_path);
                    }
                }
                Err(e) => {
                eprintln!("Failed to read directory: {}\n └─ Error: {}", dir, e)
                }
            }
        }
    } else {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path= entry.path();
                        if !file_path.is_file() {
                            continue;
                        }

                        process_file(&file_path, verbose);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory: {}\n └─ Error: {}", dir, e);
                return;
            }
        }
    }
}
