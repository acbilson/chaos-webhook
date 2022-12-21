use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs;
use walkdir::WalkDir;

mod models;
mod operators;

use models::{FrontMatter, FrontMatterError, ParseResults, ReferenceSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_dir = &args[1];
    let out_dir = &args[2];

    let results = parse_files(&start_dir);

    let backrefs = convert_to_json(&results.backrefs_map);
    let tags = convert_to_json(&results.tags_map);

    fs::write(format!("{}/tags.json", &out_dir), &tags).expect("writes tag data to tags.json");
    fs::write(format!("{}/backrefs.json", &out_dir), &backrefs)
        .expect("writes backref data to backrefs.json");
}

fn parse_files(start_dir: &str) -> ParseResults {
    // [^\s]+ = puts the first word unbroken by a space into the "src" pattern
    let re = Regex::new(r"\{\{< backref.*src=(?P<src>[^\s]+) >\}\}").unwrap();

    let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    // e.ok() skips files the program can't open (insufficient permissions for example)
    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        if !&entry.file_type().is_dir() && operators::is_markdown(&entry.file_name()) {
            let file_path = entry.into_path();
            let content = fs::read_to_string(&file_path).expect("content is readable");
            let path_str = file_path.to_str().unwrap().to_string();
            let referrer = path_str.trim_start_matches(&start_dir);

            let frontmatter = get_frontmatter(&content, &referrer).expect("parses toml");
            add_to_tags(&frontmatter, &mut tags);
            add_to_backrefs(&re, &referrer, &content, &mut backrefs);
        }
    }
    return ParseResults {
        tags_map: tags,
        backrefs_map: backrefs,
    };
}

fn add_to_backrefs(
    re: &Regex,
    referrer: &str,
    content: &str,
    backrefs: &mut HashMap<String, Vec<String>>,
) {
    // generates backref source HashMap
    for backref_captures in re.captures_iter(&content) {
        // always matches one
        if backref_captures.len() > 1 {
            let reference = format!(
                "{}.md",
                &backref_captures["src"].replace("\"", "").to_string()
            );

            match backrefs.entry(reference) {
                Entry::Vacant(e) => {
                    e.insert(vec![referrer.to_string()]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(referrer.to_string());
                }
            }
        }
    }
}

// TODO: split dedupe from json conversion
fn convert_to_json(source: &HashMap<String, Vec<String>>) -> String {
    let mut backrefs: Vec<String> = Vec::new();
    for (key, value) in source {
        let mut deduped_sources = value.to_vec();
        deduped_sources.sort();
        deduped_sources.dedup();
        let backref = ReferenceSet {
            referrer: key.to_string(),
            sources: deduped_sources,
        };
        let backref_str = match serde_json::to_string(&backref) {
            Ok(b) => b,
            Err(_) => "".to_string(),
        };
        backrefs.push(backref_str.to_string());
    }
    return format!("[{}]", backrefs.join(","));
}

fn add_to_tags(frontmatter: &FrontMatter, tags: &mut HashMap<String, Vec<String>>) {
    if let Some(current_tags) = &frontmatter.tags {
        let mut sorted_tags = current_tags.to_vec();
        sorted_tags.sort();

        for t in &sorted_tags {
            // retain() replaces the vector, so create a new vector for each tag match.
            let mut clone = &mut sorted_tags.to_owned();

            match tags.entry(String::from(t)) {
                Entry::Vacant(e) => {
                    clone.retain(|x| x != t);
                    e.insert(clone.to_vec());
                }
                Entry::Occupied(mut e) => {
                    clone.retain(|x| x != t);
                    e.get_mut().append(&mut clone);
                }
            }
        }
    }
}

fn get_frontmatter(content: &str, file_name: &str) -> Result<FrontMatter, FrontMatterError> {
    let parse_header = |c: &str| -> Result<String, FrontMatterError> {
        if !c.contains("+++") {
            return Err(FrontMatterError::MissingTomlTags);
        }
        let first_idx: usize = c.find("+++").unwrap();
        let last_idx: usize = c.rfind("+++").unwrap();

        if first_idx == last_idx {
            return Err(FrontMatterError::MissingTomlTags);
        }
        Ok(String::from(&content[first_idx + 3..last_idx]))
    };

    let header = parse_header(content)?;

    // converts toml error to my custom error type
    let frontmatter: FrontMatter = toml::from_str(&header).map_err(|e| {
        FrontMatterError::NotValidToml(format!(
            "file {} parse failure: {}",
            file_name,
            e.to_string()
        ))
    })?;

    Ok(frontmatter)
}

#[cfg(test)]
mod main_tests {
    use std::collections::HashMap;

    use crate::add_to_tags;
    use crate::FrontMatter;

    #[test]
    fn adds_frontmatter_to_tags() {
        // arrange
        let fm = FrontMatter {
            author: None,
            date: None,
            lastmod: None,
            epistemic: None,
            syndicate: None,
            syndicated: None,
            inreplyto: None,
            tags: Some(vec![
                String::from("writing"),
                String::from("style"),
                String::from("pattern"),
            ]),
        };

        // act
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        add_to_tags(&fm, &mut tags);

        // assert
        let expected_keys = ["writing", "style", "pattern"];

        let expected_values = HashMap::from([
            ("style", vec!["pattern", "writing"]),
            ("writing", vec!["pattern", "style"]),
            ("pattern", vec!["style", "writing"]),
        ]);

        for &key in &expected_keys {
            // assert key in map
            match tags.get(key) {
                Some(value) => assert_eq!(value, expected_values.get(key).unwrap()),
                None => panic!("key {} not present in tags", key),
            }
        }
    }
}
