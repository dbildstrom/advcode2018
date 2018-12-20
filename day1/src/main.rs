use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing filename");
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    let changes = parse_changes(&input);
    println!("{} {}", freq1(&changes), freq2(&changes));
}

fn parse_changes(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn freq1(changes: &[i32]) -> i32 {
    changes.iter().sum()
}

fn freq2(changes: &[i32]) -> i32 {
    let mut freq = 0;
    let mut freqs: Vec<i32> = vec![0i32];
    loop {
        for x in changes {
            freq += x;
            if freqs.contains(&freq) {
                return freq;
            }
            freqs.push(freq);
        }
    }
}

// TESTS

#[test]
fn parse_changes_test() {
    assert_eq!(parse_changes(&"+1\n-2".to_string()), vec![1, -2]);
}

#[test]
fn freq1_test() {
    assert_eq!(freq1(&vec![1, -2]), -1);
}

#[test]
fn freq2_test_a() {
    assert_eq!(freq2(&vec![1, -1]), 0);
}

#[test]
fn freq2_test_b() {
    assert_eq!(freq2(&vec![3, 3, 4, -2, -4]), 10);
}

#[test]
fn freq2_test_c() {
    assert_eq!(freq2(&vec![-6, 3, 8, 5, -6]), 5);
}

#[test]
fn freq2_test_d() {
    assert_eq!(freq2(&vec![7, 7, -2, -7, -4]), 14);
}
