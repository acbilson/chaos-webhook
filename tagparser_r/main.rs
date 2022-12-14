use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct Backref {
    referrer: String,
    sources: Vec<String>,
}

fn main() {
    let source = get_backref_source();
    let backrefs = convert_to_json(&source);
    println!("{:?}", backrefs);
}

fn get_backref_source() -> HashMap<String, Vec<String>> {
    // [^\s]+ = puts the first word unbroken by a space into the "src" pattern
    let re = Regex::new(r"\{\{< backref.*src=(?P<src>[^\s]+) >\}\}").unwrap();

    let is_markdown = |e: &DirEntry| -> bool {
        !e.file_type().is_dir() & e.file_name().to_string_lossy().ends_with(".md")
    };

    let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();

    // e.ok() skips files the program can't open (insufficient permissions for example)
    for entry in WalkDir::new("data").into_iter().filter_map(|e| e.ok()) {
        if is_markdown(&entry) {
            let file_path = entry.into_path();
            let content = fs::read_to_string(&file_path).expect("content is readable");

            // generates backref source HashMap
            for backref_captures in re.captures_iter(&content) {
                // always matches one
                if backref_captures.len() > 1 {
                    let reference = format!(
                        "{}.md",
                        &backref_captures["src"].replace("\"", "").to_string()
                    );

                    let referrer = file_path.to_str().unwrap().to_string();

                    match backrefs.entry(reference) {
                        Entry::Vacant(e) => {
                            e.insert(vec![referrer]);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(referrer);
                        }
                    }
                }
            }
        }
    }
    return backrefs;
}

fn convert_to_json(source: &HashMap<String, Vec<String>>) -> String {
    let mut backrefs: Vec<String> = Vec::new();
    for (key, value) in source {
        let backref = Backref {
            referrer: key.to_string(),
            sources: value.to_vec(),
        };
        let backref_str = match serde_json::to_string(&backref) {
            Ok(b) => b,
            Err(_) => "".to_string(),
        };
        backrefs.push(backref_str.to_string());
    }
    return backrefs.join("");
}
