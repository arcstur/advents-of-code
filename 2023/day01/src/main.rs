use std::fs::read_to_string;

use trebuchet::Trebuchet;

mod trebuchet;

fn main() -> anyhow::Result<()> {
    let s = read_to_string("input.txt")?;

    let t = Trebuchet::from_str1(&s)?;
    let sum = t.sum();
    println!("Sum 1: {}", sum);

    let t = Trebuchet::from_str2(&s)?;
    let sum = t.sum();
    println!("Sum 2: {}", sum);

    Ok(())
}
