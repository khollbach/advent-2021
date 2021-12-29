use std::io;
use std::io::BufRead;
use itertools::Itertools;

/// Input should contain a hex string on a single line.
///
/// Transform the hex string into a string of ASCII 0s and 1s.
fn read_input(input: impl BufRead) -> String {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    let bits = line.chars().map(|hex_digit| {
        let n = hex_digit.to_digit(16).unwrap();
        let four_bits = format!("{:04b}", n);
        debug_assert_eq!(four_bits.len(), 4);
        debug_assert!(four_bits.chars().all(|c| c == '0' || c == '1'));
        four_bits
    });

    // String concatenation; e.g.: [1100, 1111, 0000] => 110011110000
    bits.collect()
}

pub fn main() {
    let bits = read_input(io::stdin().lock());

    println!("{}", read_packet(&mut ascii_chars(&bits)));
}

/// The same as s.chars(), but this is an ExactSizeIterator.
fn ascii_chars<'a>(s: &'a str) -> impl ExactSizeIterator<Item=char> + 'a {
    assert!(s.is_ascii());
    s.as_bytes().iter().map(|&ascii_byte| ascii_byte as char)
}

/// Return the sum of the version numbers of this packet and subpackets.
fn read_packet(bits: &mut impl ExactSizeIterator<Item=char>) -> u32 {
    let version = read_u32(bits.take(3));
    let type_ = read_u32(bits.take(3));

    let mut version_total = version;

    // Literal.
    if type_ == 4 {
        let mut group = String::new();
        loop {
            let continue_ = bits.next().unwrap();
            group.extend(bits.take(4));

            if continue_ == '0' {
                break;
            }
        }

        // We'll probably end up using this in part 2.
        let _value = u32::from_str_radix(&group, 2);
    }
    // Operator.
    else {
        let length_type = bits.next().unwrap();

        if length_type == '1' {
            let num_subpackets = read_u32(bits.take(11));

            for _ in 0..num_subpackets {
                version_total += read_packet(bits);
            }
        } else {
            let total_subpacket_len = read_u32(bits.take(15)) as usize;
            assert!(total_subpacket_len <= bits.len());

            let leftover_len = bits.len() - total_subpacket_len;
            while bits.len() > leftover_len {
                version_total += read_packet(bits);
            }
            assert_eq!(bits.len(), leftover_len);
        }
    }

    version_total
}

fn read_u32(bits: impl Iterator<Item=char>) -> u32 {
    let s: String = bits.collect();
    u32::from_str_radix(&s, 2).unwrap()
}
