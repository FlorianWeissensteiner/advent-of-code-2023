use std::cmp;
fn main() {
    let input = include_str!("./input/part2.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let cube_reveals = parse_cube_reveals(line);
        let mut greatest_red = 0;
        let mut greatest_green = 0;
        let mut greatest_blue = 0;
        for cube_reveal in cube_reveals.iter() {
            greatest_red = cmp::max(cube_reveal.red, greatest_red);
            greatest_green = cmp::max(cube_reveal.green, greatest_green);
            greatest_blue = cmp::max(cube_reveal.blue, greatest_blue);
        }
        sum += greatest_red * greatest_green * greatest_blue;
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
        let input = include_str!("./input/example2.txt");
        let result = process(input);
        assert_eq!(result, 2286);
    }
}