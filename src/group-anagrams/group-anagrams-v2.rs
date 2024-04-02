use std::{collections::HashMap, vec};

fn hash(str: &str) -> Vec<u8> {
    let mut res = str.chars().fold(vec![], |mut prev, cur| {
        let mut bytes = [0u8; 1];
        cur.encode_utf8(&mut bytes);
        prev.push(bytes[0]);
        prev
    });
    res.sort();
    res
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result = HashMap::new();
        for str in strs {
            let mut key = hash(&str);
            let mut res_arr = result.entry(key).or_insert(vec![]);
            res_arr.push(str);
        }
        result.into_values().into_iter().collect()
    }
}
