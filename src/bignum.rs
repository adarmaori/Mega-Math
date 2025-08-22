use anyhow;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigNum(pub Vec<u64>);

impl BigNum {
    pub fn new(value: u64) -> Self {
        Self(vec![value])
    }

    pub fn resize(&mut self, size: usize) {
        self.0.resize(size, 0);
    }

    pub fn truncate(&mut self) {
        // Remove leading zero blocks
        let mut index = self.0.len();
        while index > 1 && self.0[index - 1] == 0 {
            index -= 1;
        }
        self.0.truncate(index);
    }

    pub fn add(&mut self, other: &Self) {
        let self_len = self.0.len();
        let other_len = other.0.len();
        let total_len = self_len.max(other_len);
        self.resize(total_len + 1);

        let mut index: usize = 0;
        let mut carry: u128 = 0;
        while index < total_len {
            let a = (self.0.get(index).copied().unwrap_or(0)) as u128;
            let b = (other.0.get(index).copied().unwrap_or(0)) as u128;
            let s = a + b + carry;
            let res: u64 = (s & 0xFFFFFFFFFFFFFFFF) as u64;
            carry = s >> 64;
            self.0[index] = res;
            index += 1;
        }
        if carry > 0 {
            self.0[index] = carry as u64;
        } else {
            self.0.truncate(index);
        }
    }

    pub fn sub(&mut self, other: &Self) -> anyhow::Result<()> {
        match (*self).cmp(other) {
            Ordering::Less => anyhow::bail!("Doesn't support negative numbers yet"),
            Ordering::Equal => {
                self.0 = vec![0];
                Ok(())
            }
            Ordering::Greater => {
                let mut index = 0;
                let mut borrow = 0;
                let len = self.0.len();
                while index < len {
                    let a = *self.0.get(index).unwrap_or(&0) as i128;
                    let b = *other.0.get(index).unwrap_or(&0) as i128;
                    let s = a - b - borrow;
                    if s < 0 {
                        borrow = 1;
                        self.0[index] = (s + (1i128 << 64)) as u64;
                    } else {
                        borrow = 0;
                        self.0[index] = s as u64;
                    }
                    index += 1;
                }
                self.truncate();
                if borrow != 0 {
                    anyhow::bail!("There shouldn't be borrows left after a - b when a > b");
                }
                Ok(())
            }
        }
    }

    pub fn split(&self) -> (Self, Self) {
        // TODO: tests
        let n = self.0.len();
        (
            BigNum(self.0[0..n.div_ceil(2)].to_vec()),
            BigNum(self.0[n.div_ceil(2)..].to_vec()),
        )
    }

    pub fn shift_left(&mut self, shamnt: u128) {
        if shamnt == 0 {
            return;
        }

        let word_shifts = (shamnt / 64) as usize;
        let bit_shifts = shamnt % 64;

        if word_shifts > 0 {
            // Prepend zeros efficiently
            let mut new_vec = vec![0u64; word_shifts];
            new_vec.extend_from_slice(&self.0);
            self.0 = new_vec;
        }

        // Handle remaining bit shifts within words
        if bit_shifts > 0 {
            let mut carry = 0u64;
            for i in 0..self.0.len() {
                let new_carry = self.0[i] >> (64 - bit_shifts);
                self.0[i] = (self.0[i] << bit_shifts) | carry;
                carry = new_carry;
            }
            if carry > 0 {
                self.0.push(carry);
            }
        }
    }

    pub fn shift_right(&mut self, shamnt: u128) {
        // TODO: tests
        todo!();
    }

    pub fn square(&mut self) {
        let mut n = self.0.len() as u128;
        // For small numbers, use simple squaring
        if n == 1 {
            // base case
            let res = (self.0[0] as u128).pow(2);
            let low = (res & 0xFFFFFFFFFFFFFFFF) as u64;
            let high = (res >> 64) as u64;
            if high > 0 {
                self.0 = vec![low, high];
            } else {
                self.0 = vec![low];
            }
        } else {
            n += n % 2; // Force n to be even. TODO: This isn't in the algorithm spec, and I'm not
            // sure it's necessary
            // split A into two equal parts AL and AR
            let (ar, al) = self.split();
            // Compute the intermediate values:
            // d1 = square(AL)
            let mut d1 = al.clone();
            d1.square();
            // d0 = square(AR)
            let mut d0 = ar.clone();
            d0.square();
            // d0,1 = square(AL+AR)
            let mut d01 = al.clone();
            d01.add(&ar);
            d01.square();
            // return C = d1 * r^n + (d0,1 - d0 - d1) * r^(n/2) + d0
            let _ = d01.sub(&d0);
            let _ = d01.sub(&d1);
            d01.shift_left(64 * n.div_ceil(2));
            let mut c = d1.clone();
            c.shift_left(64 * n);
            c.add(&d01);
            c.add(&d0);
            self.0 = c.0;
        }
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        // This function assumes self and other are truncated with no leading zero blocks
        let s_len = self.0.len();
        let o_len = other.0.len();
        match s_len.cmp(&o_len) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                // Recursive comparison
                let mut index = s_len - 1;
                while index > 0 {
                    let a = self.0.get(index);
                    let b = other.0.get(index);
                    if a != b {
                        return a.cmp(&b);
                    }
                    index -= 1;
                }
                self.0[0].cmp(&other.0[0])
            }
        }
    }
}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
