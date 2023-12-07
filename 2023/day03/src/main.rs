use std::fs::read_to_string;

use engine::Engine;
mod engine;

fn main() -> anyhow::Result<()> {
    let s = read_to_string("input.txt")?;

    let engine = Engine::new(s);

    let sum = engine.part_numbers_sum();
    println!("Sum of part numbers: {}", sum);

    let sum = engine.gear_ratio_sum();
    println!("Sum of gear ratios: {}", sum);

    Ok(())
}
