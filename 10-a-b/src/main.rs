#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Reciever {
    BotId(usize),
    OutputBinId(usize),
}

#[derive(Copy, Clone, Debug)]
struct Bot {
    id: usize,
    low_value: Option<usize>,
    high_value: Option<usize>,
    output_low: Reciever,
    output_high: Reciever
}

#[derive(Copy, Clone, Debug)]
struct InputBin {
    value: usize,
    output: Reciever
}

#[derive(Debug)]
struct Model {
    input_bins: Vec<Option<InputBin>>, // index == value
    bots: Vec<Option<Bot>> // index == bot number
}

impl Model {
    fn new() -> Model {
        Model { input_bins: Vec::new(), bots: Vec::new() }
    }

    fn add_bot(&mut self, bot: Bot) {
        if self.bots.len() <= bot.id {
            self.bots.resize(bot.id + 1, None);
        }

        self.bots[bot.id] = Some(bot);
    }

    fn add_input_bin(&mut self, input_bin: InputBin) {
        if self.input_bins.len() <= input_bin.value {
            self.input_bins.resize(input_bin.value + 1, None);
        }

        self.input_bins[input_bin.value] = Some(input_bin);
    }

    fn assign_bot_values(&mut self) {
        let mut bot_ids_to_process: Vec<usize> = Vec::new();

        for option_input_bin in &self.input_bins {
            if let &Some(output_bin) = option_input_bin {
                if let Reciever::BotId(bot_id) = output_bin.output {
                    let value = Some(output_bin.value);

                    if let Some(bot) = self.bots[bot_id].as_mut() {
                        if bot.low_value.is_none() {
                            bot.low_value = value;
                        } else {
                            if bot.low_value.unwrap() > output_bin.value {
                                bot.high_value = bot.low_value;
                                bot.low_value = value;
                            } else {
                                bot.high_value = value;
                            }
                            bot_ids_to_process.push(bot_id);
                        }
                    }
                }
            }
        }

        while bot_ids_to_process.len() > 0 {
            let mut new_bot_ids_to_process: Vec<usize> = Vec::new();

            for bot_id in bot_ids_to_process {
                if let Reciever::BotId(output_low_bot_id) = self.bots[bot_id].unwrap().output_low {
                    let value = self.bots[bot_id].unwrap().low_value;
                    
                    if let Some(bot) = self.bots[output_low_bot_id].as_mut() {
                        if bot.low_value.is_none() {
                            bot.low_value = value;
                        } else {
                            if bot.low_value.unwrap() > value.unwrap() {
                                bot.high_value = bot.low_value;
                                bot.low_value = value;
                            } else {
                                bot.high_value = value;
                            }
                            new_bot_ids_to_process.push(output_low_bot_id);
                        }
                    }
                }

                if let Reciever::BotId(output_high_bot_id) = self.bots[bot_id].unwrap().output_high {
                    let value = self.bots[bot_id].unwrap().high_value;
                    
                    if let Some(bot) = self.bots[output_high_bot_id].as_mut() {
                        if bot.low_value.is_none() {
                            bot.low_value = value;
                        } else {
                            if bot.low_value.unwrap() > value.unwrap() {
                                bot.high_value = bot.low_value;
                                bot.low_value = value;
                            } else {
                                bot.high_value = value;
                            }
                            new_bot_ids_to_process.push(output_high_bot_id);
                        }
                    }
                }
            }

            bot_ids_to_process = new_bot_ids_to_process;
        }
    }
}


fn main() {
    let input: String;

    match file_to_string(&"input") {
        Err(why) => panic!("Error: {}", why),
        Ok(contents) => input = contents,
    }

    let model = parse_instructions(&input);

    println!("The bot that compares 17 and 61 is bot {:?}.", get_id_of_bot_with_values(&model, 17, 61).unwrap());
    println!("The product of a single chip from output bins 0, 1 and 2 is {}", 
            get_value_in_output_bin(&model, 0).unwrap() * get_value_in_output_bin(&model, 1).unwrap() * get_value_in_output_bin(&model, 2).unwrap());
}

fn file_to_string<P: AsRef<Path>>(file_path: P) -> std::io::Result<String> {
    let mut contents = String::new();
    File::open(file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_instructions(instructions: &str) -> Model {
    lazy_static! {
        static ref INPUT_BIN_MATCHER: Regex = Regex::new(r"^value (\d+) goes to ((?:bot|output) \d+)$").unwrap();
        static ref BOT_MATCHER: Regex = Regex::new(r"^bot (\d+) gives low to ((?:bot|output) \d+) and high to ((?:bot|output) \d+)$").unwrap();
    }

    let instruction_lines = instructions.split("\r\n");
    let mut model = Model::new();

    for line in instruction_lines {
        if line.starts_with('v') {
            let captures = INPUT_BIN_MATCHER.captures(&line).unwrap();

            let input_bin = InputBin { value: captures[1].parse::<usize>().unwrap(),
                                       output: string_to_receiever(&captures[2]).unwrap() };

            model.add_input_bin(input_bin);

        } else if line.starts_with('b') {
            let captures = BOT_MATCHER.captures(&line).unwrap();
            
            let bot = Bot { id: captures[1].parse::<usize>().unwrap(),
                            output_low: string_to_receiever(&captures[2]).unwrap(),
                            output_high: string_to_receiever(&captures[3]).unwrap(),
                            low_value: None,
                            high_value: None };

            model.add_bot(bot);
        }
    }

    model.assign_bot_values();

    model
}

fn get_id_of_bot_with_values(model: &Model, low_value: usize, high_value: usize) -> Option<usize> {
    for optional_bot in &model.bots {
        if let Some(bot) = optional_bot.as_ref() {
            if bot.high_value.unwrap() == high_value
                    && bot.low_value.unwrap() == low_value
            {
                return Some(bot.id);
            }
        }
    }
    None
}

fn get_value_in_output_bin(model: &Model, bin_id: usize) -> Option<usize> {
    let target_bin = Reciever::OutputBinId(bin_id); 

    for optional_bot in &model.bots {
        if let Some(bot) = optional_bot.as_ref() {
            if bot.output_high == target_bin {
                return bot.high_value;
            } else if bot.output_low == target_bin {
                return bot.low_value;
            }
        }
    }

    None
}

fn string_to_receiever(s: &str) -> Option<Reciever> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(bot|output) (\d+)$").unwrap();
    }

    let captures;
    
    match RE.captures(s) {
        Some(caps) => captures = caps,
        None => return None,
    }

    let id;

    match captures[2].parse::<usize>() {
        Ok(num) => id = num,
        Err(_) => return None,
    }

    match &captures[1] {
        "bot" => Some(Reciever::BotId(id)),
        "output" => Some(Reciever::OutputBinId(id)),
        _ => None
    }
} 