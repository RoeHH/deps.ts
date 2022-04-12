extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    for file in WalkDir::new("./").into_iter().filter_map(|file| file.ok()) {
        if file.path().extension().unwrap_or_default() == "ts" {
            let file_content = std::fs::read_to_string(file.path()).unwrap();
            let mut imports: Vec<String> = Vec::new();
            for line in file_content.lines() {
                if line.split_whitespace().any(|e| e == "import") {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    if words[0] == "import" && words[1] == ("{") {
                        for import in words.iter().skip(2) {
                            if import == &"}" {
                                break;
                            }
                            imports.push(import.replace(",", ""));
                        }
                    }
                }
            }
            for import in imports {
                println!("{}", import);
            }
        }
    }
}
