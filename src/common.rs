pub trait MemAccess {
    fn read_mem(src: &[u8], addr: usize) -> Self;
    fn write_mem(src: &mut [u8], addr: usize, val: Self);
}


impl MemAccess for u16 {
    fn read_mem(src: &[u8], addr: usize) -> u16 {
        (src[addr] as u16) << 8 | src[addr + 1] as u16
    }

    fn write_mem(src: &mut [u8], addr: usize, val: u16) {
        src[addr]     = ((val >> 8)  & 0xFF) as u8;
        src[addr + 1] = (val & 0xFF) as u8;
    }
}


impl MemAccess for u32 {
    fn read_mem(src: &[u8], addr: usize) -> u32 {
        (src[addr]     as u32) << 24 |
        (src[addr + 1] as u32) << 16 |
        (src[addr + 2] as u32) << 8  |
         src[addr + 3] as u32
    }

    fn write_mem(src: &mut [u8], addr: usize, val: u32) {
        src[addr]     = (val >> 24) as u8;
        src[addr + 1] = ((val >> 16) & 0xFF) as u8;
        src[addr + 2] = ((val >> 8)  & 0xFF) as u8;
        src[addr + 3] = (val & 0xFF) as u8;
    }
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_u16_from_vec() {
        let word = vec!(0x12, 0x34, 0x56, 0x78);
        let half_word  = u16::read_mem(&word, 0);
        assert_eq!(0x1234, half_word);
    }

    #[test]
    fn write_u16_to_vec() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        u16::write_mem(&mut vec, 0, 0xEEFF);
        assert_eq!(vec, vec!(0xEE, 0xFF, 3, 4, 5, 6, 7, 8));
    }


    #[test]
    fn read_u32_from_vec() {
        let first_word = vec!(0x12, 0x34, 0x56, 0x78);
        let word  = u32::read_mem(&first_word, 0);
        assert_eq!(0x12345678, word);
    }

    #[test]
    fn write_u32_to_vec() {
        let mut vec = vec!(1, 2, 3, 4, 5, 6, 7, 8);
        u32::write_mem(&mut vec, 0, 0xCCDDEEFF);
        assert_eq!(vec, vec!(0xCC, 0xDD, 0xEE, 0xFF, 5, 6, 7, 8));
    }
}
