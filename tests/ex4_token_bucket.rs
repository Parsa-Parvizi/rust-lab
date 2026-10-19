use rust_lab::ex4_token_bucket::*;

#[test]
fn starts_full_and_drains() {
    let mut b = TokenBucket::new(3, 1, 0);
    assert!(b.try_acquire(0));
    assert!(b.try_acquire(0));
    assert!(b.try_acquire(0));
    assert!(!b.try_acquire(0));
}

#[test]
fn refills_over_time() {
    let mut b = TokenBucket::new(3, 1, 0);
    for _ in 0..3 {
        assert!(b.try_acquire(0));
    }
    assert!(b.try_acquire(1000));
    assert!(!b.try_acquire(1000));
}

#[test]
fn fractional_refill_accumulates() {
    let mut b = TokenBucket::new(1, 1, 0);
    assert!(b.try_acquire(0));
    assert!(!b.try_acquire(500)); // 0.5 توکن
    assert!(b.try_acquire(1000)); // 1.0 توکن
}

#[test]
fn never_exceeds_capacity() {
    let mut b = TokenBucket::new(3, 5, 0);
    assert_eq!(b.available(60_000), 3);
}

#[test]
fn clock_going_backwards_does_not_panic_or_refill() {
    let mut b = TokenBucket::new(2, 1, 10_000);
    assert!(b.try_acquire(10_000));
    assert!(b.try_acquire(10_000));
    assert!(!b.try_acquire(5_000));
}
