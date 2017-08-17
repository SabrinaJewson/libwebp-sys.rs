#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("ffi.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    use std::slice;

    #[test]
    fn test_webp() {
        let mut width = 0;
        let mut height = 0;
        let mut buf = Vec::new();
        let len = File::open("./tests/1.webp").unwrap().read_to_end(&mut buf).unwrap();

        unsafe { WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height); }
        assert!(width == 1290);
        assert!(height == 996);
    }

    #[test]
    fn test_webp_encode_lossless() {
        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif").unwrap().read_to_end(&mut buf).unwrap();

        let mut out_buf = Box::into_raw(Box::new(0u8)) as *mut _;
        unsafe {
            let l = WebPEncodeLosslessRGBA(buf.as_ptr(), 1000, 1000, 1000 * 4, &mut out_buf as *mut _);
            let mut file = File::create("./tests/test1.webp").unwrap();
            let out = slice::from_raw_parts(out_buf, l);
            file.write_all(out).unwrap();
        }

        assert!(true);
    }

    #[test]
    fn test_webp_encode() {
        println!("{:?}", env!("OUT_DIR"));
        let mut buf = Vec::new();
        let len = File::open("./tests/test1_1000x1000.bif").unwrap().read_to_end(&mut buf).unwrap();

        let mut out_buf = Box::into_raw(Box::new(0u8)) as *mut _;
        unsafe {
            let l = WebPEncodeRGBA(buf.as_ptr(), 1000, 1000, 1000 * 4, 90 as f32, &mut out_buf as *mut _);
            println!("{:?}", l);
            let mut file = File::create("./tests/test1_q90.webp").unwrap();
            let out = slice::from_raw_parts(out_buf, l);
            println!("{:?}", out.len());
            file.write_all(out).unwrap();
        }

        assert!(true);
    }
}
