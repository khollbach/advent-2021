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

    let ans = eval_packet(&mut ascii_chars(&bits));
    println!("{}", ans.version_sum);
    println!("{}", ans.value);
}

/// The same as s.chars(), but we know the exact length.
fn ascii_chars<'a>(s: &'a str) -> impl ExactSizeIterator<Item=char> + 'a {
    assert!(s.is_ascii());
    s.as_bytes().iter().map(|&ascii_byte| ascii_byte as char)
}

struct EvalRet {
    version_sum: u32,
    value: u64,
}

fn eval_packet(bits: &mut impl ExactSizeIterator<Item=char>) -> EvalRet {
    let version = read_u32(bits.take(3));
    let type_ = read_u32(bits.take(3));

    let mut version_sum = version;

    let value = if type_ == 4 {
        // Literal.
        read_literal_payload(bits)
    } else {
        // Operator.
        let inner_values = read_operator_payload(bits).into_iter().map(|inner| {
            version_sum += inner.version_sum;
            inner.value
        });

        combine_values(inner_values, type_)
    };

    EvalRet {
        version_sum,
        value,
    }
}

fn read_u32(bits: impl Iterator<Item=char>) -> u32 {
    let s: String = bits.collect();
    u32::from_str_radix(&s, 2).unwrap()
}

fn read_literal_payload(bits: &mut impl Iterator<Item=char>) -> u64 {
    let mut group = String::new();
    loop {
        let continue_ = bits.next().unwrap();
        group.extend(bits.take(4));

        if continue_ == '0' {
            break;
        }
    }

    u64::from_str_radix(&group, 2).unwrap()
}

/// Evaluate each inner packet.
fn read_operator_payload(bits: &mut impl ExactSizeIterator<Item=char>) -> Vec<EvalRet> {
    let mut inner_packets = vec![];

    let length_type = bits.next().unwrap();

    if length_type == '1' {
        let num_subpackets = read_u32(bits.take(11));

        for _ in 0..num_subpackets {
            inner_packets.push(eval_packet(bits));
        }
    } else {
        let total_subpacket_len = read_u32(bits.take(15)) as usize;
        assert!(total_subpacket_len <= bits.len());

        let leftover_len = bits.len() - total_subpacket_len;
        while bits.len() > leftover_len {
            inner_packets.push(eval_packet(bits));
        }
        assert_eq!(bits.len(), leftover_len);
    }

    inner_packets
}

fn combine_values(values: impl Iterator<Item=u64>, operator_type: u32) -> u64 {
    match operator_type {
        0 => values.sum(),
        1 => values.product(),
        2 => values.min().unwrap(),
        3 => values.max().unwrap(),
        4 => panic!("Not an operator type; 4 is reserved for literals."),
        5 | 6 | 7 => {
            let (x, y) = values.collect_tuple().unwrap();
            let b = match operator_type {
                5 => x > y,
                6 => x < y,
                7 => x == y,
                _ => unreachable!(),
            };
            b as u64
        }
        _ => panic!("Not an operator type: {}", operator_type),
    }
}
