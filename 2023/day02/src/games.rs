use regex::Regex;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct Games {
    games: Vec<Game>,
}

impl TryFrom<&str> for Games {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let games = s
            .lines()
            .map(Game::try_from)
            .collect::<Result<Vec<Game>, Self::Error>>()?;
        Ok(Self { games })
    }
}

impl Games {
    pub fn filter_possible(self, max_set: &Set) -> Self {
        let games = self
            .games
            .into_iter()
            .filter(|g| g.is_possible(max_set))
            .collect();
        Self { games }
    }

    pub fn id_sum(&self) -> u64 {
        self.games.iter().map(|g| g.id).sum()
    }

    pub fn power_sum_of_min_sets(&self) -> u64 {
        let min_sets: Vec<Set> = self.games.iter().map(|g| g.minimum_set()).collect();
        min_sets.iter().map(|s| s.power()).sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    id: u64,
    sets: Vec<Set>,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"Game (\d+): (.*)")?;
        let caps = re.captures(s).ok_or(anyhow::anyhow!("Failed parsing."))?;

        let id = caps[1].parse().unwrap();
        let sets = caps[2]
            .split(';')
            .map(Set::try_from)
            .collect::<Result<Vec<Set>, Self::Error>>()?;

        Ok(Self { id, sets })
    }
}

impl Game {
    fn is_possible(&self, max_set: &Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(max_set))
    }

    fn minimum_set(&self) -> Set {
        let mut min_set = Set::default();

        for set in &self.sets {
            min_set.red = min_set.red.max(set.red);
            min_set.blue = min_set.blue.max(set.blue);
            min_set.green = min_set.green.max(set.green);
        }

        min_set
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Set {
    pub red: u64,
    pub blue: u64,
    pub green: u64,
}

impl TryFrom<&str> for Set {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re_green = Regex::new(r"(\d+) green")?;
        let re_blue = Regex::new(r"(\d+) blue")?;
        let re_red = Regex::new(r"(\d+) red")?;

        let red = match re_red.captures(s) {
            Some(caps) => caps[1].parse().unwrap(),
            None => 0,
        };
        let blue = match re_blue.captures(s) {
            Some(caps) => caps[1].parse().unwrap(),
            None => 0,
        };
        let green = match re_green.captures(s) {
            Some(caps) => caps[1].parse().unwrap(),
            None => 0,
        };
        Ok(Set { red, blue, green })
    }
}

impl Set {
    fn is_possible(&self, max_set: &Self) -> bool {
        self.red <= max_set.red && self.blue <= max_set.blue && self.green <= max_set.green
    }

    fn power(&self) -> u64 {
        self.red * self.blue * self.green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_from_str() {
        let s = "1 red, 2 green, 6 blue";
        let set = Set::try_from(s).unwrap();
        assert_eq!(
            set,
            Set {
                red: 1,
                blue: 6,
                green: 2,
            }
        );

        let s = "3 blue, 12 red";
        let set = Set::try_from(s).unwrap();
        assert_eq!(
            set,
            Set {
                red: 12,
                blue: 3,
                green: 0,
            }
        );

        let s = "2 green";
        let set = Set::try_from(s).unwrap();
        assert_eq!(
            set,
            Set {
                red: 0,
                blue: 0,
                green: 2,
            }
        );
    }

    #[test]
    fn test_games_from_str() {
        let s = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let games = Games::try_from(s).unwrap();
        assert_eq!(
            games,
            Games {
                games: vec![
                    Game {
                        id: 1,
                        sets: vec![
                            Set {
                                blue: 3,
                                red: 4,
                                green: 0,
                            },
                            Set {
                                blue: 6,
                                red: 1,
                                green: 2,
                            },
                            Set {
                                blue: 0,
                                red: 0,
                                green: 2,
                            },
                        ]
                    },
                    Game {
                        id: 2,
                        sets: vec![
                            Set {
                                blue: 1,
                                red: 0,
                                green: 2,
                            },
                            Set {
                                blue: 4,
                                red: 1,
                                green: 3,
                            },
                            Set {
                                blue: 1,
                                red: 0,
                                green: 1,
                            },
                        ]
                    },
                    Game {
                        id: 3,
                        sets: vec![
                            Set {
                                blue: 6,
                                red: 20,
                                green: 8,
                            },
                            Set {
                                blue: 5,
                                red: 4,
                                green: 13,
                            },
                            Set {
                                blue: 0,
                                red: 1,
                                green: 5,
                            },
                        ]
                    },
                ]
            }
        )
    }
}
