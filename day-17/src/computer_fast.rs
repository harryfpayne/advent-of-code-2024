use std::ops::{BitXor};

pub fn run(a: &mut i64) -> i64 {
    // Decompiled from the operations
    let mut b = *a % 8;
    b = b.bitxor(1);
    let c = *a / 2i64.pow(b as u32);
    b = b.bitxor(5);
    *a = *a / 8; // 2^3
    b = b.bitxor(c);
    b % 8
}