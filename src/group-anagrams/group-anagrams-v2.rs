use std::{collections::HashMap, vec};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let primes = HashMap::from([
            ('a', 2),
            ('b', 3),
            ('c', 5),
            ('d', 7),
            ('e', 11),
            ('f', 13),
            ('g', 17),
            ('h', 19),
            ('i', 23),
            ('j', 29),
            ('k', 31),
            ('l', 37),
            ('m', 41),
            ('n', 43),
            ('o', 47),
            ('p', 53),
            ('q', 59),
            ('r', 61),
            ('s', 67),
            ('t', 71),
            ('u', 73),
            ('v', 79),
            ('w', 83),
            ('x', 89),
            ('y', 97),
            ('z', 101),
        ]);

        let mut result = HashMap::new();
        for str in strs {
            let mut x = 1;
            for s in str.chars() {
                x *= primes.get(&s).expect("cannot find target alphabet");
                // println!("{}", x);
            }
            let mut res_arr = result.entry(x).or_insert(vec![]);
            res_arr.push(str);
            // println!("====================");
        }
        // println!("{:?}", result);
        result
            .into_values()
            .into_iter()
            .collect::<Vec<Vec<String>>>()
    }
}
