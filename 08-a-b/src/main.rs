use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

struct Screen {
    pixels: [[bool; SCREEN_HEIGHT]; SCREEN_WIDTH],
}

impl Screen {
    fn new() -> Screen {
        Screen { pixels: [[false; SCREEN_HEIGHT]; SCREEN_WIDTH], }
    }

    fn rect(&mut self, width: usize, height: usize) {
        if width >= SCREEN_WIDTH || height >= SCREEN_HEIGHT {
            panic!("Index out of bounds! [width:{}, height:{}]", width, height);
        }

        for x in 0..width {
            for y in 0..height {
                self.pixels[x][y] = true;
            }
        }
    }

    fn rotate_column(&mut self, column: usize, rotation: usize) {
        if column >= SCREEN_WIDTH {
            panic!("Index out of bounds! column:{}", column);
        }

        let mut new_column: [bool; SCREEN_HEIGHT] = [false; SCREEN_HEIGHT];

        for y in 0..SCREEN_HEIGHT {
            new_column[y] = self.pixels[column][rotate_index(y, rotation, SCREEN_HEIGHT)];
        }

        self.pixels[column] = new_column;
    }

    fn rotate_row(&mut self, row: usize, rotation: usize) {
        if row >= SCREEN_HEIGHT {
            panic!("Index out of bounds! row:{}", row);
        }

        let mut old_row: [bool; SCREEN_WIDTH] = [false; SCREEN_WIDTH];

        for x in 0..SCREEN_WIDTH {
            old_row[x] = self.pixels[x][row];
        }

        for x in 0..SCREEN_WIDTH {
            self.pixels[x][row] = old_row[rotate_index(x, rotation, SCREEN_WIDTH)];
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                try!(write!(f, "{}", if self.pixels[x][y] {'#'} else {'.'} ));
            }
            try!(write!(f, "\n"));
        }

        Ok(())
    }
}

fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let input_lines = input.split("\r\n");
    let mut screen = Screen::new();

    for line in input_lines {
        println!("\n{:?}", line.split(' ').collect::<Vec<_>>());

        let mut tokens = line.split(' ');

        match tokens.next().unwrap() {
            "rect" => {
                let parameters = tokens.next().unwrap().split('x').collect::<Vec<_>>();
                let x = parameters[0].parse::<usize>().unwrap();
                let y = parameters[1].parse::<usize>().unwrap();
                println!("rect(x:{:?}, y:{:?})", x, y);
                screen.rect(x, y);
            },
            "rotate" => {
                match tokens.next().unwrap() {
                    "column" => {
                        let column = tokens.next().unwrap().split_at(2).1.parse::<usize>().unwrap();
                        let rotation = tokens.skip(1).next().unwrap().parse::<usize>().unwrap();
                        println!("rotate_column(column:{:?}, rotation:{:?})", column, rotation);
                        screen.rotate_column(column, rotation);
                    }
                    "row" => {
                        let row = tokens.next().unwrap().split_at(2).1.parse::<usize>().unwrap();
                        let rotation = tokens.skip(1).next().unwrap().parse::<usize>().unwrap();
                        println!("rotate_row(row:{:?}, rotation:{:?})", row, rotation);
                        screen.rotate_row(row, rotation);
                    }
                    _ => println!("unknown command"),
                }
            },
            _ => println!("unknown command"),
        }

        println!("{}", screen);
    }

    // Count lit pixels
    let mut lit_count = 0;

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            if screen.pixels[x][y] {
                lit_count += 1;
            }
        }
    }

    println!("{} pixels are lit", lit_count);
}

fn rotate_index(i: usize, rotation: usize, max: usize) -> usize {
    ((i + max) - (rotation % max)) % max
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}