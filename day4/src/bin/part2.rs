use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/part2.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let mut card_to_count: HashMap<usize, i32> = HashMap::new();
    for idx in 0..input.lines().count() {
        card_to_count.insert(idx, 1);
    }

    let mut total_card_count = 0;
    for (idx, line) in input.lines().enumerate() {
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

            for _ in 0..(*card_to_count.get(&idx).unwrap()) {
                total_card_count += 1;
                if matching_numbers_count > 0 {
                    let start = idx + 1;
                    let end = start + matching_numbers_count;
                    for card in start..end {
                        card_to_count
                            .entry(card)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
            }
        } else {
            panic!("this shouldn't happen");
        }
    }

    total_card_count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example2.txt");
        let result = process(input);
        assert_eq!(result, 13);
    }
}