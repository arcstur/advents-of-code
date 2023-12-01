use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Trebuchet {
    values: Vec<u64>,
}

impl Trebuchet {
    pub fn new(values: Vec<u64>) -> Self {
        Self { values }
    }

    pub fn sum(&self) -> u64 {
        self.values.iter().sum()
    }

    pub fn from_str1(s: &str) -> anyhow::Result<Self> {
        let first = Regex::new(r"^.*?(\d)").unwrap();
        let second = Regex::new(r"^.*(\d)").unwrap();
        Self::from_str_with_regex(s, first, second)
    }

    pub fn from_str2(s: &str) -> anyhow::Result<Self> {
        let first = Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let second = Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        let mut values = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            let first_digit = first
                .captures(line)
                .ok_or(anyhow::anyhow!("No first digit found"))?
                .get(1)
                .unwrap()
                .as_str();
            let first_digit = Self::parse_str_digit(first_digit)?;
            let second_digit = second
                .captures(line)
                .ok_or(anyhow::anyhow!("No second digit found"))?
                .get(1)
                .unwrap()
                .as_str();
            let second_digit = Self::parse_str_digit(second_digit)?;
            let value = first_digit * 10 + second_digit;
            values.push(value);
        }
        Ok(Self::new(values))
    }

    fn parse_str_digit(digit: &str) -> anyhow::Result<u64> {
        let digit = match digit {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => digit,
        };
        Ok(digit.parse::<u64>()?)
    }

    fn from_str_with_regex(s: &str, first: Regex, second: Regex) -> anyhow::Result<Self> {
        let mut values = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            let first_digit = first
                .captures(line)
                .ok_or(anyhow::anyhow!("No first digit found"))?
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()?;
            let second_digit = second
                .captures(line)
                .ok_or(anyhow::anyhow!("No second digit found"))?
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()?;
            let value = first_digit * 10 + second_digit;
            values.push(value);
        }
        Ok(Self::new(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        let trebuchet = Trebuchet::from_str1(s).unwrap();
        assert_eq!(
            trebuchet,
            Trebuchet {
                values: vec![12, 38, 15, 77]
            }
        )
    }

    #[test]
    fn test_from_str2() {
        let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let trebuchet = Trebuchet::from_str2(s).unwrap();
        assert_eq!(
            trebuchet,
            Trebuchet {
                values: vec![29, 83, 13, 24, 42, 14, 76]
            }
        )
    }
}
