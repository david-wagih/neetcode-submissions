use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // making one hashMap and incrementing from string s and decrementing from String t so if not all settled to 0 then return false
        // can fail fast if the two strings len is not the same
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        // construct the map from String s
        for x in s.chars() {
            *map.entry(x).or_insert(0) += 1;
        }
        // decrementing what exists in t from the map to check.
        for y in t.chars() {
            *map.entry(y).or_insert(0) -= 1;
            if *map.get(&y).unwrap() < 0 {
                return false;
            }
        }

        true
    }
}