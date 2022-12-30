use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::{env, fs};
use walkdir::{DirEntry, WalkDir};

mod models;
mod operators;

use models::{FrontMatter, FrontMatterError, ParseResults, ReferenceSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_dir = &args[1];
    let out_dir = &args[2];

    let results = parse_files(&start_dir);

    let backrefs = operators::hashmap_to_json(&results.backrefs_map);
    let tags = operators::hashmap_to_json(&results.tags_map);

    fs::write(format!("{}/tags.json", &out_dir), &tags).expect("writes tag data to tags.json");
    fs::write(format!("{}/backrefs.json", &out_dir), &backrefs)
        .expect("writes backref data to backrefs.json");
}

fn parse_files(start_dir: &str) -> ParseResults {
    let re = Regex::new(r"\{\{< backref.*?src=(?P<src>.*?)\s.*?>\}\}").unwrap();

    let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    let is_markdown = |e: &DirEntry| -> bool {
        !e.file_type().is_dir() && e.file_name().to_string_lossy().ends_with(".md")
    };

    // e.ok() skips files the program can't open (insufficient permissions for example)
    for entry in WalkDir::new(start_dir).into_iter().filter_map(|e| e.ok()) {
        if is_markdown(&entry) {
            let file_path = entry.into_path();
            let content = fs::read_to_string(&file_path).expect("content is readable");
            let path_str = file_path.to_str().unwrap().to_string();
            let referrer = path_str.trim_start_matches(&start_dir);

            let frontmatter = operators::get_frontmatter(&content, &referrer).expect("parses toml");
            add_to_tags(frontmatter.tags, &mut tags);
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

fn add_to_tags(new_tags: Option<Vec<String>>, tags: &mut HashMap<String, Vec<String>>) {
    if let Some(current_tags) = new_tags {
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

#[cfg(test)]
mod main_tests {
    use regex::Regex;
    use std::collections::HashMap;

    use crate::add_to_backrefs;
    use crate::add_to_tags;

    #[test]
    fn add_to_tags_adds_fresh_tags() {
        // arrange
        let new_tags = Some(vec![
            String::from("writing"),
            String::from("style"),
            String::from("pattern"),
        ]);

        // act
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        add_to_tags(new_tags, &mut tags);

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

    #[test]
    fn add_to_tags_adds_multiple_tags() {
        // arrange
        let multiple_tags = [
            Some(vec![String::from("writing"), String::from("organization")]),
            Some(vec![
                String::from("writing"),
                String::from("style"),
                String::from("pattern"),
            ]),
        ];

        // act
        let mut tags: HashMap<String, Vec<String>> = HashMap::new();

        for current_tags in multiple_tags {
            add_to_tags(current_tags, &mut tags);
        }

        // assert
        let expected_keys = ["writing", "organization", "style", "pattern"];

        let expected_values = HashMap::from([
            ("writing", vec!["organization", "pattern", "style"]),
            ("organization", vec!["writing"]),
            ("style", vec!["pattern", "writing"]),
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

    #[test]
    fn add_to_backrefs_adds_all_backrefs() {
        // arrange
        let re = Regex::new(r"\{\{< backref.*?src=(?P<src>.*?)\s.*?>\}\}").unwrap();
        let referrer = String::from("/gardens/faith/what-i-have-learned-about-trust.md");
        let content = String::from("### Cognitive & Affective

Trust can be broadly categorized into cognitive and affective.

Cognitive trust depends on a person's performance over time. If they've shown themselves trustworthy by regularly doing what they've promised, we put our cognitive trust in them.

Affective trust is build on the confidence that we know who a person is at their core. Our trust grows as the person is open and vulnerable with us; whether in a private conversation over drinks at the pub or a raucous game of chinese checkers.

While it's common for American's to expect only cognitive trust in their work relationships, most of the world feels that {{< backref src=\"/plants/leadership/foster-trust-with-both-performance-and-vulnerability\" name=\"both cognitive and affective trust are vital\" >}}. And the sense of {{< backref src=\"/plants/business/employees-need-communication-growth-recognition-and-trust\" name=\"being trusted is crucial for work engagement\" >}}.

Both categories of trust apply to business, but these ideas also resonate in trust of Yahweh. Evangelism in American contexts has leaned heavily towards cognitive trust. Read the Bible, remember Yahweh's works, trust him. Trust built upon Yahweh's track record is {{< backref src=\"/stones/bible/hear-obey-and-live\" name=\"recommended at all times\" >}}, but not to the exclusion of an affective trust. For this reason, helping people discover Jesus must include information about him, but it must also include a real encounter with him, through prayer, healing, prophesy, etc. Perhaps this helps explain why {{< backref src=\"/plants/faith/becoming-a-christian-is-a-process\" >}}?");

        // act
        let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();
        add_to_backrefs(&re, &referrer, &content, &mut backrefs);

        // assert
        let expected_keys = [
            "/plants/leadership/foster-trust-with-both-performance-and-vulnerability.md",
            "/plants/business/employees-need-communication-growth-recognition-and-trust.md",
            "/stones/bible/hear-obey-and-live.md",
            "/plants/faith/becoming-a-christian-is-a-process.md",
        ];

        let expected_values = HashMap::from([
            (
                "/plants/leadership/foster-trust-with-both-performance-and-vulnerability.md",
                vec!["/gardens/faith/what-i-have-learned-about-trust.md"],
            ),
            (
                "/plants/business/employees-need-communication-growth-recognition-and-trust.md",
                vec!["/gardens/faith/what-i-have-learned-about-trust.md"],
            ),
            (
                "/stones/bible/hear-obey-and-live.md",
                vec!["/gardens/faith/what-i-have-learned-about-trust.md"],
            ),
            (
                "/plants/faith/becoming-a-christian-is-a-process.md",
                vec!["/gardens/faith/what-i-have-learned-about-trust.md"],
            ),
        ]);

        for &key in &expected_keys {
            // assert key in map
            match backrefs.get(key) {
                Some(value) => assert_eq!(value, expected_values.get(key).unwrap()),
                None => panic!("key {} not present in backrefs", key),
            }
        }
    }
}
