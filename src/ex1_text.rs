//! تمرین ۱: Ownership, Borrowing, Lifetime, HashMap, Iterator
use std::collections::HashMap;

/// تعداد تکرار هر کلمه. کلمه = دنباله‌ای از حروف (alphabetic).
/// بزرگ/کوچک بودن مهم نیست (همه lowercase شوند). علائم نگارشی جداکننده‌اند.
/// راهنما: text.split(|c: char| !c.is_alphabetic()), entry().or_insert()
pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// طولانی‌ترین کلمه را به‌صورت *قرض* (بدون کپی) برگردان.
/// اگر چند کلمه هم‌طول بودند، اولی برنده است. متن بدون کلمه → None
/// فکر کن: چرا خروجی باید lifetime ورودی را داشته باشد؟
pub fn longest_word(text: &str) -> Option<&str> {
    todo!()
}

/// n کلمه‌ی پرتکرار؛ مرتب بر اساس تعداد (نزولی) و در تساوی بر اساس حروف الفبا (صعودی).
/// راهنما: sort_by + Ordering::then, truncate
pub fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    todo!()
}
