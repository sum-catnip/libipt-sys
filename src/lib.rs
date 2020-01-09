#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lib_link() {
        unsafe { pt_library_version() };
    }
}