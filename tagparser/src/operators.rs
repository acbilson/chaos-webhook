use std::collections::HashMap;

use crate::ReferenceSet;

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

#[cfg(test)]
mod operator_tests {
    use crate::operators;
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
}
