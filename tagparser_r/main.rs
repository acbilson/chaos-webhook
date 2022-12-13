use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use regex::Regex;
use walkdir::WalkDir;
use walkdir::DirEntry;

fn main() {
    get_backrefs();
}

fn get_backrefs() {
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

            for backref_captures in re.captures_iter(&content) {
                // always matches one
                if backref_captures.len() > 1 {
                    let reference = format!("{}.md", &backref_captures["src"].replace("\"", "").to_string());
                    println!("path {:?}", &file_path);
                    println!("src match {:?}", &reference);
                    println!("");

                    let referrer = file_path.to_str().unwrap().to_string();

                    match backrefs.entry(reference) {
                        Entry::Vacant(e) => { e.insert(vec![referrer]); },
                        Entry::Occupied(mut e) => { e.get_mut().push(referrer); },
                    }
                }
            }
        }
    }

    for (key, value) in &backrefs {
        println!("reference: {:?}", key);
        for v in value {
            println!("referrer: {:?}", v);
        }
        println!("");
    }
}
