use rayon::prelude::*;

use std::io::{self, BufRead};

type Map = Vec<(u64, u64, u64)>;

enum MapKind {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Default)]
struct Almanac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn update_category(&mut self, map_kind: MapKind, lines: &mut impl Iterator<Item = String>) {
        let category_map = match map_kind {
            MapKind::SeedToSoil => &mut self.seed_to_soil,
            MapKind::SoilToFertilizer => &mut self.soil_to_fertilizer,
            MapKind::FertilizerToWater => &mut self.fertilizer_to_water,
            MapKind::WaterToLight => &mut self.water_to_light,
            MapKind::LightToTemperature => &mut self.light_to_temperature,
            MapKind::TemperatureToHumidity => &mut self.temperature_to_humidity,
            MapKind::HumidityToLocation => &mut self.humidity_to_location,
        };

        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }

            let mut parts = line.split_whitespace();
            let dest = parts.next().unwrap().parse::<u64>().unwrap();
            let src = parts.next().unwrap().parse::<u64>().unwrap();
            let len = parts.next().unwrap().parse::<u64>().unwrap();

            category_map.push((dest, src, len));
        }
    }

    fn map_category(&self, map_kind: MapKind, from: u64) -> u64 {
        let category_map = match map_kind {
            MapKind::SeedToSoil => &self.seed_to_soil,
            MapKind::SoilToFertilizer => &self.soil_to_fertilizer,
            MapKind::FertilizerToWater => &self.fertilizer_to_water,
            MapKind::WaterToLight => &self.water_to_light,
            MapKind::LightToTemperature => &self.light_to_temperature,
            MapKind::TemperatureToHumidity => &self.temperature_to_humidity,
            MapKind::HumidityToLocation => &self.humidity_to_location,
        };

        for &(dest, src, len) in category_map {
            if let Some(to) = Self::map(from, dest, src, len) {
                return to;
            }
        }
        from
    }

    fn map(
        n: u64,
        destination_range_start: u64,
        source_range_start: u64,
        range_length: u64,
    ) -> Option<u64> {
        if source_range_start <= n && n < source_range_start + range_length {
            Some(destination_range_start + (n - source_range_start))
        } else {
            None
        }
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);

    let seeds = lines.next().unwrap();

    let mut almanac = Almanac::default();
    loop {
        let Some(map_kind) = lines.next() else {
            break;
        };

        match &map_kind[..] {
            "seed-to-soil map:" => {
                almanac.update_category(MapKind::SeedToSoil, &mut lines);
            }
            "soil-to-fertilizer map:" => {
                almanac.update_category(MapKind::SoilToFertilizer, &mut lines);
            }
            "fertilizer-to-water map:" => {
                almanac.update_category(MapKind::FertilizerToWater, &mut lines);
            }
            "water-to-light map:" => {
                almanac.update_category(MapKind::WaterToLight, &mut lines);
            }
            "light-to-temperature map:" => {
                almanac.update_category(MapKind::LightToTemperature, &mut lines);
            }
            "temperature-to-humidity map:" => {
                almanac.update_category(MapKind::TemperatureToHumidity, &mut lines);
            }
            "humidity-to-location map:" => {
                almanac.update_category(MapKind::HumidityToLocation, &mut lines);
            }
            _ => {
                continue;
            }
        }
    }

    let (prefix, seed_ranges) = seeds
        .split_once(' ')
        .map(|(prefix, seed_ranges)| (prefix, seed_ranges.trim()))
        .unwrap();
    assert_eq!(prefix, "seeds:");

    let seed_ranges = seed_ranges
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let lowest_location = seed_ranges
        .par_chunks(2)
        .map(|range| (range[0]..(range[0] + range[1])))
        .flatten()
        .map(|x| almanac.map_category(MapKind::SeedToSoil, x))
        .map(|x| almanac.map_category(MapKind::SoilToFertilizer, x))
        .map(|x| almanac.map_category(MapKind::FertilizerToWater, x))
        .map(|x| almanac.map_category(MapKind::WaterToLight, x))
        .map(|x| almanac.map_category(MapKind::LightToTemperature, x))
        .map(|x| almanac.map_category(MapKind::TemperatureToHumidity, x))
        .map(|x| almanac.map_category(MapKind::HumidityToLocation, x))
        .min()
        .unwrap();

    println!("{lowest_location}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac() {
        let mut almanac = Almanac::default();
        let mut lines = ["52 50 48", "52 50 48"].map(String::from).into_iter();
        almanac.update_category(MapKind::SeedToSoil, &mut lines);

        assert_eq!(almanac.map_category(MapKind::SeedToSoil, 79), 81);
        assert_eq!(almanac.map_category(MapKind::SeedToSoil, 14), 14);
        assert_eq!(almanac.map_category(MapKind::SeedToSoil, 55), 57);
        assert_eq!(almanac.map_category(MapKind::SeedToSoil, 13), 13);
    }
}
