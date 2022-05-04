extern crate walkdir;
use is_url::is_url;
use walkdir::WalkDir;
use std::fs::File;
use std::io::prelude::*;

struct ImportDeclaration {
    path: String,
    imports: Vec<String>,
}

fn main() {
    let mut import_declarations: Vec<ImportDeclaration> = Vec::new();
    for file in WalkDir::new("./").into_iter().filter_map(|file| file.ok()) {
        if file.path().extension().unwrap_or_default() == "ts" {
            let file_content = std::fs::read_to_string(file.path()).unwrap();
            let mut imports_to_deps: Vec<String> = Vec::new();
            let mut new_file_content = String::new();
            for line in file_content.lines() {
                if line.split_whitespace().any(|e| e == "import") {
                    let mut import_declaration = ImportDeclaration {
                        path: String::new(),
                        imports: Vec::new(),
                    };
                    let words: Vec<&str> = line.split_whitespace().collect();
                    if words[0] == "import" && words[1] == ("{") {
                        let mut search_path = false;
                        for word in words.iter().skip(2) {
                            if !search_path {
                                if word == &"}" {
                                    search_path = true;
                                } else {
                                    import_declaration.imports.push(word.replace(",", ""));
                                }
                            } else {
                                import_declaration.path = word.replace("'", "");
                            }
                        }
                    }
                    if is_url(&import_declaration.path) {
                        for import in &import_declaration.imports {
                            imports_to_deps.push(import.to_string());
                        }
                        import_declarations.push(import_declaration);
                    }else if import_declaration.path.contains("depts.ts") {
                        for import in import_declaration.imports {
                            imports_to_deps.push(import);
                        }
                    }else {
                        new_file_content += line;
                        new_file_content += "\n";
                    }
                }else {
                    new_file_content += line;
                    new_file_content += "\n";
                }

            }
            let mut deps_import = "import {".to_string();
            for import in imports_to_deps {
                deps_import += &import;
                deps_import += ",";
            }
            deps_import += "} from '~/depts.ts';\n";
            new_file_content = deps_import + &new_file_content;
            let mut f = File::create(file.path())
                .expect("Unable to create file");
            f.write_all(new_file_content.as_bytes()).unwrap();
        }
    }
    let mut deps_file_content = "".to_string();
    for import_declaration in import_declarations {
        deps_file_content += "export {\n";
        for import in import_declaration.imports {
            deps_file_content += "  ";
            deps_file_content += &import;
            deps_file_content += ",\n";
        }
        deps_file_content += "} from '";
        deps_file_content += &import_declaration.path;
        deps_file_content += "';\n";

    }
    let mut f = File::create("./depts.ts")
        .expect("Unable to create file");
        f.write_all(deps_file_content.as_bytes()).unwrap();
    println!("Pleas add \"~/\" : \"./\" to your import_map.json");
}
