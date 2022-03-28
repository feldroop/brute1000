/// iterator over all LEN-digit BASE-base numbers with num_zeros > 0 zeros as digits
pub fn digit_numbers<const LEN: usize, const BASE: u8>(
    num_zeros: usize,
) -> impl Iterator<Item = [u8; LEN]> {
    if LEN == 0 || LEN > 31 || BASE < 2 || num_zeros == 0 {
        panic!()
    } else {
        return fixed_pop_bitvalues(LEN as i32, (LEN - num_zeros) as i32)
            .map(|zero_pattern| {
                // the zero_pattern is an indicator for the positions that must be 0
                let mut next_val = [1; LEN];
                for i in 0..LEN {
                    if bit_at(zero_pattern, i as u8) == 0 {
                        next_val[LEN - i - 1] = 0;
                    }
                }

                ZeroPatternNumbers::<LEN, BASE> {
                    next_val,
                    curr_digit: 0,
                }
            })
            .flatten();
    }
}

/// iterator over all LEN-digit BASE-base numbers with no zeros as digits
pub fn digit_numbers_no_zero<const LEN: usize, const BASE: u8>() -> impl Iterator<Item = [u8; LEN]>
{
    if LEN == 0 || LEN > 31 || BASE < 2 {
        panic!()
    } else {
        return ZeroPatternNumbers::<LEN, BASE> {
            next_val: [1; LEN],
            curr_digit: 0,
        };
    }
}

pub struct ZeroPatternNumbers<const LEN: usize, const BASE: u8> {
    next_val: [u8; LEN],
    curr_digit: usize,
}

impl<const LEN: usize, const BASE: u8> Iterator for ZeroPatternNumbers<LEN, BASE> {
    type Item = [u8; LEN];

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.curr_digit == LEN {
            return None;
        }

        let curr = self.next_val;

        while curr[LEN - self.curr_digit - 1] == BASE - 1 || curr[LEN - self.curr_digit - 1] == 0 {
            // reset this digit to 1 because the zeros are fixed
            if curr[LEN - self.curr_digit - 1] == BASE - 1 {
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
fn fixed_pop_bitvalues(n: i32, k: i32) -> FixedPopBitvalues {
    if n > 31 || k > n || k == 0 {
        panic!()
    } else {
        FixedPopBitvalues {
            limit: 1 << n,
            next_val: (1 << k) - 1,
        }
    }
}

struct FixedPopBitvalues {
    limit: i32,
    next_val: i32,
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

        Some(curr)
    }
}

/// 0-based bit_index starting from the least significant bit (right regarding shift direction)
fn bit_at(val: i32, bit_index: u8) -> u8 {
    ((val >> bit_index) & 1i32) as u8
}

pub fn to_value<const LEN: usize, const BASE: u32>(digits: [u8; LEN]) -> usize {
    (0..LEN)
        .map(|i| digits[i] as u32 * BASE.pow((LEN - i - 1) as u32))
        .sum::<u32>() as usize
}
