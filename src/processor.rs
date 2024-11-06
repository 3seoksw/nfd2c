use std::fs;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

pub fn process_file(file_path: &str, verbose: bool) {
    let path = Path::new(&file_path);
    if !path.is_file() {
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
                eprintln!("[ ] {:?} >> {:?}\n └─ Error: {}", path, new_path, e);
            } else if verbose {
                println!("[x] {:?} >> {:?}", path, new_path);
            }
        } else if verbose {
            println!("[ ] NFC format already satisfied: {:?}", file_path)
        }
    }
}

pub fn process_directory(dir: &str, verbose: bool) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path= entry.path();
                    if !file_path.is_file() {
                        continue;
                    }

                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    process_file(&file_name_str, verbose);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read directory: {}\n └─ Error: {}", dir, e);
            return;
        }
    };
}
