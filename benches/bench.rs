#![feature(test)]

extern crate test;

macro_rules! bench_num {
    ($name:ident, $ty:ident, $max:ident, $read:ident, $write:ident, $bytes:expr, $data:expr) => {
        mod $name {
            use byteorder_lite::{
                BigEndian, ByteOrder, LittleEndian, NativeEndian,
            };
            use test::black_box as bb;
            use test::Bencher;

            const NITER: usize = 100_000;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(BigEndian::$read(&buf, $bytes));
                    }
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(LittleEndian::$read(&buf, $bytes));
                    }
                });
            }

            #[bench]
            fn read_native_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(NativeEndian::$read(&buf, $bytes));
                    }
                });
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max >> (1 + $ty::$max.count_ones() - $bytes * 8);
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(BigEndian::$write(&mut buf, n, $bytes));
                    }
                });
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max >> (1 + $ty::$max.count_ones() - $bytes * 8);
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(LittleEndian::$write(&mut buf, n, $bytes));
                    }
                });
            }

            #[bench]
            fn write_native_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max >> (1 + $ty::$max.count_ones() - $bytes * 8);
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(NativeEndian::$write(&mut buf, n, $bytes));
                    }
                });
            }
        }
    };
    ($ty:ident, $max:ident,
     $read:ident, $write:ident, $data:expr) => {
        mod $ty {
            use byteorder_lite::{
                BigEndian, ByteOrder, LittleEndian, NativeEndian,
            };
            use std::$ty;
            use test::black_box as bb;
            use test::Bencher;

            const NITER: usize = 100_000;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(BigEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(LittleEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn read_native_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(NativeEndian::$read(&buf));
                    }
                });
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(BigEndian::$write(&mut buf, n));
                    }
                });
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(LittleEndian::$write(&mut buf, n));
                    }
                });
            }

            #[bench]
            fn write_native_endian(b: &mut Bencher) {
                let mut buf = $data;
                let n = $ty::$max;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(NativeEndian::$write(&mut buf, n));
                    }
                });
            }
        }
    };
}

bench_num!(u16, MAX, read_u16, write_u16, [1, 2]);
bench_num!(i16, MAX, read_i16, write_i16, [1, 2]);
bench_num!(u32, MAX, read_u32, write_u32, [1, 2, 3, 4]);
bench_num!(i32, MAX, read_i32, write_i32, [1, 2, 3, 4]);
bench_num!(u64, MAX, read_u64, write_u64, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(i64, MAX, read_i64, write_i64, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(f32, MAX, read_f32, write_f32, [1, 2, 3, 4]);
bench_num!(f64, MAX, read_f64, write_f64, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(uint_1, u64, MAX, read_uint, write_uint, 1, [1]);
bench_num!(uint_2, u64, MAX, read_uint, write_uint, 2, [1, 2]);
bench_num!(uint_3, u64, MAX, read_uint, write_uint, 3, [1, 2, 3]);
bench_num!(uint_4, u64, MAX, read_uint, write_uint, 4, [1, 2, 3, 4]);
bench_num!(uint_5, u64, MAX, read_uint, write_uint, 5, [1, 2, 3, 4, 5]);
bench_num!(uint_6, u64, MAX, read_uint, write_uint, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(uint_7, u64, MAX, read_uint, write_uint, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(
    uint_8,
    u64,
    MAX,
    read_uint,
    write_uint,
    8,
    [1, 2, 3, 4, 5, 6, 7, 8]
);

bench_num!(int_1, i64, MAX, read_int, write_int, 1, [1]);
bench_num!(int_2, i64, MAX, read_int, write_int, 2, [1, 2]);
bench_num!(int_3, i64, MAX, read_int, write_int, 3, [1, 2, 3]);
bench_num!(int_4, i64, MAX, read_int, write_int, 4, [1, 2, 3, 4]);
bench_num!(int_5, i64, MAX, read_int, write_int, 5, [1, 2, 3, 4, 5]);
bench_num!(int_6, i64, MAX, read_int, write_int, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(int_7, i64, MAX, read_int, write_int, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(int_8, i64, MAX, read_int, write_int, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(
    u128,
    MAX,
    read_u128,
    write_u128,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);
bench_num!(
    i128,
    MAX,
    read_i128,
    write_i128,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

bench_num!(uint128_1, u128, MAX, read_uint128, write_uint128, 1, [1]);
bench_num!(uint128_2, u128, MAX, read_uint128, write_uint128, 2, [1, 2]);
bench_num!(uint128_3, u128, MAX, read_uint128, write_uint128, 3, [1, 2, 3]);
bench_num!(uint128_4, u128, MAX, read_uint128, write_uint128, 4, [1, 2, 3, 4]);
bench_num!(
    uint128_5,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    5,
    [1, 2, 3, 4, 5]
);
bench_num!(
    uint128_6,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    6,
    [1, 2, 3, 4, 5, 6]
);
bench_num!(
    uint128_7,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    7,
    [1, 2, 3, 4, 5, 6, 7]
);
bench_num!(
    uint128_8,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    8,
    [1, 2, 3, 4, 5, 6, 7, 8]
);
bench_num!(
    uint128_9,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    9,
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
);
bench_num!(
    uint128_10,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    10,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
);
bench_num!(
    uint128_11,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    11,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
);
bench_num!(
    uint128_12,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    12,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
bench_num!(
    uint128_13,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    13,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
bench_num!(
    uint128_14,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    14,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
bench_num!(
    uint128_15,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    15,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
bench_num!(
    uint128_16,
    u128,
    MAX,
    read_uint128,
    write_uint128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

bench_num!(int128_1, i128, MAX, read_int128, write_int128, 1, [1]);
bench_num!(int128_2, i128, MAX, read_int128, write_int128, 2, [1, 2]);
bench_num!(int128_3, i128, MAX, read_int128, write_int128, 3, [1, 2, 3]);
bench_num!(int128_4, i128, MAX, read_int128, write_int128, 4, [1, 2, 3, 4]);
bench_num!(int128_5, i128, MAX, read_int128, write_int128, 5, [1, 2, 3, 4, 5]);
bench_num!(
    int128_6,
    i128,
    MAX,
    read_int128,
    write_int128,
    6,
    [1, 2, 3, 4, 5, 6]
);
bench_num!(
    int128_7,
    i128,
    MAX,
    read_int128,
    write_int128,
    7,
    [1, 2, 3, 4, 5, 6, 7]
);
bench_num!(
    int128_8,
    i128,
    MAX,
    read_int128,
    write_int128,
    8,
    [1, 2, 3, 4, 5, 6, 7, 8]
);
bench_num!(
    int128_9,
    i128,
    MAX,
    read_int128,
    write_int128,
    9,
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
);
bench_num!(
    int128_10,
    i128,
    MAX,
    read_int128,
    write_int128,
    10,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
);
bench_num!(
    int128_11,
    i128,
    MAX,
    read_int128,
    write_int128,
    11,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
);
bench_num!(
    int128_12,
    i128,
    MAX,
    read_int128,
    write_int128,
    12,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
);
bench_num!(
    int128_13,
    i128,
    MAX,
    read_int128,
    write_int128,
    13,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
);
bench_num!(
    int128_14,
    i128,
    MAX,
    read_int128,
    write_int128,
    14,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);
bench_num!(
    int128_15,
    i128,
    MAX,
    read_int128,
    write_int128,
    15,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
bench_num!(
    int128_16,
    i128,
    MAX,
    read_int128,
    write_int128,
    16,
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);

macro_rules! bench_slice {
    ($name:ident, $numty:ty, $read:ident, $write:ident) => {
        mod $name {
            use std::mem::size_of;

            use byteorder_lite::{BigEndian, ByteOrder, LittleEndian};
            use rand::distributions;
            use rand::{self, Rng};
            use test::Bencher;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let mut numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                BigEndian::$write(&numbers, &mut bytes);

                b.bytes = bytes.len() as u64;
                b.iter(|| {
                    BigEndian::$read(&bytes, &mut numbers);
                });
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let mut numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                LittleEndian::$write(&numbers, &mut bytes);

                b.bytes = bytes.len() as u64;
                b.iter(|| {
                    LittleEndian::$read(&bytes, &mut numbers);
                });
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                b.bytes = bytes.len() as u64;
                b.iter(|| {
                    BigEndian::$write(&numbers, &mut bytes);
                });
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let numbers: Vec<$numty> = rand::thread_rng()
                    .sample_iter(&distributions::Standard)
                    .take(100000)
                    .collect();
                let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                b.bytes = bytes.len() as u64;
                b.iter(|| {
                    LittleEndian::$write(&numbers, &mut bytes);
                });
            }
        }
    };
}

bench_slice!(slice_u16, u16, read_u16_into, write_u16_into);
bench_slice!(slice_u64, u64, read_u64_into, write_u64_into);
bench_slice!(slice_i64, i64, read_i64_into, write_i64_into);
