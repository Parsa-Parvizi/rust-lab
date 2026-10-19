impl EventBus {
    pub fn new() -> Self {
        EventBus { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self, n: Box<dyn Notifier>) {
        self.subscribers.push(n);
    }

    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    pub fn publish(&mut self, event: &str) -> Vec<String> {
        let mut failures = Vec::new();
        for s in self.subscribers.iter_mut() {
            if let Err(e) = s.notify(event) {
                failures.push(format!("{}: {}", s.name(), e));
            }
        }
        failures
    }
}
