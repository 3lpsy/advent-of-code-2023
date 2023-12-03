use std::fs;
use std::path::Path;

use anyhow::Result;
fn part1and2() -> Result<()> {
    // bag of cubes red green or blue
    // each time you play, hide a secret number of cubes in the bag
    // Figure out info about number of cubes
    // grab a handful of cubes and show a few times per game
    // each set is separated by semi colon
    const AT_MOST_RED: u16 = 12;
    const AT_MOST_GREEN: u16 = 13;
    const AT_MOST_BLUE: u16 = 14;

    let mut valid_game_ids: Vec<u16> = Vec::new();

    //part 2
    let mut game_powers: Vec<u32> = Vec::new();

    let input: String = fs::read_to_string(Path::new("day02/input.txt"))?
        .trim_end_matches('\n')
        .to_string();

    for line in input.lines() {
        // for part2, what is the fewest number of cubes in each bag for each color
        // return the sum of the powers
        let mut min_red: u16 = 0;
        let mut min_blue: u16 = 0;
        let mut min_green: u16 = 0;

        // each game
        let (game_header, game_data) = line.split_once(':').expect("Expect two parts");
        let game_id = game_header
            .split_once(' ')
            .expect("Invalid game header")
            .1
            .parse::<u16>()?;

        let mut is_valid = true;
        for bundle in game_data.split(';') {
            for color_data in bundle.trim().split(',') {
                let (color_amount, color_name) = color_data
                    .trim()
                    .split_once(' ')
                    .expect("Invalid color data");
                let color_amount = color_amount.parse::<u16>()?;

                match color_name {
                    "red" => {
                        // part 2
                        if min_red < color_amount {
                            min_red = color_amount;
                        }
                        if color_amount > AT_MOST_RED {
                            is_valid = false;
                        }
                    }
                    "green" => {
                        // part 2
                        if min_green < color_amount {
                            min_green = color_amount;
                        }
                        if color_amount > AT_MOST_GREEN {
                            is_valid = false;
                        }
                    }
                    "blue" => {
                        // part 2
                        if min_blue < color_amount {
                            min_blue = color_amount;
                        }
                        if color_amount > AT_MOST_BLUE {
                            is_valid = false;
                        }
                    }
                    _ => panic!("Not a color: {}", color_name),
                }
            }
        }
        if is_valid {
            valid_game_ids.push(game_id);
        }

        game_powers.push(min_red as u32 * min_blue as u32 * min_green as u32);
    }

    // find a round where the above are violated and throw out
    // calculate sum of valid game IDs
    let total: u16 = valid_game_ids.iter().sum();
    dbg!(total);

    // part 2
    let sum_of_powers: u32 = game_powers.iter().sum();
    dbg!(sum_of_powers);
    // multiply mins
    Ok(())
}
fn main() -> Result<()> {
    part1and2()?;
    Ok(())
}
