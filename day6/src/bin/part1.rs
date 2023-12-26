fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = lines.get(0).unwrap()["Time:".len()..]
        .split(' ')
        .filter(|item| *item != "")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines.get(1).unwrap()["Distance:".len()..]
        .split(' ')
        .filter(|item| *item != "")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    let mut margin = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let mut wins = 0;
        for ms in 1..=*time {
            let speed = ms;
            if speed * (time - ms) > *distance {
                wins += 1;
            }
        }
        margin *= wins;
    }

    margin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 288);
    }
}