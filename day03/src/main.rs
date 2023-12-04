use std::fs;
use std::path::Path;

use anyhow::Result;

fn get_symbol_indices(line: &str) -> Result<Vec<u32>> {
    let mut indices: Vec<u32> = Vec::new();
    for (cindex, c) in line.chars().enumerate() {
        if !c.is_numeric() && c != '.' {
            // if not numeric or period, it's a symbol
            indices.push(cindex as u32);
        }
    }
    Ok(indices)
}

// takes start and end index of a number
// takes optional vec of where symbols are above or vbelow
// adds 1 to end index and minuses 1 to start index (if > 0) to account for diagnonal
// ranges over extended start / end to determine directly above
fn is_touching_other_line(start_index: u32, end_index: u32, symbols: &Option<Vec<u32>>) -> bool {
    if symbols.is_some() && !symbols.as_ref().unwrap().is_empty() {
        // range start - 1 to end -1, check if theres a symbole there
        let mut start = start_index;
        let end = end_index + 1; // add 1 to end;
        if start_index > 0 {
            start -= 1; // -1 if not 0
        }

        for i in start..=end {
            if symbols.as_ref().unwrap().contains(&i) {
                return true;
            }
            // loop over range, if it's in the list
        }
    }

    false
}

fn is_touching(
    start_index: u32,
    end_index: u32,
    current_symbols: &Option<Vec<u32>>,
    previous_symbols: &Option<Vec<u32>>,
    next_symbols: &Option<Vec<u32>>,
) -> Result<bool> {
    // first check current line, symbols could be at +/-
    // check left
    if start_index > 0
        && current_symbols
            .as_ref()
            .unwrap()
            .contains(&(start_index - 1))
    {
        // it's on the left
        return Ok(true);
    }
    // check right
    if current_symbols.as_ref().unwrap().contains(&(end_index + 1)) {
        return Ok(true);
    }
    if is_touching_other_line(start_index, end_index, previous_symbols)
        || is_touching_other_line(start_index, end_index, next_symbols)
    {
        return Ok(true);
    }

    Ok(false)
}

fn get_number_indices(line: &str) -> Result<Vec<(u32, u32, u32)>> {
    // get the numbers for each line and accumulate their indicies
    let mut data: Vec<(u32, u32, u32)> = Vec::new();
    let mut current_num: Option<u32> = None;
    let mut current_num_start_index: Option<u32> = None;
    let mut cindex: u32 = 0;
    let mut c_iter = line.chars().peekable();

    while let Some(c) = c_iter.next() {
        let c_next = c_iter.peek();
        if c.is_numeric() {
            let cnum = c.to_digit(10).unwrap();
            match current_num {
                Some(num) => current_num = Some((num * 10) + cnum),
                None => {
                    // we're starting a new current_num
                    current_num = Some(cnum);
                    // set start index
                    current_num_start_index = Some(cindex);
                }
            }
            // check if next exists and is not a digit, then do the calculations
            match c_next {
                Some(c_next) => {
                    if !c_next.is_numeric() {
                        // c_next is not numeric, this current_num is done
                        data.push((
                            current_num.unwrap(),
                            current_num_start_index.unwrap(),
                            cindex,
                        ));
                        // reset
                        // reset
                        current_num = None;
                        current_num_start_index = None;
                    }
                }
                None => {
                    // no other chars, end of the road, run the comparisons and calculate
                    data.push((
                        current_num.unwrap(),
                        current_num_start_index.unwrap(),
                        cindex,
                    ));
                    // reset
                    // reset
                    current_num = None;
                    current_num_start_index = None;
                }
            }
        }
        cindex += 1;
    }
    Ok(data)
}

// (value, start, end)
type NumberData = Vec<(u32, u32, u32)>;

fn get_numbers_touching_gears_outside(
    gear_index: u32,
    number_indices: &Option<NumberData>,
) -> Option<Vec<u32>> {
    let mut nums: Vec<u32> = Vec::new();
    if number_indices.is_some() && !number_indices.as_ref().unwrap().is_empty() {
        for number_data in number_indices.as_ref().unwrap() {
            // range start - 1 to end -1, check if theres a symbole there
            let mut start = number_data.1;
            let end = number_data.2 + 1; // add 1 to end;
                                         //
            if number_data.1 > 0 {
                start -= 1; // -1 if not 0
            }

            for i in start..=end {
                // loooping over extended number indexes
                if i == gear_index {
                    // diagnonal or vertical connector
                    // push
                    nums.push(number_data.0);
                    // break out to not add the number again
                    break;
                }
            }
        }
    }

    Some(nums)
}

fn get_two_numbers_touching_gears(
    gear_index: u32,
    number_indices: &Option<NumberData>,
    prev_number_indices: &Option<NumberData>,
    next_number_indices: &Option<NumberData>,
) -> Option<(u32, u32)> {
    let mut nums: Vec<u32> = Vec::new();

    for num in number_indices.as_ref().unwrap() {
        let val = num.0;
        let start = num.1;
        let end = num.2;

        // check number to left
        if gear_index > 0 && end == (gear_index - 1) {
            nums.push(val);
        }
        // number to the right
        if start == gear_index + 1 {
            nums.push(val);
        }
    }

    dbg!(&prev_number_indices);
    let prev_nums: Option<Vec<u32>> =
        get_numbers_touching_gears_outside(gear_index, &prev_number_indices);
    dbg!(&prev_nums);
    if prev_nums.is_some() && !prev_nums.clone().unwrap().is_empty() {
        nums.append(&mut prev_nums.unwrap());
    }

    let next_nums: Option<Vec<u32>> =
        get_numbers_touching_gears_outside(gear_index, &next_number_indices);
    dbg!(&next_nums);

    if next_nums.is_some() && !next_nums.clone().unwrap().is_empty() {
        nums.append(&mut next_nums.unwrap());
    }

    if nums.len() == 2 {
        // only two
        Some((nums[0], nums[1]))
    } else {
        // too many / not enought
        None
    }
}

fn part2() -> Result<()> {
    let mut gear_total: u32 = 0;
    let input = fs::read_to_string(Path::new("day03/input.txt"))?;

    let mut prev_number_indices: Option<Vec<(u32, u32, u32)>> = Some(Vec::new());

    let mut input_iter = input.lines().peekable();
    while let Some(line) = input_iter.next() {
        // loop over line
        // (number_value, start_index, end_index)
        let number_indices: Option<Vec<(u32, u32, u32)>> = Some(get_number_indices(line)?);
        let mut next_number_indices = None;

        if input_iter.peek().is_some() {
            next_number_indices = Some(get_number_indices(input_iter.peek().unwrap())?);
        }

        // we now have the numbers and indices for the current, previous and next line
        // now loop over this line for gears and try to find touching numbers

        for (cindex, c) in line.chars().enumerate() {
            if c == '*' {
                dbg!(c);
                dbg!(cindex);
                dbg!(&number_indices);
                dbg!(&prev_number_indices);
                dbg!(&next_number_indices);
                let touching: Option<(u32, u32)> = get_two_numbers_touching_gears(
                    cindex as u32,
                    &number_indices,
                    &prev_number_indices,
                    &next_number_indices,
                );
                dbg!(&touching);

                if touching.is_some() {
                    gear_total = gear_total + (touching.unwrap().0 * touching.unwrap().1);
                }
            }
        }

        prev_number_indices = number_indices;
    }

    dbg!(gear_total);
    Ok(())
}

fn part1() -> Result<()> {
    // input: any number adjacent to a symbol, even diagonally, is a part pnumber
    // periods do not count as symbol
    // each line is 140 chars
    let mut part_numbers: Vec<u32> = Vec::new();
    let input = fs::read_to_string(Path::new("day03/input.txt"))?;
    // let input = fs::read_to_string(Path::new("day03/input.txt"))?;
    let mut prev_symbol_indices: Option<Vec<u32>> = Some(Vec::new());

    let mut input_iter = input.lines().peekable();
    while let Some(line) = input_iter.next() {
        let mut current_num: Option<u32> = None;
        let mut current_num_start_index: Option<u32> = None;

        let mut c_iter = line.chars().peekable();
        let symbol_indices = Some(get_symbol_indices(line)?);
        let mut next_symbol_indices = None;

        if input_iter.peek().is_some() {
            next_symbol_indices = Some(get_symbol_indices(input_iter.peek().unwrap())?);
        }

        let mut cindex: u32 = 0;
        while let Some(c) = c_iter.next() {
            let c_next = c_iter.peek();
            if c.is_numeric() {
                let cnum = c.to_digit(10).unwrap();
                match current_num {
                    Some(num) => current_num = Some((num * 10) + cnum),
                    None => {
                        // we're starting a new current_num
                        current_num = Some(cnum);
                        // set start index
                        current_num_start_index = Some(cindex);
                    }
                }
                // check if next exists and is not a digit, then do the calculations
                match c_next {
                    Some(c_next) => {
                        if !c_next.is_numeric() {
                            // c_next is not numeric, this current_num is done

                            if is_touching(
                                current_num_start_index.unwrap(),
                                cindex, // current_num_end_index
                                &symbol_indices,
                                &prev_symbol_indices,
                                &next_symbol_indices,
                            )? {
                                part_numbers.push(current_num.unwrap());
                            }

                            // reset
                            current_num = None;
                            current_num_start_index = None;
                        }
                    }
                    None => {
                        if is_touching(
                            current_num_start_index.unwrap(),
                            cindex, // current_num_end_index
                            &symbol_indices,
                            &prev_symbol_indices,
                            &next_symbol_indices,
                        )? {
                            part_numbers.push(current_num.unwrap());
                        }

                        // no other chars, end of the road, run the comparisons and calculate
                        // reset
                        current_num = None;
                        current_num_start_index = None;
                    }
                }
            }
            cindex += 1;
        }

        prev_symbol_indices = symbol_indices;
    }

    let total: u32 = part_numbers.iter().sum();
    dbg!(total);

    Ok(())
}
fn main() -> Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
