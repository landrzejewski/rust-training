use rust_training::wrapper::Wrapper;

// https://github.com/la10736/rstest
// https://github.com/becheran/ntest

#[test]
fn my_test() {
    let first_wrapper = Wrapper::new(1);
    let second_wrapper = Wrapper::new(1);
    assert_eq!(first_wrapper + second_wrapper, Wrapper::new(2));
}
