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

    let mut tallies = [[0; 26]; 8];

    for line in input_lines {
        for (i, c) in line.chars().enumerate() {
            tallies[i][c as usize - 'a' as usize] += 1;
        }
    }

    let mut output1 = String::with_capacity(8);

    for tally in tallies.iter() {
        output1.push((find_max(tally) as u8 + 'a' as u8) as char);
    }

    println!("part 1: {}", output1);

    let mut output2 = String::with_capacity(8);

    for tally in tallies.iter() {
        output2.push((find_min(tally) as u8 + 'a' as u8) as char);
    }

    println!("part 2: {}", output2);
}

fn find_max<T>(array: &[T]) -> usize 
where T: Ord
{
    let mut index_of_max = 0;
    let mut cur_max = &array[0];

    for (i, value) in array.iter().enumerate() {
        if value > cur_max {
            cur_max = value;
            index_of_max = i;
        }
    }

    index_of_max
}

fn find_min<T>(array: &[T]) -> usize 
where T: Ord
{
    let mut index_of_min = 0;
    let mut cur_min = &array[0];

    for (i, value) in array.iter().enumerate() {
        if value < cur_min {
            cur_min = value;
            index_of_min = i;
        }
    }

    index_of_min
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}