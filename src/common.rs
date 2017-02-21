use std::ptr;
use std::mem;

// Took the read and write bytes code with minor modifications from the
// byteorder crate.
// The reason we're not using the actual crate is that byteorder will always
// try to convert the byte interpreted data to the processors native byte order,
// which will result in a byte reshuffle on every byte read (on a little-endian
// machine). That doesn't sound very efficient for a cpu emulator.
macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr) => ({
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            ptr::copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data
    });
}

#[inline]
pub fn read_u16 (src: &[u8]) -> u16 {
    read_num_bytes!(u16, 2, src)
}

#[inline]
pub fn read_u32 (src: &[u8]) -> u32 {
    read_num_bytes!(u32, 4, src)
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr) => ({
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = mem::transmute::<_, [u8; $size]>($n);
            ptr::copy_nonoverlapping((&bytes).as_ptr(),
                                     $dst.as_mut_ptr(),
                                     $size);
        }
    });
}

#[inline]
pub fn write_u16(dst: &mut [u8], n: u16) {
    write_num_bytes!(u16, 2, n, dst);
}

#[inline]
pub fn write_u32(dst: &mut [u8], n: u32) {
    write_num_bytes!(u32, 4, n, dst);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_u16_from_vec() {
        let word = vec!(0x12, 0x34, 0x56, 0x78);
        let half_word  = read_u16(&word[..2]);
        assert_eq!(0x3412, half_word);
    }

    #[test]
    #[should_panic]
    fn read_u16_from_vec_too_little_bytes() {
        let word = vec!(0x12, 0x34, 0x56, 0x78);
        read_u16(&word[..1]);
    }
    
    #[test]
    fn write_u16_to_vec() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        write_u16(&mut vec[..2], 0xEEFF);
    }

    
    #[test]
    #[should_panic]
    fn write_u16_to_vec_too_many_bytes() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        write_u16(&mut vec[..1], 0xEEFF);
    }

    
    #[test]
    fn read_u32_from_vec() {
        let first_word = vec!(0x12, 0x34, 0x56, 0x78);
        let word  = read_u32(&first_word);
        assert_eq!(0x78563412, word);
    }

    #[test]
    #[should_panic]
    fn read_u32_from_vec_too_little_bytes() {
        let first_word = vec!(0x12, 0x34, 0x56, 0x78, 0x9A);
        read_u32(&first_word[..2]);
    }
    
    #[test]
    fn write_u32_to_vec() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        write_u32(&mut vec[0..4], 0xCCDDEEFF);
        assert_eq!(vec, vec!(0xFF, 0xEE, 0xDD, 0xCC, 5, 6, 7, 8));
    }

    #[test]
    #[should_panic]
    fn write_u32_to_vec_too_little_bytes() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        write_u32(&mut vec[0..2], 0xCCDDEEFF);
    }
}
