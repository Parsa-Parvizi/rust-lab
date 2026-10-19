impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        LruCache { capacity, map: HashMap::new(), order: VecDeque::new() }
    }

    fn touch(&mut self, key: &K) {
        if let Some(pos) = self.order.iter().position(|k| k == key) {
            self.order.remove(pos);
        }
        self.order.push_back(key.clone());
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.touch(key);
        }
        self.map.get(key)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<K> {
        if self.map.contains_key(&key) {
            self.touch(&key);
            self.map.insert(key, value);
            return None;
        }
        let mut evicted = None;
        if self.map.len() == self.capacity {
            if let Some(old) = self.order.pop_front() {
                self.map.remove(&old);
                evicted = Some(old);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
        evicted
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
