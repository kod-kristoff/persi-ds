#![no_std]

extern crate alloc;

pub mod common;
pub mod sync;
pub mod unsync;
// pub mod list;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
