use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input: String;
    let keypad = [
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', '5', ' ', ' ', ' '],
        [' ', ' ', '2', '6', 'A', ' ', ' '],
        [' ', '1', '3', '7', 'B', 'D', ' '],
        [' ', ' ', '4', '8', 'C', ' ', ' '],
        [' ', ' ', ' ', '9', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ];
    let mut x: usize = 3;
    let mut y: usize = 1;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let input_lines = input.split("\r\n"); 

    for line in input_lines {
        for c in line.chars() {
            let (new_x, new_y) = match c {
                'U' => (x, y.saturating_sub(1)),
                'D' => (x, y.saturating_add(1)),
                'L' => (x.saturating_sub(1), y),
                'R' => (x.saturating_add(1), y),
                _   => panic!("{} isn't a valid instruction", c),
            };

            if keypad[new_x][new_y] != ' ' {
                x = new_x;
                y = new_y;
            }
        }
        println!("{:?}", keypad[x][y]);
    }
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}