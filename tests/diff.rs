use std::time::Instant;

use rdiff::diff::diff;

// Run with cargo test --test diff -- --nocapture

const TARGET_LINE: &str =
    "Hello Rust! This line will be use for generate large collections of benchmarks.";

#[test]
fn test_diff_size_100() {
    let now = Instant::now();
    let items = vec![TARGET_LINE; 100];
    diff(&items, &items);
    let elapsed = now.elapsed();
    println!("Size 100: {:.2?}", elapsed);
}

#[test]
fn test_diff_size_1000() {
    let now = Instant::now();
    let items = vec![TARGET_LINE; 1000];
    diff(&items, &items);
    let elapsed = now.elapsed();
    println!("Size 1000: {:.2?}", elapsed);
}

#[test]
fn test_diff_size_10000() {
    let now = Instant::now();
    let items = vec![TARGET_LINE; 10000];
    diff(&items, &items);
    let elapsed = now.elapsed();
    println!("Size 10000: {:.2?}", elapsed);
}
