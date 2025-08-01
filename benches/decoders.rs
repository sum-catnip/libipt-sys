#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::mem;

    use libipt_sys::*;
    use test::Bencher;

    #[bench]
    fn bench_pkt_decoder(b: &mut Bencher) {
        let trace = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/benches/pt_example_trace.bin"
        ));
        let mut config: pt_config = unsafe { mem::zeroed() };
        config.size = size_of::<pt_config>();
        config.begin = trace.as_ptr() as *mut u8;
        config.end = unsafe { trace.as_ptr().add(trace.len()) } as *mut u8;

        b.iter(|| {
            let decoder = unsafe { pt_pkt_alloc_decoder(&raw mut config) };

            let err = unsafe { pt_pkt_sync_forward(decoder) };
            if err < 0 {
                panic!("pt_pkt_sync_forward failed: {}", err);
            }

            loop {
                let mut packet = mem::MaybeUninit::uninit();
                let err =
                    unsafe { pt_pkt_next(decoder, packet.as_mut_ptr(), size_of::<pt_packet>()) };
                if err < 0 {
                    if -err == pt_error_code_pte_eos as i32 {
                        break;
                    }
                    panic!("pt_pkt_next failed: {}", err);
                }
            }

            unsafe { pt_pkt_free_decoder(decoder) };
        });
    }
}
