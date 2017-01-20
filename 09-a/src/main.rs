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
    let mut decompressed_text: String = "".to_string();

    loop {
        let unparsed_text_clone = unparsed_text.clone();
        let text_captures;

        match marker_matcher.captures(&unparsed_text_clone) {
            Some(caps) => text_captures = caps,
            None => break,
        }

        let plain_text = &text_captures[1];
        let marker_text = &text_captures[2];
        let post_marker_text = &text_captures[3];

        let marker_captures = marker_deconstructer.captures(marker_text).unwrap();

        let repeat_length = marker_captures[1].parse::<usize>().unwrap();
        let repeat_times = marker_captures[2].parse::<u32>().unwrap();

        let mut chars = post_marker_text.chars();

        let repeat_text: String = chars.by_ref().take(repeat_length).collect();
        unparsed_text = chars.collect();

        decompressed_text.push_str(plain_text);

        for _ in 0..repeat_times {
            decompressed_text.push_str(&repeat_text);
        }

        // println!(":{}\n:{}\n:{}x{}\n:{}\n----", plain_text, marker_text, repeat_length, repeat_times, repeat_text);
    }

    println!("{}", decompressed_text);
    println!("{}", decompressed_text.len());
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}