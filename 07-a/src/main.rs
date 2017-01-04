use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let input_lines = input.split("\r\n");
    let mut num_valid = 0;

    for line in input_lines {
        let mut hypernet_seqs = Vec::new();
        let mut non_hypernet_seqs = Vec::new();

        let mut substr_start = 0;
        let mut substr_end;

        for (i, c) in line.chars().enumerate() {
            match c {
                '[' | ']' => {
                    substr_end = i;

                    if c == '[' {
                        non_hypernet_seqs.push(&line[substr_start..substr_end]);
                    } else {
                        hypernet_seqs.push(&line[substr_start..substr_end]);
                    };

                    substr_start = i+1;
                }
                _ => ()
            }
        }

        if substr_start != line.len() {
            non_hypernet_seqs.push(&line[substr_start..]);
        }

        let mut non_hypernet_abbas: Vec<(char, char)> = Vec::new();
        let mut hypernet_abbas: Vec<(char, char)> = Vec::new();

        for sequence in &non_hypernet_seqs {
            let chars: Vec<char> = sequence.chars().collect();

            for i in 0..chars.len()-3 {
                let slice = &chars[i..i+4];
                
                if is_abba(slice) {
                    non_hypernet_abbas.push((slice[0], slice[1]));
                }
            }
        }

        for sequence in &hypernet_seqs {
            let chars: Vec<char> = sequence.chars().collect();

            for i in 0..chars.len()-3 {
                let slice = &chars[i..i+4];

                if is_abba(slice) {
                    hypernet_abbas.push((slice[0], slice[1]));
                }
            }
        }

        if non_hypernet_abbas.len() > 0 && hypernet_abbas.len() == 0 {
            println!("nhns:{:?}\nabbas:{:?}", non_hypernet_seqs, non_hypernet_abbas);
            println!("hns:{:?}\nabbas:{:?}\n", hypernet_seqs, hypernet_abbas);

            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}

fn is_abba(chars: &[char]) -> bool {
    chars.len() == 4 &&
        chars[0] == chars[3] && 
        chars[1] == chars[2] && 
        chars[0] != chars[1]
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}