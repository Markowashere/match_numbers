# match_numbers
Some number matching functions done in Rust as practice

Three different functions with same purpose: Take vector and target integer as inputs, find pair of numbers in the vector that add up to the target integer.

- find_pair_loop: go through vector using 2 for loops. O(n^2)

- find_pair_sort: sort vector (just default rust sort), use left and right iterator to go through number pairs. O(n log n)

- find_pair_map: Go through vector once, add "value" : "target - value" pairs to hashmap, lookup matching integers as you go. O(n)

unit tests: cargo test

benchmark speed: cargo bench
