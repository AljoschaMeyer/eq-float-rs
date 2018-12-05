# Eq-Float

Wrappers around rust floats that implement Eq by letting `NAN == NAN` be true. This lets you pretend there was a total order on floating point numbers. Use with care, this goes against the IEEE 754 standard.

Also implements Ord and Hash. The hash digest for positive and negative zero is identical, since `0.0 == -0.0`. The hash digest for all NANs is also identical.

```rust
extern crate eq_float;

use eq_float::F32;

fn main() {
    assert!(!(std::f32::NAN == std::f32::NAN));
    assert!(F32(std::f32::NAN) == F32(std::f32::NAN));
    assert!(F32(std::f32::NAN) < F32(5.0));
}
```
