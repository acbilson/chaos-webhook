use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs;
use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct ReferenceSet {
    referrer: String,
    sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Syndicated {
    mastodon: String,
}

impl PartialEq for Syndicated {
    fn eq(&self, o: &Self) -> bool {
        self.mastodon == o.mastodon
    }
}

impl Eq for Syndicated {}

#[derive(Serialize, Deserialize, Default, Debug)]
struct FrontMatter {
    author: Option<String>,
    date: Option<String>,
    lastmod: Option<String>,
    epistemic: Option<String>,
    tags: Option<Vec<String>>,
    syndicate: Option<bool>,
    syndicated: Option<Syndicated>,

    #[serde(alias = "in-reply-to")]
    inreplyto: Option<String>,
}

impl PartialEq for FrontMatter {
    fn eq(&self, o: &Self) -> bool {
        self.author == o.author
            && self.date == o.date
            && self.lastmod == o.lastmod
            && self.epistemic == o.epistemic
            && self.tags == o.tags
            && self.syndicate == o.syndicate
            && self.syndicated == o.syndicated
    }
}

impl Eq for FrontMatter {}

#[derive(Debug)]
pub enum FrontMatterError {
    MissingTomlTags,
    NotValidToml(String),
}

struct ParseResults {
    tags_map: HashMap<String, Vec<String>>,
    backrefs_map: HashMap<String, Vec<String>>,
}

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

    let is_markdown = |e: &DirEntry| -> bool {
        !e.file_type().is_dir() & e.file_name().to_string_lossy().ends_with(".md")
    };

    let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    // e.ok() skips files the program can't open (insufficient permissions for example)
    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        if is_markdown(&entry) {
            let file_path = entry.into_path();
            let content = fs::read_to_string(&file_path).expect("content is readable");
            let referrer = file_path.to_str().unwrap().to_string();

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
    return backrefs.join("");
}

fn add_to_tags(frontmatter: &FrontMatter, tags: &mut HashMap<String, Vec<String>>) {
    if let Some(current_tags) = &frontmatter.tags {
        let mut sorted_tags = current_tags.to_vec();
        sorted_tags.sort();
        let mut clone = sorted_tags.clone();

        for t in sorted_tags {
            match tags.entry(String::from(&t)) {
                Entry::Vacant(e) => {
                    clone.retain(|x| *x != t);
                    e.insert(clone.to_vec());
                }
                Entry::Occupied(mut e) => {
                    clone.retain(|x| *x != t);
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
