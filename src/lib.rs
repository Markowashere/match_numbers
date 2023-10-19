use rand::seq::SliceRandom;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn find_pair_loop(source: &Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    for (i, num1) in source.iter().enumerate() {
        for (_, num2) in source[i..].iter().enumerate() {
            if num1 + num2 == *target {
                return Some([*num1, *num2]);
            }
        }
    }
    None
}

pub fn find_pair_sort(source: &Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    let mut sorted_source = source.clone();
    sorted_source.sort();
    let mut left = 0;
    let mut right = sorted_source.len() - 1;

    while left < right {
        let sum = sorted_source[left] + sorted_source[right];
        match sum.cmp(target) {
            Ordering::Equal => return Some([sorted_source[left], sorted_source[right]]),
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
        }
    }
    None
}

pub fn find_pair_map(source: &Vec<i32>, target: &i32) -> Option<[i32; 2]> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in source {
        let comp = target - num;
        let search = map.get(num);

        match search {
            Some(&_) => return Some([comp, *num]),
            None => {
                map.insert(comp, *num);
            }
        }
    }
    None
}

pub fn _generate_vec(n: i32, num1: i32, num2: i32, seed: u64) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn find_pair_works() {
        use super::*;
        let vec = vec![1, 12, 34, 534, 35, 9];
        let target = 10;
        let ans = find_pair_loop(&vec, &target);
        assert_eq!(ans, Some([1, 9]));
    }

    #[test]
    fn find_pair_i_works() {
        use super::*;
        let vec = vec![1, 12, 34, 534, 35, 9];
        let target = 10;
        let ans = find_pair_sort(&vec, &target);
        assert_eq!(ans, Some([1, 9]));
    }

    #[test]
    fn find_pair_map_works() {
        use super::*;
        let vec = vec![1, 12, 34, 534, 35, 9];
        let target = 10;
        let ans = find_pair_map(&vec, &target);
        assert_eq!(ans, Some([1, 9]));
    }
}
