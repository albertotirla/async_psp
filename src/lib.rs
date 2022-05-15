#![cfg_attr(not(test), no_std)]
pub mod task;

#[cfg(not(test))]
extern crate alloc;

/* 
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/