extern crate parser;
use parser::parser::{find_matches, Signature};

fn main() {
    let signatures = vec![
        Signature {
            signature: r"\s\$_FILES".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\s\$_GET".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\s\$_POST".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\s\$_REQUEST".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\ssha1\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\ssha1_file\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\sbase64_decode\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\sbase64_encode\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\sbzcompress\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
        Signature {
            signature: r"\sbzdecompress\s*\(".to_string(),
            filetypes: vec!["php".to_string()],
        },
    ];

    let matches = find_matches(
        signatures,
        "/Users/dasith/Developer/projects/auditor/tmp-project",
    );
    println!("{:#?}", matches);
}
