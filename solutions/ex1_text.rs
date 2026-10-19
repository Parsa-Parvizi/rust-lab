use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in text.split(|c: char| !c.is_alphabetic()).filter(|w| !w.is_empty()) {
        *m.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    m
}

pub fn longest_word(text: &str) -> Option<&str> {
    let mut best: Option<&str> = None;
    for w in text.split(|c: char| !c.is_alphabetic()).filter(|w| !w.is_empty()) {
        match best {
            Some(b) if b.chars().count() >= w.chars().count() => {}
            _ => best = Some(w),
        }
    }
    best
}

pub fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut v: Vec<(String, usize)> = freq.iter().map(|(k, c)| (k.clone(), *c)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(n);
    v
}
