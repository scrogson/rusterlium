#![allow(dead_code)]
#[macro_use]
extern crate rusterlium_codegen;

#[cfg(test)]
mod test {
    #[test]
    fn test_nif_macro() {
        #[nif(hello, 1)]
        fn hello(env: String) -> String {
            format!("Hello {}", env)
        }
    }
}
