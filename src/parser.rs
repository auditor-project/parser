use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub signature: String,
    pub filetypes: Vec<String>,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct MatchResult {
    pub id: usize,
    pub file: String,
    pub filetype: String,
    pub search: String,
    pub match_str: String,
    pub hits: String,
    pub line: usize,
}

pub fn find_matches(signatures: Vec<Signature>, directory: &str) -> Vec<MatchResult> {
    let mut matches = Vec::new();
    let mut id_counter = 1;

    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if !signatures
            .iter()
            .any(|s| s.filetypes.contains(&extension.to_string()))
        {
            continue;
        }
        let content = fs::read_to_string(path).unwrap();
        for signature in &signatures {
            if !signature.filetypes.contains(&extension.to_string()) {
                continue;
            }
            let regex = Regex::new(&signature.signature).unwrap();
            for (i, line) in content.lines().enumerate() {
                for capture in regex.captures_iter(line) {
                    let match_str = capture.get(0).unwrap().as_str().to_string();
                    let result = MatchResult {
                        id: id_counter,
                        file: path.to_str().unwrap_or_default().to_string(),
                        filetype: extension.to_string(),
                        search: signature.signature.to_string(),
                        hits: line.to_string(),
                        match_str,
                        line: i + 1,
                    };
                    matches.push(result);
                    id_counter += 1;
                }
            }
        }
    }

    matches
}
