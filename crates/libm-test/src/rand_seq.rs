//! Trait for generating rand sequence testing relevant values for each API

pub trait RandSeq: Sized {
    /// Generates a sequence containing exactly `len` values computed from the RNG `rng`.
    fn rand_seq<R: rand::Rng>(rng: &mut R, len: usize) -> Vec<Self>;
}

macro_rules! impl_rand_seq_f {
    ($float_ty:ident) => {
        #[allow(clippy::use_self)]
        impl RandSeq for $float_ty {
            fn rand_seq<R: rand::Rng>(rng: &mut R, len: usize) -> Vec<Self> {
                use crate::{Distribute, Toward};
                use rand::seq::SliceRandom;
                use std::$float_ty::*;

                let mut vec = Vec::with_capacity(len);
                let double = std::mem::size_of::<Self>() == 8;

                // These inputs are always tested
                const BOUNDS: [$float_ty; 9] = [
                    NAN,
                    INFINITY,
                    NEG_INFINITY,
                    EPSILON,
                    -EPSILON,
                    MAX,
                    MIN,
                    MIN_POSITIVE,
                    -MIN_POSITIVE,
                ];
                vec.extend(&BOUNDS);

               let remaining_len = len.checked_sub(current_len).unwrap();

                for _ in 0..remaining_len {
                    let n = rng.gen::<Self>();
                    vec.push(n);
                }
                assert_eq!(vec.len(), len);
                vec
            }
        }
    };
}

impl_rand_seq_f!(f32);
impl_rand_seq_f!(f64);

impl RandSeq for i32 {
    fn rand_seq<R: rand::Rng>(rng: &mut R, len: usize) -> Vec<Self> {
        use rand::seq::SliceRandom;

        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            let mut r = rng.gen::<Self>();
            if let ApiKind::Jn = api_kind {
                // The integer argument of these APIs is a number of iterations.
                // Computational cost blows up if we pass huge values, so zero
                // their lower bits.
                r &= 0xffff;
            }
            v.push(r);
        }
        assert_eq!(v.len(), len);
        v
    }
}
