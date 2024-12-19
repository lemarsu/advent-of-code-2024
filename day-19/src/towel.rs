use std::collections::HashMap;

fn match_pattern(word: &[char], pattern: &[char]) -> bool {
    if word.len() == 0 {
        return true;
    }
    if word.len() < pattern.len() || pattern[0] != word[0] {
        return false;
    }
    (0..pattern.len()).all(|i| word[i] == pattern[i])
}

pub fn can_make_onsen_flag(word: &[char], patterns: &[Vec<char>]) -> bool {
    for pattern in patterns {
        if match_pattern(word, pattern) {
            let idx = pattern.len();
            if idx == word.len() || can_make_onsen_flag(&word[idx..], patterns) {
                return true;
            }
        }
    }
    false
}

pub fn all_onsen_flag(word: &[char], patterns: &[Vec<char>], cache: &mut HashMap<(Vec<char>, Vec<Vec<char>>), usize>) -> usize {
    let key = (word.to_vec(), patterns.to_vec());
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut ret = 0;
    for pattern in patterns {
        if match_pattern(word, pattern) {
            let idx = pattern.len();
            if idx == word.len() {
                ret += 1;
                continue;
            }
            ret += all_onsen_flag(&word[idx..], patterns, cache);
        }
    }
    cache.insert(key, ret);
    ret
}
