#![feature(proc_macro_hygiene)]
mod engine;

pub use engine::*;

#[cfg(test)]
mod tests {
    pub use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
