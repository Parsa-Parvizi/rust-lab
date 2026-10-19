use rust_lab::ex5_lru::*;

#[test]
fn put_and_get() {
    let mut c = LruCache::new(2);
    assert_eq!(c.put("a", 1), None);
    assert_eq!(c.put("b", 2), None);
    assert_eq!(c.get(&"a"), Some(&1));
    assert_eq!(c.get(&"zzz"), None);
    assert_eq!(c.len(), 2);
}

#[test]
fn evicts_least_recently_used() {
    let mut c = LruCache::new(2);
    c.put("a", 1);
    c.put("b", 2);
    c.get(&"a"); // حالا b قدیمی‌ترین است
    assert_eq!(c.put("c", 3), Some("b"));
    assert!(!c.contains(&"b"));
    assert!(c.contains(&"a"));
    assert!(c.contains(&"c"));
    assert_eq!(c.len(), 2);
}

#[test]
fn updating_existing_key_does_not_evict_and_refreshes() {
    let mut c = LruCache::new(2);
    c.put("a", 1);
    c.put("b", 2);
    assert_eq!(c.put("a", 10), None);
    assert_eq!(c.get(&"a"), Some(&10));
    // b الان قدیمی‌ترین است
    assert_eq!(c.put("c", 3), Some("b"));
}

#[test]
fn capacity_one() {
    let mut c = LruCache::new(1);
    c.put(1, "x");
    assert_eq!(c.put(2, "y"), Some(1));
    assert_eq!(c.get(&2), Some(&"y"));
}

#[test]
#[should_panic]
fn zero_capacity_panics() {
    let _c: LruCache<i32, i32> = LruCache::new(0);
}
