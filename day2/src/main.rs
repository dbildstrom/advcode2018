use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

#[macro_export]
macro_rules! string {
    ( $( $x:expr )* ) => {
        $(
            String::from($x)
        )*
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing filename");
        process::exit(1);
    }
    let input = fs::read_to_string(&args[1]).unwrap();
    let strings = parse_strings(&input);
    part1(&strings);
    part2(&strings);
}

fn part1(strings: &Vec<String>) {
    let (twos, threes) = count_strings(&strings);
    let checksum = calc_checksum(&twos, &threes);
    println!("part1 {}", checksum);
}

fn part2(strings: &Vec<String>) {
    let id = find_id(&strings);
    println!("part2 {}", id);
}

fn find_id(strings: &Vec<String>) -> String {
    for (idx, item) in strings.iter().enumerate() {
        for j in idx..strings.len() {
            let item2 = strings.get(j).unwrap();
            let feh = compare_id(item, item2);
            if feh != None {
                let mut id = item.clone();
                id.remove(feh.unwrap());
                return id;
            }
        }
    }
    string!("")
}

#[test]
fn find_id_test_1() {
    let strings = vec![string!("abcde"), string!("ieukg")];
    let id = find_id(&strings);
    assert_eq!(id, string!(""));
}

#[test]
fn find_id_test_2() {
    let strings = vec![string!("abcde"), string!("ieukg"), string!("abfde")];
    let id = find_id(&strings);
    assert_eq!(id, string!("abde"));
}

fn parse_strings(input: &String) -> Vec<String> {
    input.trim().lines().map(|s| string!(s)).collect()
}

fn count_strings(strings: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    for s in strings.iter() {
        char_count(s, &mut twos, &mut threes);
    }
    (twos, threes)
}

fn calc_checksum(twos: &Vec<i32>, threes: &Vec<i32>) -> i32 {
    let x: i32 = twos.iter().sum();
    let y: i32 = threes.iter().sum();
    x * y
}

fn char_count(input: &String, twos: &mut Vec<i32>, threes: &mut Vec<i32>) {
    let mut chars: HashMap<char, i32> = HashMap::new();
    for c in input.chars() {
        // if !chars.contains_key(&c) {
        //     chars.insert(c, 1);
        // } else {
        //     chars[&c] += 1;
        // }
        let stat = chars.entry(c).or_insert(0);
        *stat += 1;
    }
    let values: Vec<i32> = chars.values().cloned().collect();
    if values.contains(&2) {
        twos.push(1);
    } else {
        twos.push(0);
    }
    if values.contains(&3) {
        threes.push(1);
    } else {
        threes.push(0);
    }
}

#[test]
fn calc_checksum_test1() {
    assert_eq!(calc_checksum(&vec![0, 1, 0, 1], &vec![1, 1, 1, 0]), 6);
}

#[test]
fn calc_checksum_test2() {
    assert_eq!(calc_checksum(&vec![0, 1, 1, 1], &vec![1, 1, 1, 1]), 12);
}

#[test]
fn count_strings_test1() {
    let (twos, threes) = count_strings(&vec![string!("abbdef"), string!("aabb"), string!("aaabb")]);
    assert_eq!(twos.as_slice(), [1, 1, 1]);
    assert_eq!(threes.as_slice(), [0, 0, 1]);
}

#[test]
fn char_count_test1() {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    let s = string!("abcdef");
    char_count(&s, &mut twos, &mut threes);
    assert_eq!(twos.as_slice(), [0]);
    assert_eq!(threes.as_slice(), [0]);
}
#[test]
fn char_count_test2() {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    let s = string!("abbdef");
    char_count(&s, &mut twos, &mut threes);
    assert_eq!(twos.as_slice(), [1]);
    assert_eq!(threes.as_slice(), [0]);
}
#[test]
fn char_count_test3() {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    let s = string!("abcccdef");
    char_count(&s, &mut twos, &mut threes);
    assert_eq!(twos.as_slice(), [0]);
    assert_eq!(threes.as_slice(), [1]);
}
#[test]
fn char_count_test4() {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    let s = string!("abbcccdef");
    char_count(&s, &mut twos, &mut threes);
    assert_eq!(twos.as_slice(), [1]);
    assert_eq!(threes.as_slice(), [1]);
}
#[test]
fn char_count_test5() {
    let mut twos: Vec<i32> = Vec::new();
    let mut threes: Vec<i32> = Vec::new();
    let s = string!("abbccdddeeef");
    char_count(&s, &mut twos, &mut threes);
    assert_eq!(twos.as_slice(), [1]);
    assert_eq!(threes.as_slice(), [1]);
}

fn compare_id(id1: &String, id2: &String) -> Option<usize> {
    if id1.len() != id2.len() {
        return None;
    }
    let mut diff_index: Option<usize> = None;
    let mut index: usize = 0;
    // let mut diffIndex: i32 = -1;
    let length = id1.len();
    while index < length {
        if id1.get(index..index + 1) != id2.get(index..index + 1) {
            if diff_index != None {
                return None;
            }
            diff_index = Some(index);
        }
        index += 1;
    }
    diff_index
}
#[test]
fn compare_id_test_1() {
    assert_eq!(compare_id(&string!("abcdef"), &string!("ghijkl")), None);
}

#[test]
fn compare_id_test_2() {
    assert_eq!(compare_id(&string!("abc"), &string!("abcd")), None);
}
#[test]
fn compare_id_test_3() {
    assert_eq!(compare_id(&string!(""), &string!("")), None);
}
#[test]
fn compare_id_test_4() {
    assert_eq!(compare_id(&string!("abc"), &string!("abc")), None);
}
#[test]
fn compare_id_test_5() {
    assert_eq!(compare_id(&string!("abc"), &string!("adc")), Some(1));
}
#[test]
fn compare_id_test_6() {
    assert_eq!(compare_id(&string!("abcd"), &string!("addd")), None);
}
