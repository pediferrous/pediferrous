//! Implementation of PDF Color Space and Color specification.

use std::fmt;

/// Possible errors that might be returned when creating a new [`CmykValue`] instance.
#[derive(Debug, thiserror::Error)]
pub enum CmykValueErr<T: fmt::Display> {
    /// Indicates that the provided does not fit into [`CmykValue`].
    #[error("Provided value '{}' is out of range", .0)]
    OutOfRange(T),
}

/// Newtype for ensuring correct values are used in CMYK color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmykValue(u8);

impl CmykValue {
    /// Create a new [`CmykValue`] from a constant that is in range. Range is checked at compile
    /// time.
    pub const fn from_const<const N: u8>() -> Self {
        if N > 100 {
            panic!("Only values in range [0, 100] are allowed.");
        }

        Self(N)
    }
}

impl TryFrom<u8> for CmykValue {
    type Error = CmykValueErr<u8>;

    /// Create a new [`CmykValue`] from an [`u8`], with valid range being `[0, 100]`.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(CmykValueErr::OutOfRange(value));
        }

        Ok(Self(value))
    }
}

impl TryFrom<f32> for CmykValue {
    type Error = CmykValueErr<f32>;

    /// Create a new [`CmykValue`] from an [`f32`], with valid range being `[0.0, 1.0]`.
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

#[cfg(test)]
mod tests {
    use super::CmykValue;

    #[test]
    fn out_of_range_u8() {
        let res = CmykValue::try_from(101);
        assert!(res.is_err());
    }

    #[test]
    fn out_of_range_f32() {
        let res = CmykValue::try_from(1.01);
        assert!(res.is_err());
    }

    #[test]
    fn in_range() {
        let res = CmykValue::try_from(0.1);
        assert!(res.is_ok());
        let res = CmykValue::try_from(0.0);
        assert!(res.is_ok());
        let res = CmykValue::try_from(0.99);
        assert!(res.is_ok());

        let res = CmykValue::try_from(0);
        assert!(res.is_ok());
        let res = CmykValue::try_from(10);
        assert!(res.is_ok());
        let res = CmykValue::try_from(99);
        assert!(res.is_ok());
    }
}
