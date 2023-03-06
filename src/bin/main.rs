extern crate parser;
use parser::parser::{find_matches, Signature};
use std::fs;

fn main() {
    let yaml_content = fs::read_to_string("signatures.yaml").unwrap();
    let signatures: Vec<Signature> = serde_yaml::from_str(&yaml_content).unwrap();

    let matches = find_matches(
        signatures,
        "/Users/dasith/Developer/projects/auditor/tmp-project",
    );
    println!("{:#?}", matches);
}
