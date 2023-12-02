fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut combined_number = String::from("");
        let first_number = line.chars().find(|char| char.is_numeric());
        if let Some(first_number) = first_number {
            combined_number.push(first_number);
        }
        let last_number = line.chars().rev().find(|char| char.is_numeric());
        if let Some(last_number) = last_number {
            combined_number.push(last_number);
        }
        if let Ok(combined_number) = combined_number.parse::<u32>() {
            sum += combined_number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 142);
    }
}