// http://adventofcode.com/2016/day/1
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Transform {
    pos: Vec2,
    facing: char,
}

fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    println!("input: {}", input);

    println!("distance to end: {}", calc_route_distance(&input));
    
    match find_first_intersection(&input) {
        Some(intersection) => println!("distance to intersection: {:?}", intersection.x.abs() + intersection.y.abs()),
        None => println!("Couldn't find shit"),
    }
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String>{
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}

fn find_first_intersection(input: &str) -> Option<Vec2> {
    let commands = input.split(", ");
    let mut cur_transform = Transform{ pos: Vec2 { x: 0, y: 0 }, facing: 'N' };
    let mut path = vec![Vec2 { x:0, y:0 }];
    let mut intersection = None;

    'outer: for command in commands {
        let command_rot = command.chars().next().unwrap();
        let command_dist = command[1..].parse::<i32>().unwrap();

        cur_transform.facing = rotate_facing(cur_transform.facing, command_rot);

        for _ in 0..command_dist {
            translate(&mut cur_transform, 1);

            if path.contains(&cur_transform.pos) {
                intersection = Some(cur_transform.pos);
                break 'outer;
            } else {
                path.push(cur_transform.pos);
            }
        }
    }

    intersection
}

fn calc_route_distance(input: &str) -> i32 {
    let commands = input.split(", ");

    let end_transform = commands.fold(Transform{ pos: Vec2 { x: 0, y: 0 }, facing: 'N' }, 
        |transform, cur_command| {
            let mut new_transform = transform;

            let command_rot = cur_command.chars().next().unwrap();
            let command_dist = cur_command[1..].parse::<i32>().unwrap();

            new_transform.facing = rotate_facing(new_transform.facing, command_rot);
            translate(&mut new_transform, command_dist);

            println!("pos: {:?}, command_rot: {:?}, command_dist: {:?}", 
                transform, 
                command_rot,
                command_dist
            );

            new_transform
        }
    );

    (end_transform.pos.x + end_transform.pos.y).abs()
}

fn rotate_facing(facing: char, rotation: char) -> char {
    match rotation {
        'L' => match facing {
            'N' => 'W',
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            _   => panic!("That's not N, E, S or W!"),
        },
        'R' => match facing {
            'N' => 'E',
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            _   => panic!("That's not N, E, S or W!"),
        },
        _   => panic!("That's not L or R!"),
    }
}

fn translate(transform: &mut Transform, dist: i32) {
    match transform.facing {
        'N' => transform.pos.y += dist,
        'E' => transform.pos.x += dist,
        'S' => transform.pos.y -= dist,
        'W' => transform.pos.x -= dist,
        _   => panic!("That's not N, E, S or W!"),
    }
}