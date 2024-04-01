use std::fs;
use std::io;
use std::io::Write;

fn main() {
    println!("Please enter the directory path:");
    let mut text_input = String::new();
    io::stdin()
        .read_line(&mut text_input)
        .expect("Failed to read line");
    let path_dir = text_input.trim();

    if let Err(err) = remove_empty_directories(path_dir) {
        println!("Error: {}", err);
    } else {
        println!(r"
     _    ____  _                   _           
    / \  / ___|| | ____ _ _ __   __| | ___ _ __ 
   / _ \ \___ \| |/ / _` | '_ \ / _` |/ _ \ '__|
  / ___ \ ___) |   < (_| | | | | (_| |  __/ |   
 /_/   \_\____/|_|\_\__,_|_| |_|\__,_|\___|_|   
                                                
        ");
        println!("Empty directories removed successfully.");
        
    }
}

fn remove_empty_directories(dir_path: &str) -> io::Result<()> {
    visit_dirs(dir_path, &|entry| {
        if entry.path().is_dir() {
            let sub_dir_path = entry.path();
            visit_dirs(sub_dir_path.to_str().unwrap(), &|_| Ok(()))?;
            if is_empty_directory(&sub_dir_path)? {
                print!("Removing directory '{}'... ", sub_dir_path.display());
                io::stdout().flush().unwrap();
                fs::remove_dir(sub_dir_path)?;
                println!("Done");
            }
        }
        Ok(())
    })
}

fn visit_dirs(dir_path: &str, cb: &dyn Fn(fs::DirEntry) -> io::Result<()>) -> io::Result<()> {
    let dir_entries = fs::read_dir(dir_path)?;
    for entry in dir_entries {
        let entry = entry?;
        cb(entry)?;
    }
    Ok(())
}

fn is_empty_directory(dir_path: &std::path::Path) -> io::Result<bool> {
    let mut entries = fs::read_dir(dir_path)?;
    Ok(entries.next().is_none())
}
