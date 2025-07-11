use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField};
use ark_poly::multivariate::{SparsePolynomial, SparseTerm};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

pub type MaskPolynomial<E: Pairing> = SparsePolynomial<E::ScalarField, SparseTerm>;
pub trait Math {
    fn square_root(self) -> usize;
    fn exp2(self) -> usize;
    fn get_bits(self, num_bits: usize) -> Vec<bool>;
    fn log_2(self) -> usize;
}

impl Math for usize {
    #[inline]
    fn square_root(self) -> usize {
        (self as f64).sqrt() as usize
    }

    #[inline]
    fn exp2(self) -> usize {
        let base: usize = 2;
        base.pow(self as u32)
    }

    /// Returns the num_bits from n in a canonical order
    fn get_bits(self, num_bits: usize) -> Vec<bool> {
        (0..num_bits)
            .map(|shift_amount| ((self & (1 << (num_bits - shift_amount - 1))) > 0))
            .collect::<Vec<bool>>()
    }

    fn log_2(self) -> usize {
        if self.is_power_of_two() {
            (1usize.leading_zeros() - self.leading_zeros()) as usize
        } else {
            (0usize.leading_zeros() - self.leading_zeros()) as usize
        }
    }
}

#[derive(Debug, CanonicalSerialize, CanonicalDeserialize, Clone)]
pub struct SparseMatEntry<F: Field> {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) val: F,
}

impl<F: PrimeField> SparseMatEntry<F> {
    pub fn new(row: usize, col: usize, val: F) -> Self {
        SparseMatEntry { row, col, val }
    }
}
