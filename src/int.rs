//https://cdn.discordapp.com/attachments/622534733351485470/967048993492332544/unknown.png

use std::ops::{Add, Sub};

pub struct BigUint {
    limbs: Vec<u64>
}

impl BigUint {
    pub fn new() -> Self {
        Self{
            limbs: Vec::new()
        }
    }

    pub fn from_limbs(limbs: &[u64]) -> Self {
        Self{
            limbs: Vec::from(limbs)
        }
    }

    fn min_limbs(&mut self) {
        for i in (0..self.limbs.len()).rev() {
            if self.limbs[i] == 0 {
                assert_eq!(self.limbs.pop(), Some(0));
            } else {
                break;
            }
        }
    }

    fn clone_capacity(&self, capacity: usize) -> Self {
        assert!(self.limbs.len() <= capacity);
        let mut new_limbs = self.limbs.clone();
        for _i in self.limbs.len()..capacity {
            new_limbs.push(0);
        }
        Self{
            limbs: new_limbs
        }
    }
}

impl From<u64> for BigUint {
    fn from(val: u64) -> Self {
        Self{
            limbs: vec![val]
        }
    }
}

impl Add for BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add for &BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone_capacity(std::cmp::max(self.limbs.len(), rhs.limbs.len()) + 1);

        let mut carry = 0u64;
        for i in 0..rhs.limbs.len() {
            let (sum, overflow) = result.limbs[i].overflowing_add(carry);
            carry = 0;
            if overflow {
                carry = 1;
            }
            let (sum, overflow) = sum.overflowing_add(rhs.limbs[i]);
            if overflow {
                assert!(carry == 0);
                carry = 1;
            }
            result.limbs[i] = sum;
        }

        let mut i = rhs.limbs.len();
        loop {
            let (sum, overflow) = result.limbs[i].overflowing_add(carry);
            result.limbs[i] = sum;
            if !overflow {
                break;
            }
            i += 1;
        }

        result.min_limbs();
        result
    }
}

impl Sub for BigUint {
    type Output = BigUint;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub for &BigUint {
    type Output = BigUint;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.clone_capacity(std::cmp::max(self.limbs.len(), rhs.limbs.len()));

        let mut borrow = 0u64;
        for i in 0..rhs.limbs.len() {
            let (diff, overflow) = result.limbs[i].overflowing_sub(borrow);
            borrow = 0;
            if overflow {
                borrow = 1;
            }
            let (diff, overflow) = diff.overflowing_sub(rhs.limbs[i]);
            if overflow {
                assert!(borrow == 0);
                borrow = 1;
            }
            result.limbs[i] = diff;
        }

        let mut i = rhs.limbs.len();
        while borrow != 0 {
            if i >= result.limbs.len() {
                panic!("attempt to subtract with overflow");
            }
            let (diff, overflow) = result.limbs[i].overflowing_sub(borrow);
            result.limbs[i] = diff;
            if !overflow {
                break;
            }
            i += 1;
        }

        result.min_limbs();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::BigUint;

    #[test]
    fn init() {
        let int = BigUint::new();
        assert_eq!(int.limbs, vec![]);

        let int = BigUint::from(0x12345678);
        assert_eq!(int.limbs, vec![0x12345678]);

        let int = BigUint::from_limbs(&[0x1234, 5678]);
        assert_eq!(int.limbs, vec![0x1234, 5678]);
    }

    #[test]
    fn add_and_sub() {
        let a = BigUint::from_limbs(&[0xfedcba9876543210, 0xffffffffffffffff]);
        let b = BigUint::from_limbs(&[0x2234567898765432]);
        let sum = &a + &b;
        assert_eq!(sum.limbs, &[0x211111110ECA8642, 0x0000000000000000, 1]);
        assert_eq!((&sum - &b).limbs, a.limbs);
        assert_eq!((&sum - &a).limbs, b.limbs);
        
        let a = BigUint::from_limbs(&[0x2234567898765432]);
        let b = BigUint::from_limbs(&[0xfedcba9876543210, 0xffffffffffffffff]);
        let sum = &a + &b;
        assert_eq!(sum.limbs, &[0x211111110ECA8642, 0x0000000000000000, 1]);
        assert_eq!((&sum - &b).limbs, a.limbs);
        assert_eq!((&sum - &a).limbs, b.limbs);
    }
}
