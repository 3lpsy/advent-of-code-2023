use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::Result;

fn main() -> Result<()> {
    //part1()?;
    part2()?;
    Ok(())
}

fn parse_deck(
    mut deck: &Vec<String>,
    dictionary: &Vec<String>,
    mut current_index: u32,
    mut current_total: u32,
) -> Result<u32> {
    for round in deck.into_iter() {
        let matching_amount = get_matching_amount(&round)?;
        dbg!(current_index);
        let new_cards: Vec<String> = dictionary
            .clone()
            .into_iter()
            .skip(current_index as usize + 1)
            .take(matching_amount as usize)
            .collect();
        current_total += 1;
        current_index += 1;
        let additional_matches = parse_deck(&new_cards, dictionary, current_index, current_total)?;
        current_total = additional_matches;
    }
    Ok(current_total)
}
fn part2() -> Result<()> {
    // scratchcarsds let you win more scratchcards equal to number of winning numbers you have
    //
    let input = fs::read_to_string(Path::new("day04/input.txt"))?;
    // need to offset index
    let dictionary: Vec<String> = input
        .lines()
        .map(String::from_str)
        .map(|x| x.unwrap())
        .collect();
    let deck = dictionary.clone();
    dbg!("Start");
    let x = parse_deck(&deck, &dictionary, 0, 0)?;
    dbg!(x);

    Ok(())
}
fn part1() -> Result<()> {
    // each card has two lists separated by bar, not equal to eachother but equal between lines
    // (same for each line)
    //
    // first list is list of winning numbers
    // second list is your numbers
    //
    // Question: Which of the numbers you have appear in the list of winning numbers
    // first match worth one point
    // each subsewquent match doubles that card's points
    //
    //
    //
    let input = fs::read_to_string(Path::new("day04/input.txt"))?;
    let total = input.lines().fold(0, |acc, line| {
        let line = line.replace("  ", " ");

        let (winners, hand) = line.split_once(':').unwrap().1.split_once('|').unwrap();

        let winners: Vec<u32> = winners
            .trim()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let hand: Vec<u32> = hand
            .trim()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        acc + hand.into_iter().fold(0, |mut hacc, candidate| {
            if winners.contains(&candidate) {
                dbg!("Found", candidate);
                if hacc < 2 {
                    // handle zero and 1 case
                    hacc += 1;
                } else {
                    hacc *= 2;
                }
            }
            hacc
        })
    });
    dbg!(total);

    Ok(())
}

fn get_matching_amount(line: &str) -> Result<u32> {
    let line = line.replace("  ", " ");
    let (winners, hand) = line.split_once(':').unwrap().1.split_once('|').unwrap();

    let winners: Vec<u32> = winners
        .trim()
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    let hand: Vec<u32> = hand
        .trim()
        .split(' ')
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    Ok(hand.into_iter().fold(0, |mut hacc, candidate| {
        if winners.contains(&candidate) {
            hacc += 1;
        }
        hacc
    }))
}
