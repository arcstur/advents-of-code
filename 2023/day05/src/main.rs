use std::fs::read_to_string;

mod almanac;

fn main() -> anyhow::Result<()> {
    let s = read_to_string("input.txt")?;

    let mut almanac = almanac::Almanac::from1(&s)?;
    almanac.process_times(10);
    println!(
        "Smallest location number: {}",
        almanac.smallest_number().unwrap()
    );

    println!("Part 2 is still horribly optimized, it crashes!");

    Ok(())
}
