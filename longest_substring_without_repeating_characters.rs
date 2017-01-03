use std::collections::HashMap;

fn length_of_longest_substring(s: &str) -> (usize, usize, usize) {
    let mut map = HashMap::new();
    let (mut max, mut start, mut a, mut b) = (0, 0, 0, 0);
    for (i, c) in s.chars().enumerate() {
        if map.contains_key(&c) {
            let prev = *map.get(&c).unwrap();
            let new = i - start;
            if new > max {
                max = new;
                a = start;
                b = i;
            }
            start = prev + 1;
            map.remove(&c);
        }
        map.insert(c, i);
    }
    let i = s.len();
    if i - start > max {
        max = i - start;
        a = start;
        b = i;
    }
    return (a, b, max);
}

fn main()
{
    let ans = length_of_longest_substring("pwwkew");
    println!("{} - {}, {}", ans.0, ans.1, ans.2);
}
