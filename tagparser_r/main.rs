use std::fs;
use regex::Regex;

fn main() {
    let entries = fs::read_dir("data").expect("stuff");
    for entry in entries {
        let path = entry.expect("other stuff").path();
        //println!("{:?}", path);
        let content = fs::read_to_string(path).expect("content inaccessible");
        let re = Regex::new(r"< backref src=(?P<src>[^']+) >").unwrap();
        for found in re.captures_iter(&content) {
            println!("{:?}", &found["src"]);
        }
    }
}
