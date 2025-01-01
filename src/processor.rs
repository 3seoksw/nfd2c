use std::fs;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;
use walkdir::WalkDir;

pub fn normalize_name(path: &Path, verbose: bool, is_file: bool) {
    if (is_file && !path.is_file()) || (!is_file && !path.is_dir()) {
        let type_str = if is_file { "file" } else { "directory" };
        eprintln!("No such {} found: {:?}", type_str, path);
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
            println!("[ ] {:?}\n └─ NFC format already satisfied", path)
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
                        normalize_name(&file_path, verbose, true);
                        // process_file(&file_path, verbose);
                    } else if file_path.is_dir() {
                        normalize_name(&file_path, verbose, false);
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
                            normalize_name(&file_path, verbose, false);
                            continue;
                        }

                        normalize_name(&file_path, verbose, true);
                        // process_file(&file_path, verbose);
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
