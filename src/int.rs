//https://cdn.discordapp.com/attachments/622534733351485470/967048993492332544/unknown.png

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
}

impl From<u64> for BigUint {
    fn from(val: u64) -> Self {
        Self{
            limbs: vec![val]
        }
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
}
