impl TokenBucket {
    pub fn new(capacity: u32, refill_per_sec: u32, now_ms: u64) -> Self {
        TokenBucket { capacity, refill_per_sec, tokens: capacity as f64, last_ms: now_ms }
    }

    fn refill(&mut self, now_ms: u64) {
        let elapsed_s = now_ms.saturating_sub(self.last_ms) as f64 / 1000.0;
        self.tokens = (self.tokens + elapsed_s * self.refill_per_sec as f64).min(self.capacity as f64);
        self.last_ms = self.last_ms.max(now_ms);
    }

    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        self.refill(now_ms);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn available(&mut self, now_ms: u64) -> u32 {
        self.refill(now_ms);
        self.tokens.floor() as u32
    }
}
