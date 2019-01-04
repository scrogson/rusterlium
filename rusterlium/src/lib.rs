pub use rusterlium_codegen::rusterlium;
pub use rustler::Env;
pub use rustler::Term;

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use crate::*;

    #[test]
    fn test_nif_macro() {
        #[rusterlium(name = "add", schedule = "DirtyCpu")]
        fn add(a: u32, b: u32) -> u32 {
            a + b
        }
    }
}
