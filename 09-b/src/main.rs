extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

lazy_static! {
    static ref MARKER_MATCHER: Regex = Regex::new(r"(?x)
        ^(.*?)        # text before the marker
        (\(\d+x\d+\)) # Marker, eg '(9x9)'
        (.*)$         # text after the marker
    ").unwrap();

    static ref MARKER_DECONSTRUCTER: Regex = Regex::new(r"(?x)
        \(
        (\d+) # repetition length
        x
        (\d+) # repetition times
        \)
    ").unwrap();
}

fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    println!("{}", calc_decompressed_len(&input));
}

fn calc_decompressed_len(compressed_text: &str) -> usize {
    let mut decompressed_length = 0;
    let mut unparsed_text = compressed_text.to_string();

    loop {
        let unparsed_text_clone = unparsed_text.clone();
        let text_captures;

        match MARKER_MATCHER.captures(&unparsed_text_clone) {
            Some(caps) => text_captures = caps,
            None => break,
        }

        let plain_text = &text_captures[1];
        let marker_text = &text_captures[2];
        let post_marker_text = &text_captures[3];

        let marker_captures = MARKER_DECONSTRUCTER.captures(marker_text).unwrap();

        let repeat_length = marker_captures[1].parse::<usize>().unwrap();
        let repeat_times = marker_captures[2].parse::<usize>().unwrap();

        let mut chars = post_marker_text.chars();

        let repeat_text: String = chars.by_ref().take(repeat_length).collect();
        unparsed_text = chars.collect();

        decompressed_length += plain_text.len() + calc_decompressed_len(&repeat_text) * repeat_times;
    }

    decompressed_length + unparsed_text.len()
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}