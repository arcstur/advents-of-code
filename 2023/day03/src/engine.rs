use regex::Regex;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Engine {
    schematic: String,
}

impl Engine {
    const SYMBOL_REGEX: &'static str = r"[^.\d\n]+";
    const NUMBER_REGEX: &'static str = r"\d+";
    const GEAR_REGEX: &'static str = r"\*";

    pub fn new(schematic: String) -> Engine {
        Self { schematic }
    }

    pub fn part_numbers_sum(&self) -> u64 {
        self.part_numbers().iter().sum()
    }

    fn part_numbers(&self) -> Vec<u64> {
        let mut final_numbers = Vec::new();
        let mut last_matches: Vec<Range<usize>> = Vec::new();
        let mut last_possible_numbers: Vec<Number> = Vec::new();

        for line in self.schematic.lines() {
            let matches = Self::symbol_matches(line);
            let mut possible_numbers = Vec::new();

            for n in Self::numbers(line) {
                match n.is_adjacent_list(&matches) {
                    true => final_numbers.push(n.number),
                    false => possible_numbers.push(n),
                }
            }
            final_numbers.extend(
                last_possible_numbers
                    .into_iter()
                    .filter(|n| n.is_adjacent_list(&matches))
                    .map(|n| n.number),
            );

            last_possible_numbers = Vec::new();

            for n in &possible_numbers {
                match n.is_adjacent_list(&last_matches) {
                    true => final_numbers.push(n.number),
                    false => last_possible_numbers.push(n.clone()),
                }
            }

            last_matches = matches;
        }

        final_numbers
    }

    fn symbol_matches(line: &str) -> Vec<Range<usize>> {
        let re = Regex::new(Self::SYMBOL_REGEX).unwrap();
        re.find_iter(line).map(|m| m.range()).collect()
    }

    fn numbers(line: &str) -> Vec<Number> {
        let re = Regex::new(Self::NUMBER_REGEX).unwrap();
        re.find_iter(line)
            .map(|m| Number::new(m.as_str().parse().expect("Regex matched digits"), m.range()))
            .collect()
    }

    pub fn gear_ratio_sum(&self) -> u64 {
        self.gears().iter().map(|g| g.ratio()).sum()
    }

    fn gears(&self) -> Vec<Gear> {
        let mut final_gears = Vec::new();
        let mut last_numbers: Vec<Number> = Vec::new();
        let mut last_possible_gears: Vec<PossibleGear> = Vec::new();

        for line in self.schematic.lines() {
            let numbers = Self::numbers(line);
            let mut possible_gears = Self::possible_gears(line);

            for n in &numbers {
                for g in &mut possible_gears {
                    if g.is_adjacent(n) {
                        g.adjacents.push(n.number);
                    }
                }
                for g in &mut last_possible_gears {
                    if g.is_adjacent(n) {
                        g.adjacents.push(n.number);
                    }
                }
            }

            final_gears.extend(last_possible_gears);

            for g in &mut possible_gears {
                for n in &last_numbers {
                    if g.is_adjacent(n) {
                        g.adjacents.push(n.number);
                    }
                }
            }

            last_numbers = numbers;
            last_possible_gears = possible_gears
        }

        final_gears
            .into_iter()
            .filter_map(|g| g.into_gear())
            .collect()
    }

    fn possible_gears(line: &str) -> Vec<PossibleGear> {
        let re = Regex::new(Self::GEAR_REGEX).unwrap();
        re.find_iter(line)
            .map(|m| PossibleGear::new(m.start()))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Number {
    number: u64,
    range: Range<usize>,
}

impl Number {
    fn new(number: u64, range: Range<usize>) -> Number {
        Self { number, range }
    }

    fn conflict_range(&self) -> Range<usize> {
        let start = match self.range.start {
            0 => 0,
            n => n - 1,
        };
        let end = self.range.end + 1;
        start..end
    }

    fn is_adjacent(&self, symbol_match: &Range<usize>) -> bool {
        // Number is adjacent when the symbol range
        // intersects the Numbers' bounding box (#123#).
        // Or the tail of the symbol hits the numbers' box,
        // or the start.
        let conflict_range = self.conflict_range();
        let end_is_contained = conflict_range.contains(&(symbol_match.end - 1));
        let start_is_contained = conflict_range.contains(&symbol_match.start);
        end_is_contained || start_is_contained
    }

    fn is_adjacent_list(&self, symbol_matches: &[Range<usize>]) -> bool {
        symbol_matches.iter().any(|m| self.is_adjacent(m))
    }
}

#[derive(Debug, Clone)]
struct PossibleGear {
    index: usize,
    adjacents: Vec<u64>,
}

impl PossibleGear {
    fn new(index: usize) -> Self {
        Self {
            index,
            adjacents: Vec::new(),
        }
    }

    fn adjacent_count(&self) -> usize {
        self.adjacents.len()
    }

    fn into_gear(self) -> Option<Gear> {
        match self.adjacent_count() {
            2 => Some(Gear {
                adjacent1: self.adjacents[0],
                adjacent2: self.adjacents[1],
            }),
            _ => None,
        }
    }

    fn is_adjacent(&self, number: &Number) -> bool {
        number.is_adjacent(&(self.index..(self.index + 1)))
    }
}

#[derive(Debug, Clone)]
struct Gear {
    adjacent1: u64,
    adjacent2: u64,
}

impl Gear {
    fn ratio(&self) -> u64 {
        self.adjacent1 * self.adjacent2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_engine() -> Engine {
        let s = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        Engine::new(s.to_string())
    }

    #[test]
    fn part_numbers() {
        let e = test_engine();
        let numbers = vec![467, 35, 633, 617, 592, 755, 664, 598];
        assert_eq!(numbers, e.part_numbers());
    }

    #[test]
    fn part_numbers_sum() {
        let e = test_engine();
        assert_eq!(4361, e.part_numbers_sum());
    }

    #[test]
    fn test_regex() {
        let re = Regex::new(Engine::SYMBOL_REGEX).unwrap();
        let s = "..123.*..##.22*..
...&..@../...
.............";
        let matches: Vec<_> = re.find_iter(s).map(|m| m.range()).collect();
        assert_eq!(matches, vec![6..7, 9..11, 14..15, 21..22, 24..25, 27..28]);
    }

    #[test]
    fn gear_ratio_sum() {
        let e = test_engine();
        assert_eq!(467835, e.gear_ratio_sum());
    }
}
