use std::process::exit;

pub fn get_args() -> Vec<String> {
   std::env::args().skip(1).collect()
}

// Named to avoid confusion with the std `assert!` macro: this helper does
// not panic — it reports the failure via the callback and exits the process.
pub fn ensure_or_exit<T>(value: T, predicate: impl Fn(T) -> bool, on_failure: impl Fn()) {
    if !predicate(value) {
        on_failure();
        // A failed precondition is an error, so the process must NOT report
        // success: exit(0) would make scripts and CI treat this as a pass.
        // 2 is the conventional code for "wrong usage".
        exit(2);
    }
}

pub fn is_not_empty<T>(vec: &[T]) -> bool {
    !vec.is_empty()
}

pub fn min_length<T>(length: usize) -> impl Fn(&[T]) -> bool {
    move |vec| vec.len() >= length
}

