use match_numbers::{find_pair, find_pair_i, find_pair_map, _generate_vec};

fn main() {
    let my_vec = _generate_vec(10000, 5643, 967, 40);
    let my_vec_2 = my_vec.clone();
    let my_vec_3 = my_vec.clone();
    let target: i32 = 6610;

    if let Some(ans) = find_pair(&my_vec, &target) {
        println!("number one: {}, number two: {}", ans[0], ans[1]);
    } else {
        println!("No pair found for target in find_pair.");
    }

    if let Some(ans) = find_pair_i(&my_vec_2, &target) {
        println!("number one: {}, number two: {}", ans[0], ans[1]);
    } else {
        println!("No pair found for target in find_pair_i.");
    }

    if let Some(ans) = find_pair_map(&my_vec_3, &target) {
        println!("number one: {}, number two: {}", ans[0], ans[1]);
    } else {
        println!("No pair found for target in find_pair_i.");
    }
}
