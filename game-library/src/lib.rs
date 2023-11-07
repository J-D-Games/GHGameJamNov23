#![warn(missing_docs)]
//! This is crate level documentation and is the first page that shows up.


pub mod save_load;
pub mod mutual_exclusivity_guard;


/// This is function level documentation, it shows up with the function itself in the documentation.
/// Example
/// This shows up as an example in the documentation. Additionally, examples are compiled and run with cargo test to ensure that they still work.
/// ```rust
/// use game_library::add;
/// 
/// let a = 5usize;
/// let b = 10usize;
/// 
/// assert_eq!(add(a,b), 15)
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// This is module level documentation, it shows up when looking at a module overview (although tests isn't public, so it won't show up in the generated doc).
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
