use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    let mut map = HashMap::new();
    for (i, x) in nums.iter().enumerate() {
        let df = target - x;
        if map.contains_key(&df) {
            return (*map.get(&df).unwrap(), i);
        }
        map.insert(x, i);
    }
    return (0, 0);
}

fn main() {
    let ret = two_sum(&vec![1,2,3], 4);
    println!("{}, {}", ret.0, ret.1);
}

