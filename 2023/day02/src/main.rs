use std::fs::read_to_string;

use games::{Games, Set};
mod games;

fn main() -> anyhow::Result<()> {
    let s = read_to_string("input.txt")?;

    let games = Games::try_from(s.as_str())?;
    let max_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = games.filter_possible(&max_set);
    let id_sum = games.id_sum();
    println!("ID sum (possible games): {}", id_sum);

    let games = Games::try_from(s.as_str())?;
    println!(
        "Power sum of minimum sets: {}",
        games.power_sum_of_min_sets()
    );

    Ok(())
}
