use erlang_nif_sys::{c_int, ErlNifEnv, ERL_NIF_TERM};
pub use rusterlium_codegen::rusterlium;
pub use rustler::{Decoder, Encoder, Env, Term};

//macro_rules! rusterlium_init {
//($module:expr, [$($name:path),*]) => {
//#[no_mangle]
//pub extern "C" fn nif_init() -> *const ErlNifEntry {
//let mut nifs: Vec<ErlNifFunc> = Vec::new();
//$(nifs.push(concat!($name, "_ErlNifFunc")))*
//}
//};
//}

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    use crate::*;

    #[test]
    fn test_nif_macro() {
        #[rusterlium(schedule = "DirtyCpu")]
        pub fn add(a: u32, b: u32) -> u32 {
            a + b
        }

        #[rusterlium(name = "sub2", schedule = "DirtyCpu")]
        fn sub(a: u32, b: u32) -> u32 {
            a - b
        }

        //rusterlium_init!("Math", [add, sub]);
    }
}
