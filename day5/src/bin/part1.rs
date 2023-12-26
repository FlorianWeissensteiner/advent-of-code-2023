use std::collections::HashMap;

fn main() {
    let input = include_str!("./input/part1.txt");
    let output = process(input);
    println!("{output}");
}

fn process(input: &str) -> u64 {
    let mut almanac = Almanac{
        seeds: vec![],
        maps: HashMap::new(),
    };
    let mut active_map_type = AlmanacMapType::None;
    for line in input.lines() {
        if line.starts_with("seeds:") {
            almanac.seeds = line[7..].split(' ').map(|number| number.parse::<u64>().expect("should work")).collect();
        }

        let current_map_type = get_current_map_type(&line);
        if current_map_type != AlmanacMapType::None {
            active_map_type = current_map_type;
        }

        if active_map_type != AlmanacMapType::None && current_map_type == AlmanacMapType::None && line != "" {
            let map: Vec<u64> = line.split(' ').map(|number| number.parse::<u64>().expect("should work")).collect();
            let maps = almanac.maps.entry(active_map_type).or_insert(vec![]);
            maps.push(AlmanacMap{
                destination_range_start: *map.get(0).unwrap(),
                source_range_start: *map.get(1).unwrap(),
                range_length: *map.get(2).unwrap(),
            });
        }
    }

    let mut smallest_location = u64::MAX;
    for seed in almanac.seeds.iter() {
        let soil = almanac.apply_map(AlmanacMapType::SeedToSoil, *seed);
        let fertilizer = almanac.apply_map(AlmanacMapType::SoilToFertilizer, soil);
        let water = almanac.apply_map(AlmanacMapType::FertilizerToWater, fertilizer);
        let light = almanac.apply_map(AlmanacMapType::WaterToLight, water);
        let temperature = almanac.apply_map(AlmanacMapType::LightToTemperature, light);
        let humidity = almanac.apply_map(AlmanacMapType::TemperatureToHumidity, temperature);
        let location = almanac.apply_map(AlmanacMapType::HumidityToLocation, humidity);
        smallest_location = smallest_location.min(location);
    }

   smallest_location 
}

fn get_current_map_type(line: &str) -> AlmanacMapType {
    match line {
        "seed-to-soil map:" => AlmanacMapType::SeedToSoil,
        "soil-to-fertilizer map:" => AlmanacMapType::SoilToFertilizer,
        "fertilizer-to-water map:" => AlmanacMapType::FertilizerToWater,
        "water-to-light map:" => AlmanacMapType::WaterToLight,
        "light-to-temperature map:" => AlmanacMapType::LightToTemperature,
        "temperature-to-humidity map:" => AlmanacMapType::TemperatureToHumidity,
        "humidity-to-location map:" => AlmanacMapType::HumidityToLocation,
        _ => AlmanacMapType::None,
    }
}


#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<AlmanacMapType, Vec<AlmanacMap>>,
}

impl Almanac {
    fn apply_map(&self, map_type: AlmanacMapType, value: u64) -> u64 {
        for map in self.maps.get(&map_type).unwrap().iter() {
            if (map.source_range_start..(map.source_range_start+map.range_length)).contains(&value) {
                return map.destination_range_start + (value - map.source_range_start);
            }
        }
        value
    }
}

#[derive(Debug)]
struct AlmanacMap {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum AlmanacMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = include_str!("./input/example1.txt");
        let result = process(input);
        assert_eq!(result, 35);
    }
}