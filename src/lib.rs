#[derive(Debug, Clone)]
struct BigNum(Vec<u64>);

impl BigNum {
    fn new(value: u64) -> Self {
        Self(vec![value])
    }

    fn resize(&mut self, size: usize) {
        self.0.resize(size, 0);
    }
    fn add(&mut self, other: &Self) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_small_number() {
        let n = BigNum::new(13);
        assert_eq!(n.0, vec![13u64]);
    }

    #[test]
    fn resize_number() {
        let mut n = BigNum::new(13);
        n.resize(3);
        assert_eq!(n.0, vec![13u64, 0u64, 0u64]);
    }

    #[test]
    fn add_numbers() {
        let mut n = BigNum::new(13);
        let m = BigNum::new(13);
        n.add(&m);
        assert_eq!(n.0, vec![26u64]);
    }
}
