pub type Digit = u32;

/// iterator over all LEN-digit BASE-base numbers with num_zeros zeros as digits
pub fn digit_numbers<const LEN: usize, const BASE: usize>(
    num_zeros: usize,
) -> impl Iterator<Item = [Digit; LEN]> {
    if LEN == 0 || LEN > 31 || BASE < 2 {
        panic!()
    } else {
        // if num_zeros is LEN, we use LEN ones instead of 0 and flip,
        // because gospers hack works only with k > 0
        let num_ones = if LEN == num_zeros {
            LEN
        } else {
            LEN - num_zeros
        } as i32;

        fixed_pop_bitvalues(LEN as i32, num_ones, LEN == num_zeros).flat_map(|zero_pattern| {
            // the zero_pattern is an indicator for the positions that must be 0
            let mut next_val = [1; LEN];
            for i in 0..LEN {
                if bit_at(zero_pattern, i) == 0 {
                    next_val[LEN - i - 1] = 0;
                }
            }

            ZeroPatternNumbers::<LEN, BASE> {
                next_val,
                curr_digit: 0,
            }
        })
    }
}

pub struct ZeroPatternNumbers<const LEN: usize, const BASE: usize> {
    next_val: [Digit; LEN],
    curr_digit: usize,
}

impl<const LEN: usize, const BASE: usize> Iterator for ZeroPatternNumbers<LEN, BASE> {
    type Item = [Digit; LEN];

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.curr_digit == LEN {
            return None;
        }

        let curr = self.next_val;

        while curr[LEN - self.curr_digit - 1] == (BASE - 1) as Digit
            || curr[LEN - self.curr_digit - 1] == 0
        {
            // reset this digit to 1 because the zeros are fixed
            if curr[LEN - self.curr_digit - 1] == (BASE - 1) as Digit {
                self.next_val[LEN - self.curr_digit - 1] = 1;
            }

            self.curr_digit += 1;

            if self.curr_digit == LEN {
                return Some(curr);
            }
        }

        self.next_val[LEN - self.curr_digit - 1] += 1;
        self.curr_digit = 0;

        Some(curr)
    }
}

/// iterator over all n-digit bitvalues with k ones
/// if flip == true, instead k zeros are given
/// using gospers hack
fn fixed_pop_bitvalues(n: i32, k: i32, flip: bool) -> FixedPopBitvalues {
    if n > 31 || k > n || k == 0 {
        panic!()
    } else {
        FixedPopBitvalues {
            limit: 1 << n,
            next_val: (1 << k) - 1,
            flip,
        }
    }
}

struct FixedPopBitvalues {
    limit: i32,
    next_val: i32,
    flip: bool,
}

impl Iterator for FixedPopBitvalues {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.next_val >= self.limit {
            return None;
        }

        let curr = self.next_val;

        // https://programmingforinsomniacs.blogspot.com/2018/03/gospers-hack-explained.html
        let rightmost1 = curr & (-curr);
        let moved_rightmost1 = curr + rightmost1;
        self.next_val = (((moved_rightmost1 ^ curr) >> 2) / rightmost1) | moved_rightmost1;

        if self.flip {
            Some(!curr)
        } else {
            Some(curr)
        }
    }
}

/// 0-based bit_index starting from the least significant bit (right regarding shift direction)
fn bit_at(val: i32, bit_index: usize) -> u32 {
    ((val >> bit_index) & 1i32) as u32
}

/// convert a number of length LEN in base BASE from given digits to usize value
pub fn to_value<const LEN: usize, const BASE: usize>(digits: &[Digit; LEN]) -> usize {
    (0..LEN)
        .map(|i| digits[i] as usize * BASE.pow((LEN - i - 1) as u32))
        .sum()
}

// some sanity checks
#[cfg(test)]
mod tests {
    use super::*;

    fn factorial(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    fn binomial_coefficient(n: usize, k: usize) -> usize {
        if k > n {
            0
        } else {
            factorial(n) / (factorial(k) * factorial(n - k))
        }
    }

    // checks whether the number of elements in the digit_numbers iterator
    // has the value it theoretically should have
    #[test]
    fn test_digit_numbers_cardinality() {
        const LEN: usize = 5;
        const BASE: usize = 7;

        for num_zeros in 0..=LEN {
            let cardinality = digit_numbers::<LEN, BASE>(num_zeros).count();
            let expected = binomial_coefficient(LEN, num_zeros)
                * ((BASE - 1) as usize).pow((LEN - num_zeros) as u32) as usize;
            assert_eq!(cardinality, expected)
        }
    }

    // check whether all cells of the table are hit exactly once,
    // which should theoretically be the case
    #[test]
    fn test_table_coverage() {
        const LEN: usize = 5;
        const BASE: usize = 7;
        const TABLE_SIZE: usize = (BASE as usize).pow(LEN as u32);

        let mut table: Vec<u8> = vec![0; TABLE_SIZE];

        for num_zeros in 0..=LEN {
            for digits in digit_numbers::<LEN, BASE>(num_zeros) {
                table[to_value::<LEN, BASE>(&digits)] += 1;
            }
        }

        for val in table {
            assert_eq!(val, 1)
        }
    }
}
