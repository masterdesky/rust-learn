fn test_integer() {
    let int8: i8 = std::i8::MAX;
    let uint8: u8 = std::u8::MAX;
    let int16: i16 = std::i16::MAX;
    let uint16: u16 = std::u16::MAX;
    let int32: i32 = std::i32::MAX;
    let uint32: u32 = std::u32::MAX;
    let int64: i64 = std::i64::MAX;
    let uint64: u64 = std::u64::MAX;
    let int128: i128 = std::i128::MAX;
    let uint128: u128 = std::u128::MAX;
    let isize_test: isize = std::isize::MAX; 
    let usize_test: usize = std::usize::MAX;

    println!("Testing integer types with their maximum values...:");
    println!("i8: {}, u8: {}", int8, uint8);
    println!("i16: {}, u16: {}", int16, uint16);
    println!("i32: {}, u32: {}", int32, uint32);
    println!("i64: {}, u64: {}", int64, uint64);
    println!("i128: {}, u128: {}", int128, uint128);
    println!("isize: {}, usize: {}", isize_test, usize_test);
}

fn test_literals() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("Testing various integer literals...:");
    println!("Decimal: {}", decimal);
    println!("Hexadecimal: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", binary);
    println!("Byte: {}", byte);
}

fn main() {
    test_integer();
    println!();
    test_literals();
}
