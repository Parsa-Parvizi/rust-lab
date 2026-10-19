//! تمرین ۵: Generic + Trait Bound + کش LRU
//! پیاده‌سازی ساده با HashMap + VecDeque (O(n) برای touch؛ قابل قبول برای یادگیری).
//! چالش بعدی: با پیاده‌سازی O(1) (لیست پیوندی/ایندکس‌ها) جایگزینش کن.
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>, // جلو = قدیمی‌ترین ، عقب = تازه‌ترین
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    /// اگر capacity == 0 باشد panic کن (assert!)
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// خواندن، کلید را «تازه» می‌کند.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// درج/به‌روزرسانی. اگر کش پر بود، قدیمی‌ترین کلید حذف و برگردانده می‌شود.
    /// به‌روزرسانی کلید موجود → eviction ندارد و None برمی‌گرداند.
    pub fn put(&mut self, key: K, value: V) -> Option<K> {
        todo!()
    }

    pub fn contains(&self, key: &K) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}
