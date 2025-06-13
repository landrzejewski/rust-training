use rust_training::wrappers::Integer;

#[test]
fn add_integers() {
    let lhs = Integer::new(1);
    let rhs = Integer::new(2);
    assert_eq!(lhs + rhs, Integer::new(3));
}

#[test]
#[should_panic]
fn should_panic() {
   panic!("Test")
}

// cargo test -- --test-threads=1 --show-output