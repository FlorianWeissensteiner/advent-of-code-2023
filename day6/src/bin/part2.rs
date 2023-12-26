fn main() {
    let input = include_str!("./input/part2.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines.get(0).unwrap()["Time:".len()..]
        .split(' ')
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>().unwrap();
    let distance: u64 = lines.get(1).unwrap()["Distance:".len()..]
        .split(' ')
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>().unwrap();

    let mut win_start = 0;
    let mut win_end = 0;
    for ms in 1..=time {
        let speed = ms;
        if speed * (time - ms) > distance {
            win_start = ms;
            break;
        }
    }
    for ms in (1..=time).rev() {
        let speed = ms;
        if speed * (time - ms) > distance {
            win_end = ms;
            break;
        }
    }

    (win_start..=win_end).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example2.txt");
        let result = process(input);
        assert_eq!(result, 71503);
    }
}