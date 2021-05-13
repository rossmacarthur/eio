mod support;

use std::any::type_name;
use std::fmt::LowerHex;
use std::io::Cursor;
use std::ops::Sub;

use eio::*;

use crate::support::*;

#[test]
fn integers() {
    test::<u8, 1>(&[0x01], 0x01, 0x01);
    test::<i8, 1>(&[0x01], 0x01, 0x01);

    test::<u16, 2>(&[0x01, 0x02], 0x0102, 0x0201);
    test::<i16, 2>(&[0x01, 0x02], 0x0102, 0x0201);

    test::<u32, 4>(&[0x01, 0x02, 0x03, 0x04], 0x01020304, 0x04030201);
    test::<i32, 4>(&[0x01, 0x02, 0x03, 0x04], 0x01020304, 0x04030201);

    test::<u64, 8>(
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        0x0102030405060708,
        0x0807060504030201,
    );
    test::<i64, 8>(
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        0x0102030405060708,
        0x0807060504030201,
    );

    test::<u128, 16>(
        &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ],
        0x000102030405060708090a0b0c0d0e0f,
        0x0f0e0d0c0b0a09080706050403020100,
    );
    test::<i128, 16>(
        &[
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ],
        0x000102030405060708090a0b0c0d0e0f,
        0x0f0e0d0c0b0a09080706050403020100,
    );

    fn test<T: LowerHex + Num<N>, const N: usize>(data: &[u8], be: T, le: T) {
        let msg = format!(
            "type = {}, data = `{:?}`, be = 0x{:x}, le = 0x{:x}",
            type_name::<T>(),
            data,
            be,
            le,
        );

        let mut rdr = Cursor::new(data);
        assert_eq!(rdr.read_be::<T>().expect(&msg), be, "be failed: {}", msg);

        let mut rdr = Cursor::new(data);
        assert_eq!(rdr.read_le::<T>().expect(&msg), le, "le failed: {}", msg);

        let mut wtr = Vec::with_capacity(N);
        wtr.write_be(be).expect(&msg);
        assert_eq!(wtr, data, "be failed: {}", msg);

        let mut wtr = Vec::with_capacity(N);
        wtr.write_le(le).expect(&msg);
        assert_eq!(wtr, data, "le failed: {}", msg);
    }
}

#[test]
fn floats() {
    test(123.45678_f32);
    test(123.45678_f64);

    fn test<T, const N: usize>(f: T)
    where
        T: Copy + Sub<Output = T> + PartialOrd + Abs + Epsilon + Num<N>,
    {
        let msg = format!("type = {}, value = {}", type_name::<T>(), f);

        let mut wtr = Vec::new();
        wtr.write_be(f).expect(&msg);
        wtr.write_le(f).expect(&msg);

        let mut rdr = Cursor::new(&wtr);
        assert!(
            (rdr.read_be::<T>().unwrap() - f).abs() < T::epsilon(),
            "be failed: {}",
            msg
        );
        assert!(
            (rdr.read_le::<T>().unwrap() - f).abs() < T::epsilon(),
            "le failed: {}",
            msg
        );
    }
}
