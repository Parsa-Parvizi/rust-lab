# Rust Lab — ۶ تمرین عملی با تست

## اجرا
```bash
cargo test                         # همه‌ی تست‌ها (در ابتدا همه fail می‌شوند: todo!())
cargo test --test ex1_text         # فقط تمرین ۱
cargo test --test ex1_text -- --nocapture
cargo clippy                       # بعد از سبز شدن تست‌ها، هشدارها را رفع کن
```

## روش کار
1. فایل `src/exN_*.rs` را باز کن و `todo!()` ها را پیاده‌سازی کن.
2. فایل `tests/exN_*.rs` را **اول بخوان**؛ تست‌ها مثل مستندات رفتار مورد انتظارند.
3. فقط اگر گیر کردی، `solutions/` را ببین (بدنه‌ی توابع؛ جای `todo!()` بگذار).

## مفاهیمی که یاد می‌گیری
| # | تمرین | مفاهیم |
|---|-------|--------|
| 1 | Text utils | ownership/borrowing، lifetime، HashMap، iterator، sort_by |
| 2 | Log parser | enum، struct، `Result`، custom error، `?` |
| 3 | RPN calculator | `Vec` به‌عنوان stack، `match`، error handling |
| 4 | Token bucket | struct + impl، تزریق زمان برای تست‌پذیری، `saturating_sub` |
| 5 | LRU cache | generic، trait bound (`Eq + Hash + Clone`)، `HashMap` + `VecDeque` |
| 6 | Event bus | trait، `Box<dyn Trait>`، dynamic dispatch |

## چالش‌های بعدی
- تمرین ۵ را O(1) کن. - تمرین ۴ را thread-safe کن (`Arc<Mutex<_>>`).
- تمرین ۲ را با `impl std::fmt::Display` و `impl std::error::Error` برای ParseError کامل کن.
- تمرین ۳ را به parser عبارت infix (shunting-yard) گسترش بده.
