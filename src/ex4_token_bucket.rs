//! تمرین ۴: Rate Limiter با الگوریتم Token Bucket
//! زمان را از بیرون می‌گیریم (now_ms) تا تست‌ها deterministic باشند؛ این یک الگوی طراحی مهم است.

pub struct TokenBucket {
    capacity: u32,
    refill_per_sec: u32,
    tokens: f64,
    last_ms: u64,
}

impl TokenBucket {
    /// سطل در ابتدا پر است.
    pub fn new(capacity: u32, refill_per_sec: u32, now_ms: u64) -> Self {
        todo!()
    }

    /// ابتدا بر اساس زمان سپری‌شده شارژ کن (حداکثر تا capacity)،
    /// سپس اگر حداقل ۱ توکن بود، یکی بردار و true برگردان.
    /// نکته: now_ms ممکن است کوچک‌تر از last_ms باشد → saturating_sub
    pub fn try_acquire(&mut self, now_ms: u64) -> bool {
        todo!()
    }

    /// تعداد توکن‌های کامل موجود (کف عدد اعشاری) بعد از شارژ
    pub fn available(&mut self, now_ms: u64) -> u32 {
        todo!()
    }
}
