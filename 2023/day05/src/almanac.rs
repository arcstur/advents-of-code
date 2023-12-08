use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct Almanac {
    values: Vec<Value>,
    maps: Vec<Map>,
}

impl Almanac {
    const SEEDS_REGEX: &'static str = r"seeds: ([\d ]+)";
    const MAP_REGEX: &'static str = r"^(\d+) (\d+) (\d+)$";

    pub fn from1(s: &str) -> anyhow::Result<Self> {
        let numbers = Self::parse_seed_numbers(s)?;
        let values = Self::parse_values1(numbers);
        let maps = Self::parse_maps(s);
        Ok(Self { values, maps })
    }

    pub fn from2(s: &str) -> anyhow::Result<Self> {
        let numbers = Self::parse_seed_numbers(s)?;
        let values = Self::parse_values2(numbers);
        let maps = Self::parse_maps(s);
        Ok(Self { values, maps })
    }

    fn parse_seed_numbers(s: &str) -> anyhow::Result<Vec<u64>> {
        let re = Regex::new(Self::SEEDS_REGEX).unwrap();
        let captures = re.captures(s).ok_or(anyhow::anyhow!("Regex failed"))?;
        let numbers = captures.get(1).unwrap().as_str();
        Ok(numbers
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>())
    }

    fn parse_values1(numbers: Vec<u64>) -> Vec<Value> {
        numbers
            .into_iter()
            .map(|v| Value {
                kind: Kind::Seed,
                value: v,
            })
            .collect()
    }

    fn parse_values2(numbers: Vec<u64>) -> Vec<Value> {
        let mut values = Vec::new();
        for chunk in numbers.chunks_exact(2) {
            let start = chunk[0];
            let length = chunk[1];
            for v in start..(start + length) {
                values.push(Value {
                    value: v,
                    kind: Kind::Seed,
                })
            }
        }
        values
    }

    fn parse_maps(s: &str) -> Vec<Map> {
        let mut maps = Vec::new();
        let map_re = Regex::new(Self::MAP_REGEX).unwrap();
        let (mut from, mut to) = (Kind::Seed, Kind::Seed);

        for line in s.lines().filter(|s| !s.is_empty()) {
            (from, to) = match line.trim() {
                "seed-to-soil map:" => (Kind::Seed, Kind::Soil),
                "soil-to-fertilizer map:" => (Kind::Soil, Kind::Fertilizer),
                "fertilizer-to-water map:" => (Kind::Fertilizer, Kind::Water),
                "water-to-light map:" => (Kind::Water, Kind::Light),
                "light-to-temperature map:" => (Kind::Light, Kind::Temperature),
                "temperature-to-humidity map:" => (Kind::Temperature, Kind::Humidity),
                "humidity-to-location map:" => (Kind::Humidity, Kind::Location),
                _ => {
                    let captures = map_re.captures(line);
                    let captures = match captures {
                        Some(captures) => captures,
                        None => continue,
                    };
                    let dest_start = captures.get(1).unwrap().as_str().parse().unwrap();
                    let source_start = captures.get(2).unwrap().as_str().parse().unwrap();
                    let length = captures.get(3).unwrap().as_str().parse().unwrap();

                    maps.push(Map {
                        dest_start,
                        source_start,
                        length,
                        from,
                        to,
                    });

                    (from, to)
                }
            };
        }
        maps
    }

    pub fn process(&mut self) {
        for v in &mut self.values {
            v.process(&self.maps);
        }
    }

    pub fn process_times(&mut self, times: usize) {
        for i in 0..times {
            self.process()
        }
    }

    pub fn smallest_number(&self) -> Option<u64> {
        self.values.iter().map(|v| v.value).min()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Value {
    kind: Kind,
    value: u64,
}

impl Value {
    fn process(&mut self, maps: &[Map]) {
        let maps: Vec<&Map> = maps.iter().filter(|m| m.from == self.kind).collect();
        if !maps.is_empty() {
            self.kind = maps[0].to;
            for m in maps {
                if (m.source_start..(m.source_start + m.length)).contains(&self.value) {
                    self.value += m.dest_start;
                    self.value -= m.source_start;
                    break;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Kind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    dest_start: u64,
    source_start: u64,
    length: u64,
    from: Kind,
    to: Kind,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "seeds: 79 14 55 13
seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn almanac() {
        let almanac = Almanac::from1(TEST_INPUT).unwrap();
        assert_eq!(
            almanac.values,
            vec![
                Value {
                    kind: Kind::Seed,
                    value: 79
                },
                Value {
                    kind: Kind::Seed,
                    value: 14,
                },
                Value {
                    kind: Kind::Seed,
                    value: 55,
                },
                Value {
                    kind: Kind::Seed,
                    value: 13,
                }
            ]
        );
        assert_eq!(almanac.maps.len(), 18);
    }

    #[test]
    fn locations() {
        let mut almanac = Almanac::from1(TEST_INPUT).unwrap();
        almanac.process_times(10);
        assert_eq!(almanac.smallest_number().unwrap(), 35);
    }
}
