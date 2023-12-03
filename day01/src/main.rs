use anyhow::Result;
use std::fs;
use std::path::Path;

fn part1and2() -> Result<()> {
    const INITIAL_FLOOR: i32 = 0;
    const MOVE_UP: char = '(';
    const MOVE_DOWN: char = ')';

    let mut floor = INITIAL_FLOOR;
    let input: String = (fs::read_to_string(Path::new("day01/input.txt"))?)
        .trim_end_matches('\n')
        .to_string();

    let mut entered = false;
    // change to enum to track position for part 2
    for (index, ch) in input.chars().enumerate() {
        match ch {
            MOVE_UP => floor += 1,
            MOVE_DOWN => {
                floor -= 1;
                // part 2 stuff
                if floor < 0 && !entered {
                    println!("Entering basement on position: {}", (index + 1));
                    entered = true;
                }
            }
            _any => println!("Bad char: {}", _any),
        }
    }

    println!("Floor: {:?}", floor);
    Ok(())
}

fn main() -> Result<()> {
    // read in file
    part1and2()?;
    Ok(())
}
