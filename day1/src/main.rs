
use std::{io::{prelude::*, Lines, BufReader}};

fn get_elves(lines: Lines<BufReader<std::fs::File>>) -> Vec<u32> {
    let mut elves: Vec<u32> = vec![];
    let mut buffer_calories: Vec<u32> = vec![];
    for line in lines {
        match line {
            Ok(val) => match val.as_str() {
                "\n" | "" => {
                    elves.push(buffer_calories.iter().sum());
                    buffer_calories.clear();
                    continue;
                }
                calorie_str => {
                    match calorie_str.parse::<u32>() {
                        Ok(val) => buffer_calories.push(val),
                        Err(why) => panic!("Error parsing calorie input: {}", why.to_string())
                    };
                }
            },
            Err(why) => panic!("Error while parsing input file: {}", why.to_string())
        };
    }
    elves
}

fn get_elf_with_most_calories(elves: &Vec<u32>) -> &u32 {
    let mut high = &elves[0];
    for elf in elves {
        if elf > high {
            high = elf;
        }
    };

    high
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "input.txt".to_string());
    let file = std::fs::File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let elves = get_elves(reader.lines());
    let high_elf = get_elf_with_most_calories(&elves);

    println!("The elf with the most calories is: {}", high_elf);
}
