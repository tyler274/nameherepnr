//#![feature(const_mut_refs)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_convert)]
#![feature(const_box)]
pub mod kernel;
pub mod place;
pub mod route;
pub mod ice40;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
