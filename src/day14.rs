#[allow(dead_code)]
mod day10;


fn part1(key: &str) -> usize {
    (0..128).map(|n| {
        let mut ring = day10::Ring::new();
        ring.hash(&format!("{}-{}", key, n));
        ring.result()
    }).map::<usize, _>(|row|
        row.iter().map(|b| b.count_ones() as usize).sum()
    ).sum()
}


fn main() {
    const INPUT: &str = "hfdlxzhv";
    println!("Used squares: {}", part1(INPUT));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        const INPUT: &str = "flqrgnkx";
        assert_eq!(part1(INPUT), 8108);
    }
}
