use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone, Copy)]
pub struct F32(pub f32);

/// This works like `PartialEq` on `f32`, except that `NAN == NAN` is true.
impl PartialEq for F32 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for F32 {}

/// This works like `PartialOrd` on `f32`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN). This always returns a `Some`.
impl PartialOrd for F32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This works like `PartialOrd` on `f32`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN).
impl Ord for F32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            if self.0.is_nan() && !other.0.is_nan() {
                Ordering::Less
            } else if !self.0.is_nan() && other.0.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
}

impl Hash for F32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            0x7fc00000u32.hash(state); // a particular bit representation for NAN
        } else if self.0 == 0.0 { // catches both positive and negative zero
            0u32.hash(state);
        } else {
            self.0.to_bits().hash(state);
        }
    }
}

impl From<F32> for f32 {
    fn from(f: F32) -> Self {
        f.0
    }
}

impl From<f32> for F32 {
    fn from(f: f32) -> Self {
        F32(f)
    }
}

impl fmt::Display for F32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct F64(pub f64);

/// This works like `PartialEq` on `f64`, except that `NAN == NAN` is true.
impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl Eq for F64 {}

/// This works like `PartialOrd` on `f64`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN). This always returns a `Some`.
impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// This works like `PartialOrd` on `f64`, except that `NAN` sorts below all other floats
/// (and is equal to another NAN).
impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or_else(|| {
            if self.0.is_nan() && !other.0.is_nan() {
                Ordering::Less
            } else if !self.0.is_nan() && other.0.is_nan() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        })
    }
}

impl Hash for F64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            0x7ff8000000000000u64.hash(state); // a particular bit representation for NAN
        } else if self.0 == 0.0 { // catches both positive and negative zero
            0u64.hash(state);
        } else {
            self.0.to_bits().hash(state);
        }
    }
}

impl From<F64> for f64 {
    fn from(f: F64) -> Self {
        f.0
    }
}

impl From<f64> for F64 {
    fn from(f: f64) -> Self {
        F64(f)
    }
}

impl fmt::Display for F64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::{F32, F64};

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn f32_eq() {
        assert!(F32(std::f32::NAN) == F32(std::f32::NAN));
        assert!(F32(std::f32::NAN) != F32(5.0));
        assert!(F32(5.0) != F32(std::f32::NAN));
        assert!(F32(0.0) == F32(-0.0));
    }

    #[test]
    fn f32_cmp() {
        assert!(F32(std::f32::NAN) == F32(std::f32::NAN));
        assert!(F32(std::f32::NAN) < F32(5.0));
        assert!(F32(5.0) > F32(std::f32::NAN));
        assert!(F32(0.0) == F32(-0.0));
    }

    #[test]
    fn f32_hash() {
        assert!(calculate_hash(&F32(0.0)) == calculate_hash(&F32(-0.0)));
        assert!(calculate_hash(&F32(std::f32::NAN)) == calculate_hash(&F32(-std::f32::NAN)));
    }

    #[test]
    fn f64_eq() {
        assert!(F64(std::f64::NAN) == F64(std::f64::NAN));
        assert!(F64(std::f64::NAN) != F64(5.0));
        assert!(F64(5.0) != F64(std::f64::NAN));
        assert!(F64(0.0) == F64(-0.0));
    }

    #[test]
    fn f64_cmp() {
        assert!(F64(std::f64::NAN) == F64(std::f64::NAN));
        assert!(F64(std::f64::NAN) < F64(5.0));
        assert!(F64(5.0) > F64(std::f64::NAN));
        assert!(F64(0.0) == F64(-0.0));
    }

    #[test]
    fn f64_hash() {
        assert!(calculate_hash(&F64(0.0)) == calculate_hash(&F64(-0.0)));
        assert!(calculate_hash(&F64(std::f64::NAN)) == calculate_hash(&F64(-std::f64::NAN)));
    }
}
