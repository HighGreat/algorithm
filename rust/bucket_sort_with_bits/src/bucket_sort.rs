const BITS_PER_WORD: usize = 32; // i32 is 32 bits
const SHIFT: i32 = 5; // 32 is 5 bits
const MASK: i32 = 0x1f;
const MAX: usize = 100_000_000;
const ARRAY_SIZE: usize = 1 + MAX / BITS_PER_WORD;

pub fn sort(numbers: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut bits = vec![0; ARRAY_SIZE];

    for num in numbers.iter() {
        bit_set(&mut bits, *num);
    }

    let mut index = 0;

    for n in 0..=MAX {
        if bit_test(&bits, n as i32) {
            numbers[index] = n as i32;

            index += 1;
        }
    }

    numbers
}

fn bit_set(bits: &mut Vec<i32>, number: i32) {
    let index = number >> SHIFT;
    bits[index as usize] |= 1 << (number & MASK);
}

fn bit_test(bits: &[i32], number: i32) -> bool {
    let index = number >> SHIFT;
    (bits[index as usize] & (1 << (number & MASK))) != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        assert_eq!(
            sort(&mut vec![3, 1, 2, 0, 100_000_000, 99_999_999]),
            &mut vec![0, 1, 2, 3, 99_999_999, 100_000_000]
        );
    }
}
