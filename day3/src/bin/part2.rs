use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/part2.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    let mut gear_to_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (y, line) in schematic.iter().enumerate() {
        let mut part_number_start = 0;
        let mut part_number = String::new();
        for (x, char) in line.iter().enumerate() {
            if char.is_numeric() {
                part_number.push(*char);
            } else {
                part_number_start += 1
            }

            let last_digit = if x == line.len() - 1 {
                true 
            } else {
                !line.get(x + 1).unwrap().is_numeric()
            };

            if last_digit && part_number.len() > 0 {

                let area_y_start = ((y as i32) - 1).max(0) as usize;
                let area_y_end = (y + 1).min(schematic.len() - 1);
                let area_x_start = ((part_number_start as i32) - 1).max(0) as usize;
                let area_x_end = (x + 1).min(line.len() - 1);

                let area_gears = get_area_gears(&schematic, area_y_start, area_y_end, area_x_start, area_x_end);
                for gear in area_gears.iter() {
                    if let Ok(part_number) = part_number.parse::<u32>() {
                        gear_to_numbers
                            .entry(*gear)
                            .and_modify(|numbers| numbers.push(part_number))
                            .or_insert(vec![part_number]);
                    }
                }

                part_number.clear();
                part_number_start = (x + 1).min(line.len());
            }
        }
    }

    for (_gear, numbers) in gear_to_numbers {
        if numbers.len() == 2 {
            sum += numbers.iter().fold(1, |acc, &number| acc * number);
        }
    }

    sum
}

fn get_area_gears(schematic: &Vec<Vec<char>>, area_y_start: usize, area_y_end: usize, area_x_start: usize, area_x_end: usize) -> Vec<(usize, usize)> {
    let mut gears = Vec::new();
    for area_y in area_y_start..=area_y_end {
        let area_line = schematic.get(area_y).expect("Line should exist");
        for area_x in area_x_start..=area_x_end {
            let symbol = area_line.get(area_x).expect("Char should exist");
            if *symbol == '*' {
                gears.push((area_x, area_y));
            }
        }
    }
    gears
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 467835);
    }
}