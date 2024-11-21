#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn libipt_version() {
        let libipt_version = unsafe { pt_library_version() };
        assert_eq!(
            (
                libipt_version.major,
                libipt_version.minor,
                libipt_version.patch
            ),
            (2, 1, 1)
        );
    }
}
