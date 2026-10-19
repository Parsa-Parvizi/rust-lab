use rust_lab::ex6_event_bus::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Recorder {
    label: String,
    log: Rc<RefCell<Vec<String>>>,
    fail_on: Option<String>,
}

impl Notifier for Recorder {
    fn name(&self) -> String {
        self.label.clone()
    }
    fn notify(&mut self, event: &str) -> Result<(), String> {
        if self.fail_on.as_deref() == Some(event) {
            return Err("boom".to_string());
        }
        self.log.borrow_mut().push(format!("{}:{}", self.label, event));
        Ok(())
    }
}

fn rec(label: &str, log: &Rc<RefCell<Vec<String>>>, fail_on: Option<&str>) -> Box<dyn Notifier> {
    Box::new(Recorder {
        label: label.to_string(),
        log: Rc::clone(log),
        fail_on: fail_on.map(String::from),
    })
}

#[test]
fn delivers_to_all_in_order() {
    let log = Rc::new(RefCell::new(vec![]));
    let mut bus = EventBus::new();
    bus.subscribe(rec("email", &log, None));
    bus.subscribe(rec("sms", &log, None));
    assert_eq!(bus.subscriber_count(), 2);

    let errors = bus.publish("signup");
    assert!(errors.is_empty());
    assert_eq!(*log.borrow(), vec!["email:signup", "sms:signup"]);
}

#[test]
fn one_failure_does_not_stop_others() {
    let log = Rc::new(RefCell::new(vec![]));
    let mut bus = EventBus::new();
    bus.subscribe(rec("email", &log, Some("payment")));
    bus.subscribe(rec("sms", &log, None));

    let errors = bus.publish("payment");
    assert_eq!(errors, vec!["email: boom".to_string()]);
    assert_eq!(*log.borrow(), vec!["sms:payment"]);
}
