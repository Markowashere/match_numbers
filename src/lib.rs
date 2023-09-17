use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng, rngs::StdRng};

pub fn find_pair(source: &Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    for (i, num1) in source.iter().enumerate() {
        for (_, num2) in source[i..].iter().enumerate() {
            if num1 + num2 == *target {
                return Some([*num1, *num2]);
            }
        }
    }
    None
}

pub fn find_pair_i(source: &mut Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    source.sort();
    let mut i = 0;
    let mut j = source.len() - 1;

    while i < j {
        let sum = source[i] + source[j];
        if sum == *target {
            return Some([source[i], source[j]]);
        } else if sum < *target {
            i += 1;
        } else {
            j -= 1;
        }
    }
    None
}

pub fn find_pair_map(source: &Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in source {
        let comp = target - num;
        if let Some(&other_num) = map.get(&comp) {
            return Some([*num, other_num]);
        } else {
            map.insert(*num, comp);
        }
    }
    None
}

pub fn generate_vec(n: i32, num1: i32, num2: i32, seed: u64) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..n {
        vec.push(rng.gen_range(1..1000000000));
    }
    vec.push(num1);
    vec.push(num2);
    vec.shuffle(&mut rng);
    vec
}