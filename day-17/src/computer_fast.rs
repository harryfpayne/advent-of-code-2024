use std::ops::{BitAnd, BitXor};

const powers_of_two: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub fn run(a: &mut i64, b: &mut i64, c: &mut i64) -> i64 {
    *b = *a % 8;
    *b = b.bitxor(1);
    *c = *a / 2i64.pow(*b as u32);
    *b = b.bitxor(5);
    *a = *a / 8; // 2^3
    *b = b.bitxor(*c);
    *b % 8
}

pub fn run2(a: &mut u8, b: &mut u8, c: &mut u8) -> u8 {
    *b = *a % 8;
    *b = b.bitxor(1);
    *c = *a / powers_of_two[*b as usize];
    *b = b.bitxor(5);
    *a = *a / 8; // 2^3
    *b = b.bitxor(*c);
    *b % 8
}

pub fn run_test(a: &mut i64, b: &mut i64, c: &mut i64) -> i64 {
    *a = *a / 8;
    *a % 8
}

const one_mask: i64 = 0b10000010000000110100010110101;
const zero_mask: i64 = !0b11111111111111111110111111110111101;
const max: i64 = 164_532_461_596_349;
pub struct BitMaskIterator {
    current: i64,
}

impl BitMaskIterator {
    pub fn new() -> Self {
        let mut gaps = vec![];
        for (u, i) in (0..i64::BITS).enumerate() {
            if one_mask & (1 << i) == 0 && zero_mask & (1 << i) == 0 {
                gaps.push((i as i64, u));
            }
        }


        BitMaskIterator{
            current: 0,
        }
    }
}

impl Iterator for BitMaskIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = one_mask;
        let mut c = 0;
        for i in (0..i64::BITS) {
            if one_mask & (1 << i) == 0 && zero_mask & (1 << i) == 0 && self.current & (1 << i) != 0 {
                value |= (1 << c);
                c += 1
            }
        }

        self.current += 1;
        Some(value)
    }
}