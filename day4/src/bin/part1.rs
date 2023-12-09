fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let start = line.find(':').expect("should exist") + 1;
        let numbers: Vec<Vec<u32>> = line[start..]
            .split('|')
            .map(|numbers|
                numbers
                    .split(' ')
                    .filter_map(|number| {
                        if number.is_empty() {
                            None
                        } else {
                            Some(number.parse::<u32>().expect("should work"))
                        }
                    })
                    .collect()
            )
            .collect();

        if let [winning_numbers, card_numbers] = &numbers[..] {
            let matching_numbers_count = card_numbers
                .iter()
                .filter(|number| winning_numbers.contains(number))
                .count();
            sum += get_points(matching_numbers_count as u32);

        } else {
            panic!("this shouldn't happen");
        }
    }

    sum
}

fn get_points(matching_number_count: u32) -> u32 {
    match matching_number_count {
        0 => 0,
        1 => 1,
        count => (1..count).fold(1, |acc, _| acc * 2),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 13);
    }
}