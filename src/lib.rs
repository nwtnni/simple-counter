/// Generates a thread-local global counter.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate count;
/// 
/// generate_counter!(Counter, usize);
/// 
/// fn main() {
/// 
///   assert_eq!(Counter::next(), 0);
///   assert_eq!(Counter::next(), 1);
///   assert_eq!(Counter::next(), 2);
/// 
///   Counter::reset();
/// 
///   assert_eq!(Counter::next(), 0);
/// }
/// ```
#[macro_export]
macro_rules! generate_counter {
    ($name:ident, $type:ident) => {

        #[allow(non_snake_case)]
        pub mod $name {
            use std::cell::Cell;

            thread_local!(
                static COUNTER: Cell<$type> = Cell::new(0);
            );

            pub fn next() -> $type {
                COUNTER.with(|cell| {
                    let n = cell.get();
                    cell.set(n + 1);
                    n
                })
            }

            #[allow(dead_code)]
            pub fn set(n: $type) {
                COUNTER.with(|cell| cell.set(n));
            }

            #[allow(dead_code)]
            pub fn reset() {
                COUNTER.with(|cell| cell.set(0));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_basic() {
        generate_counter!(Counter, i8);
        assert_eq!(0, Counter::next());
        assert_eq!(1, Counter::next());
        assert_eq!(2, Counter::next());
    }

    #[test]
    fn test_reset() {
        generate_counter!(Counter, i8);
        assert_eq!(0, Counter::next());
        assert_eq!(1, Counter::next());
        Counter::reset();
        assert_eq!(0, Counter::next());
    }

    #[test]
    fn test_set() {
        generate_counter!(Counter, u32);
        Counter::set(100);
        assert_eq!(100, Counter::next());
        assert_eq!(101, Counter::next());
        assert_eq!(102, Counter::next());
    }
}
