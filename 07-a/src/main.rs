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
        let mut non_hypernet_seqs = Vec::new();
        let mut hypernet_seqs = Vec::new();

        tokenize_ipv7(line, &mut hypernet_seqs, &mut non_hypernet_seqs);

        let non_hypernet_abbas: Vec<(char, char)>;
        let hypernet_abbas: Vec<(char, char)>;

        non_hypernet_abbas = find_abbas_in_sequences(&non_hypernet_seqs);
        hypernet_abbas = find_abbas_in_sequences(&hypernet_seqs);

        if non_hypernet_abbas.len() > 0 && hypernet_abbas.len() == 0 {
            println!("nhns:{:?}\nabbas:{:?}", non_hypernet_seqs, non_hypernet_abbas);
            println!("hns:{:?}\nabbas:{:?}\n", hypernet_seqs, hypernet_abbas);

            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}

fn tokenize_ipv7(ip: &str, hypernet_seqs: &mut Vec<String>, non_hypernet_seqs: &mut Vec<String>) {
    let mut substr_start = 0;
    let mut substr_end;

    for (i, c) in ip.chars().enumerate() {
        match c {
            '[' | ']' => {
                substr_end = i;

                if c == '[' {
                    non_hypernet_seqs.push(ip[substr_start..substr_end].to_string());
                } else {
                    hypernet_seqs.push(ip[substr_start..substr_end].to_string());
                };

                substr_start = i+1;
            }
            _ => ()
        }
    }

    if substr_start != ip.len() {
        non_hypernet_seqs.push(ip[substr_start..].to_string());
    }
}

fn find_abbas_in_sequences(sequences: &Vec<String>) -> Vec<(char, char)> {
    let mut abbas: Vec<(char, char)> = Vec::new();
    
    for sequence in sequences {
        let chars: Vec<char> = sequence.chars().collect();

        for i in 0..chars.len()-3 {
            let slice = &chars[i..i+4];

            if is_abba(slice) {
                abbas.push((slice[0], slice[1]));
            }
        }
    }

    abbas
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