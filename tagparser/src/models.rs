use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ReferenceSet {
    pub referrer: String,
    pub sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Syndicated {
    pub mastodon: String,
}

impl PartialEq for Syndicated {
    fn eq(&self, o: &Self) -> bool {
        self.mastodon == o.mastodon
    }
}

impl Eq for Syndicated {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FrontMatter {
    pub author: Option<String>,
    pub date: Option<String>,
    pub lastmod: Option<String>,
    pub epistemic: Option<String>,
    pub tags: Option<Vec<String>>,
    pub syndicate: Option<bool>,
    pub syndicated: Option<Syndicated>,

    #[serde(alias = "in-reply-to")]
    pub inreplyto: Option<String>,
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

pub struct ParseResults {
    pub tags_map: HashMap<String, Vec<String>>,
    pub backrefs_map: HashMap<String, Vec<String>>,
}
