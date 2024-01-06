use test_app::*;

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    for i in 1..=5 {
        assert_eq!(counter.next(), Some(i));
    }
    assert_eq!(counter.next(), None);
}

#[test]
fn other() {
    let counter: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|num| num % 3 == 0)
        .sum();

    assert_eq!(18, counter);
}
