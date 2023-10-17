use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // bucket for each string
    // bucket result as key, string as value in map
    // return map values as vec![vec!]

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        // strs[i].length <= 100, u8 = 256 is enough
        let mut count_s = vec![0u8; 26];
        s.chars().for_each(|c| {
            count_s[((c as u8) - ('a' as u8)) as usize] += 1;
        });

        let key = String::from_utf8(count_s).unwrap();

        // if key exist, mut the vec!
        let x = map.get_mut(&key);

        if let Some(x) = x {
            x.append(&mut vec![s]);
        } else {
            map.insert(key, vec![s]);
        }
    }

    let ans = map.into_values().collect();

    ans
}

fn main() {
    println!(
        "{:?}",
        group_anagrams(
            ["eat", "tea", "tan", "ate", "nat", "bat"]
                .map(String::from)
                .to_vec()
        )
    );
}
