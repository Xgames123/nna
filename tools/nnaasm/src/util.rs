use libnna::u4;
use std::str::FromStr;

pub fn to_hex4(val: u4) -> String {
    let mut str = String::from_str("0x").unwrap();
    str.push(char::from_digit(val.into_u32(), 16).unwrap());
    str
}

pub fn count_nonzero_banks(data: &[u4; 256]) -> usize {
    let mut count = 0;
    for bank in data.chunks(16) {
        let mut zero = true;
        for nib in bank {
            if *nib != u4::ZERO {
                zero = false;
                break;
            }
        }
        if zero {
            count += 1;
        }
    }
    count
}
