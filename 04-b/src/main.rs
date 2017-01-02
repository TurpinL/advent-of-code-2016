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

    let input_lines = input.split("\r\n");
    let match_room_line = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]+)\]$").unwrap();
    let mut sector_id_sum = 0;

    for line in input_lines {
        let groups = match_room_line.captures(line).unwrap();

        let name = groups.at(1).unwrap_or("");
        let sector_id = groups.at(2).unwrap_or("").parse::<u32>().unwrap();
        let checksum = groups.at(3).unwrap_or("");
        let calculated_checksum = calc_checksum(name);

        if checksum == calculated_checksum {
            sector_id_sum += sector_id;

            println!("{}: {}", 
                sector_id,
                decrypt(name, sector_id)
            );
        }
    }
}

fn calc_checksum(name: &str) -> String{
    let letters: [_; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut tally: Vec<(_, _)>= letters.into_iter().map(|c| (c, 0)).collect();

    for c in name.chars() {
        match c {
            'a' ... 'z' => tally[(c as usize) - ('a' as usize)].1 += 1,
            _           => (),
        }
    }

    tally.sort_by(|a, b| 
        if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(b.0)
        }
    );

    tally.iter().take(5).map(|x| *x.0).collect()
}

fn decrypt(cyper_text: &str, shift: u32) -> String{
    let adjusted_shift = (shift % 26) as u8;

    cyper_text.chars()
        .map(|c| {
            match c {
                '-' => ' ',
                'a'...'z' => (((c as u8 - 97 + adjusted_shift) % 26) + 97) as char ,
                _ => c,
            }
        }).collect::<String>()
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}