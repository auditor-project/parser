extern crate parser;
use clap::{App, Arg};
use parser::parser::{find_matches, Signature};
use std::fs;

fn main() {
    let matches = App::new("Auditor")
        .version("1.0")
        .author("Author <z9fr@protonmail.com>")
        .about("Finds matching signatures in code")
        .arg(
            Arg::with_name("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path to the code to scan")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("sig_file")
                .short('s')
                .long("sig-file")
                .value_name("SIG_FILE")
                .help("Sets the path to the signature file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let sig_file = matches.value_of("sig_file").unwrap();

    // Read signature file
    let yaml_content = fs::read_to_string(sig_file).unwrap();
    let signatures: Vec<Signature> = serde_yaml::from_str(&yaml_content).unwrap();

    // Find matches
    let matches = find_matches(signatures, path);
    println!("{:#?}", matches);
}
