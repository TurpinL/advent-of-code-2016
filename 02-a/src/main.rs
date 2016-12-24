use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input: String;
    let keypad = [
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
    ];
    let mut x: usize = 1;
    let mut y: usize = 1;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let input_lines = input.split("\r\n"); 

    for line in input_lines {
        for c in line.chars() {
            match c {
                'U' => y = y.saturating_sub(1),
                'D' => y = y.saturating_add(1),
                'L' => x = x.saturating_sub(1),
                'R' => x = x.saturating_add(1),
                _   => panic!("{} isn't a valid instruction", c),
            }
            x = clamp(x, 0, 2);
            y = clamp(y, 0, 2);
        }
        println!("{:?}", keypad[x][y]);
    }
}

fn clamp<T: Ord>(x: T, min: T, max: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}