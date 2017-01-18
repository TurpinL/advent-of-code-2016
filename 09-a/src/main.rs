extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let marker_matcher = Regex::new(r"(?x)
        ^(.*?)        # text before the marker
        (\(\d+x\d+\)) # Marker, eg '(9x9)'
        (.*)$         # text after the marker
    ").unwrap();

    let marker_deconstructer = Regex::new(r"(?x)
        \(
        (\d+) # repetition length
        x
        (\d+) # repetition times
        \)
    ").unwrap();

    let mut unparsed_text: String = input;

    loop {
        let unparsed_text_clone = unparsed_text.clone();
        let text_captures;

        match marker_matcher.captures(&unparsed_text_clone) {
            Some(caps) => text_captures = caps,
            None => break,
        }

        let plain_text = &text_captures[1];
        let marker_text = &text_captures[2];
        unparsed_text = text_captures[3].to_string();

        let marker_captures = marker_deconstructer.captures(marker_text).unwrap();

        println!(":{}\n:{}\n:{:?}\n----", plain_text, marker_text, marker_captures);
    }

    println!(":{}", unparsed_text);
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}