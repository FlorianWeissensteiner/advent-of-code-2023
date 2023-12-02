fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut combined_number = String::from("");
        let numbers = find_numbers(line);
        if let Some(first_number) = numbers.first() {
            combined_number.push(*first_number);
        }
        if let Some(last_number) = numbers.last() {
            combined_number.push(*last_number);
        }
        if let Ok(combined_number) = combined_number.parse::<u32>() {
            sum += combined_number;
        }
    }
    sum
}

fn find_numbers(line: &str) -> Vec<char> {
    let mut numbers = Vec::new();
    let numbers_lookup = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, char) in line.char_indices() {
        if char.is_numeric() {
            numbers.push(char);
            continue;
        }
        for number in numbers_lookup.iter() {
            let end = if i + number.len() > line.len() { line.len() } else { i + number.len() };
            if *number == &line[i..end] {
                numbers.push(convert_to_numeric(number));
                continue;
            }
        }
    }
    numbers
}

fn convert_to_numeric(number: &str) -> char {
    match number {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!("this should never happen"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example2.txt");
        let result = process(input);
        assert_eq!(result, 281);
    }
}