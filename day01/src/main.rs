use std::fs;
use std::path::Path;

use anyhow::Result;

fn grab_numbers(line: &str) -> Result<u16> {
    let nums: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    if nums.len() == 1 {
        let x = nums[0].to_digit(10).unwrap() as u16;
        return Ok(x);
    }
    let mut fl = String::new();
    fl.push(nums[0]);
    fl.push(*nums.last().unwrap());
    Ok(fl.parse::<u16>()?)
}

fn part1(input: &str) -> Result<()> {
    let mut total = 0;
    for line in input.lines() {
        total += grab_numbers(line)?;
    }
    dbg!(total);
    Ok(())
}

const NUMSTRS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(input: &str) -> Result<()> {
    let mut modded = input.to_string();
    // pretty inefficient but less code
    // this didn't work because it needs to be sliding window, you can have twone which should be
    // two not one
    // ACTUALLY twone should be both 2 and 1, not just 2... pain...
    //
    // for (index, needle) in NUMSTRS.iter().enumerate() {
    //     let newnum = (index + 1).to_string();
    //     println!("Replacing: {} with {}", needle.clone(), newnum.clone());
    //     modded = modded.replace(needle, &newnum);
    //     dbg!(&modded);
    // }
    //

    // for index in 0..(input.len() - 1) {
    //     // modded shrinks so, just bail if at end
    //     if index >= modded.len() {
    //         break;
    //     }
    //     let current = (modded[index..]).to_string();
    //     for (index, needle) in NUMSTRS.iter().enumerate() {
    //         if current.starts_with(needle) {
    //             let newnum = (index + 1).to_string();
    //             let short_needle = &needle[0..(needle.len() - 1)];
    //             // prettty hacky, just leave the last one in there to help complete the rest of the
    //             // next... so for twoone, on two, replace tw with 2 to become 2one and then 21...
    //             modded = modded.replacen(short_needle, &newnum, 1);
    //             println!(
    //                 "Replacing: {} with {}",
    //                 short_needle.clone(),
    //                 newnum.clone()
    //             );
    //         }
    //     }
    // }
    //
    //

    // rewrite everything
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut linenums: Vec<u32> = Vec::new();
        for (index, c) in line.chars().enumerate() {
            let current = &line[index..];
            if c.is_numeric() {
                linenums.push(c.to_digit(10).unwrap() as u32);
            } else {
                for (index, needle) in NUMSTRS.iter().enumerate() {
                    if current.starts_with(needle) {
                        let newnum = (index + 1) as u32;
                        linenums.push(newnum);
                        break;
                    }
                }
            }
        }
        dbg!(&line);
        dbg!(&linenums);
        if linenums.len() == 1 {
            // if only single number, like 7, should be 77
            // undocumented behavior in question
            let mut fl = String::new();
            fl.push(char::from_digit(linenums[0], 10).unwrap());
            fl.push(char::from_digit(linenums[0], 10).unwrap());
            let another = fl.parse::<u32>()?;
            println!("Adding to total: {}", another);
            total += another;
        } else {
            let mut fl = String::new();
            fl.push(char::from_digit(linenums[0], 10).unwrap());
            fl.push(char::from_digit(*linenums.last().unwrap(), 10).unwrap());
            let another = fl.parse::<u32>()?;
            println!("Adding to total: {}", another);
            total += another;
        }
    }
    dbg!(total);
    Ok(())
}
fn main() -> Result<()> {
    let input = fs::read_to_string(Path::new("day01/input.txt"))?;
    //part1(&input)?;
    part2(&input)?;
    Ok(())
}
