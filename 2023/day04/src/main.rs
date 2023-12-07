use std::fs::read_to_string;

use cards::Cards;
mod cards;

fn main() -> anyhow::Result<()> {
    let s = read_to_string("input.txt")?;

    let mut cards = Cards::try_from(s.as_str())?;
    println!("Total points: {}", cards.points());

    cards.process();
    println!("Total count post-process: {}", cards.count());

    Ok(())
}
