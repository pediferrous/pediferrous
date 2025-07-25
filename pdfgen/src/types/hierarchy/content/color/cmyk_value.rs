use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum CmykValueErr<T: fmt::Display> {
    #[error("Provided value '{}' is out of range", .0)]
    OutOfRange(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmykValue(u8);

impl CmykValue {
    pub const fn from_const<const N: u8>() -> Self {
        if N > 100 {
            panic!("Only values in range [0, 100] are allowed.");
        }

        Self(N)
    }
}

impl TryFrom<u8> for CmykValue {
    type Error = CmykValueErr<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(CmykValueErr::OutOfRange(value));
        }

        Ok(Self(value))
    }
}

impl TryFrom<f32> for CmykValue {
    type Error = CmykValueErr<f32>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0. ..=1.).contains(&value) {
            return Err(CmykValueErr::OutOfRange(value));
        }

        let value = value * 100.;
        Ok(Self(value as u8))
    }
}

impl From<CmykValue> for u8 {
    fn from(value: CmykValue) -> Self {
        value.0
    }
}
