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
    let mut valid_count = 0;

    for line in input_lines {
        let mut side_strings = line.split_whitespace();
        let mut sides: [i32; 3] = [0; 3];

        for (i, side_string) in side_strings.enumerate() {
            if i > 3 {
                panic!("Too many sides!");
            }

            match side_string.parse::<i32>() {
                Err(why) => panic!("Error: {}", why),
                Ok(num) => sides[i] = num,
            }
        }

        println!("{:?}", sides);

        if sides[0] + sides[1] > sides[2] &&
           sides[1] + sides[2] > sides[0] &&
           sides[2] + sides[0] > sides[1] 
        {
            valid_count = valid_count + 1;
        }
    }

    println!("{}", valid_count);
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}