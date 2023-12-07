use std::convert::TryFrom;

use regex::Regex;

#[derive(Debug)]
pub struct Cards(Vec<CardCopies>);

impl TryFrom<&str> for Cards {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let copies = s
            .lines()
            .filter(|s| !s.is_empty())
            .map(Card::try_from)
            .map(|c| c.map(CardCopies::new))
            .collect::<Result<Vec<CardCopies>, Self::Error>>()?;
        Ok(Self(copies))
    }
}

impl Cards {
    pub fn points(&self) -> u64 {
        self.0.iter().map(|copy| copy.card.points()).sum()
    }

    pub fn process(&mut self) {
        for i in 0..self.0.len() {
            let copy = &self.0[i];
            let copies = copy.copies;
            let matching = copy.card.matching();

            for j in i..(i + matching) {
                if let Some(copy_mut) = self.0.get_mut(j + 1) {
                    copy_mut.copies += copies;
                }
            }
        }
    }

    pub fn count(&self) -> u64 {
        self.0.iter().map(|copy| copy.copies).sum()
    }
}

#[derive(Debug)]
struct CardCopies {
    card: Card,
    copies: u64,
}

impl CardCopies {
    fn new(card: Card) -> Self {
        Self { card, copies: 1 }
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    id: u64,
    winning: Vec<u64>,
    selected: Vec<u64>,
}

impl Card {
    const REGEX: &'static str = r"Card\s+(\d+): ([\d ]*)\|([\d ]*)";

    fn collect_str(s: &str) -> Vec<u64> {
        s.split(' ')
            .filter(|s| !s.is_empty())
            .map(|d| d.parse().unwrap())
            .collect()
    }

    fn matching(&self) -> usize {
        self.selected
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn points(&self) -> u64 {
        let winners = self.matching() as u32;
        match winners {
            0 => 0,
            n => 2u64.pow(n - 1),
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(Self::REGEX).unwrap();
        let captures = re.captures(s).ok_or(anyhow::anyhow!("Regex failed"))?;
        let id = captures.get(1).unwrap().as_str().parse()?;
        let winning_str = captures.get(2).unwrap().as_str();
        let selected_str = captures.get(3).unwrap().as_str();

        let winning = Self::collect_str(winning_str);
        let selected = Self::collect_str(selected_str);

        Ok(Self {
            id,
            winning,
            selected,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_str() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    }

    #[test]
    fn test_from_str() {
        let s = test_str();
        for line in s.lines() {
            let _ = Card::try_from(line).unwrap();
        }

        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::try_from(s).unwrap();
        assert_eq!(
            card,
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                selected: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        )
    }

    #[test]
    fn points() {
        let s = test_str();
        let cards = Cards::try_from(s).unwrap();
        assert_eq!(13, cards.points());
    }

    #[test]
    fn count_post_process() {
        let s = test_str();
        let mut cards = Cards::try_from(s).unwrap();
        cards.process();
        assert_eq!(30, cards.count());
    }
}
