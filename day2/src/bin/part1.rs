fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut sum = 0;
    for line in input.lines() {
        let game_id_start = 5; // "Game _" 5th character
        let game_id_end = line.find(':').unwrap();
        let game_id = &line[game_id_start..game_id_end].parse::<u32>().unwrap();
        let cube_reveals = parse_cube_reveals(line);
        let mut is_possible = true;
        for cube_reveal in cube_reveals.iter() {
            if cube_reveal.red > red_max || cube_reveal.green > green_max || cube_reveal.blue > blue_max {
                is_possible = false;
                break;
            }
        }
        if is_possible {
            sum += game_id;
        }
    }
    sum
}

fn parse_cube_reveals(line: &str) -> Vec<CubeReveal> {
    let mut cube_reveals = Vec::new();
    let reveals_start = line.find(':').unwrap() + 1;
    let reveals = line[reveals_start..line.len()].split(';');
    for reveal in reveals {
        let mut cube_reveal = CubeReveal{red:0,green:0,blue:0};
        let cube_amounts = reveal.split(',');
        for cube_amount in cube_amounts {
            let items: Vec<&str> = cube_amount.trim().split(' ').collect();
            let amount = items[0].parse::<u32>().unwrap_or(0);
            let color = items[1];
            match color {
                "red" => cube_reveal.red = amount, 
                "green" => cube_reveal.green = amount, 
                "blue" => cube_reveal.blue = amount, 
                _ => (),
            }
        }
        cube_reveals.push(cube_reveal);
    }
   cube_reveals 
}

#[derive(Debug)]
struct CubeReveal {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 8);
    }
}