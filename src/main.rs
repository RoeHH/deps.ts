extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    for file in WalkDir::new("./").into_iter().filter_map(|file| file.ok()) {
        if file.path().extension().unwrap_or_default() == "ts" {
            let file_content = std::fs::read_to_string(file.path()).unwrap();
            for line in file_content.lines() {
                if line.trim((" " "", "\t", "\r", "\n")).contains("import") {

                    println!("{}", line);
                }
            }
        }
    }
}