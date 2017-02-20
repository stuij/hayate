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
