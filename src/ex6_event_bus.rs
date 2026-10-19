//! تمرین ۶: Trait Object (`Box<dyn Trait>`)، polymorphism پویا

pub trait Notifier {
    fn name(&self) -> String;
    fn notify(&mut self, event: &str) -> Result<(), String>;
}

pub struct EventBus {
    subscribers: Vec<Box<dyn Notifier>>,
}

impl EventBus {
    pub fn new() -> Self {
        todo!()
    }

    pub fn subscribe(&mut self, n: Box<dyn Notifier>) {
        todo!()
    }

    pub fn subscriber_count(&self) -> usize {
        todo!()
    }

    /// event را به همه‌ی subscriber ها بده (به ترتیب ثبت).
    /// خطای یکی نباید بقیه را متوقف کند. خروجی: لیست خطاها با فرمت "name: error"
    pub fn publish(&mut self, event: &str) -> Vec<String> {
        todo!()
    }
}
