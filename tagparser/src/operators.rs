use std::collections::HashMap;

use crate::{FrontMatter, FrontMatterError, ReferenceSet};

pub fn hashmap_to_json(source: &HashMap<String, Vec<String>>) -> String {
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

pub fn get_toml_header(content: &str) -> Result<String, FrontMatterError> {
    if !content.contains("+++") {
        return Err(FrontMatterError::MissingTomlTags);
    }
    let first_idx: usize = content.find("+++").unwrap();
    let last_idx: usize = content.rfind("+++").unwrap();

    if first_idx == last_idx {
        return Err(FrontMatterError::MissingTomlTags);
    }
    Ok(String::from(&content[first_idx + 3..last_idx]))
}

pub fn get_frontmatter(content: &str, file_name: &str) -> Result<FrontMatter, FrontMatterError> {
    let header = get_toml_header(content)?;

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
mod operator_tests {
    use crate::operators;
    use crate::FrontMatterError;
    use std::collections::HashMap;

    #[test]
    fn hashmap_to_json_successful_conversion() {
        // arrange
        let map = HashMap::from([(
            String::from("writing"),
            vec![String::from("style"), String::from("pattern")],
        )]);

        // act
        let result = operators::hashmap_to_json(&map);

        // assert
        assert_eq!(
            result,
            String::from("[{\"referrer\":\"writing\",\"sources\":[\"pattern\",\"style\"]}]")
        );
    }

    #[test]
    fn get_toml_header_has_header_success() {
        // arrange
        let content = String::from("+++\nname = \"Alex\"\nage = 34\n+++\nThis is some content\n");

        // act
        let result = operators::get_toml_header(&content);

        // assert
        match result {
            Ok(c) => assert_eq!(c, String::from("\nname = \"Alex\"\nage = 34\n")),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn get_toml_header_missing_tags_error() {
        // arrange
        let content = String::from("name = \"Alex\"\nage = 34\nThis is some content\n");

        // act
        let result = operators::get_toml_header(&content);

        // assert
        match result {
            Ok(_) => panic!("should not return content without tags"),
            Err(e) => assert_eq!(e, FrontMatterError::MissingTomlTags),
        }
    }

    #[test]
    fn get_toml_header_incomplete_tags_error() {
        // arrange
        let content = String::from("+++\nname = \"Alex\"\nage = 34\nThis is some content\n");

        // act
        let result = operators::get_toml_header(&content);

        // assert
        match result {
            Ok(_) => panic!("should not return content with incomplete tags"),
            Err(e) => assert_eq!(e, FrontMatterError::MissingTomlTags),
        }
    }
}
