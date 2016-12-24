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
    let mut sides: [[i32; 3]; 3] = [[0; 3]; 3];

    for (i, line) in input_lines.enumerate() {
        let side_strings = line.split_whitespace();

        for (j, side_string) in side_strings.enumerate() {
            if j > 3 {
                panic!("Too many sides!");
            }

            match side_string.parse::<i32>() {
                Err(why) => panic!("Error: {}", why),
                Ok(num) => sides[i%3][j] = num,
            }
        }

        if i % 3 == 2 {
            println!("{:?}", sides);

            for x in 0..3 {
                if validate_triangle(sides[0][x], sides[1][x], sides[2][x]) {
                    valid_count += 1;
                }
            }
        }
    }

    println!("{}", valid_count);
}

fn validate_triangle(side_1: i32, side_2: i32, side_3: i32) -> bool {
    if side_1 + side_2 > side_3 &&
    side_2 + side_3 > side_1 &&
    side_3 + side_1 > side_2 
    {
        true
    } else {
        false
    }
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}